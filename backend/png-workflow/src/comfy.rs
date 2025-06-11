use crate::{GenerationInfo, InfoExtractorError};
use png::text_metadata::TEXtChunk;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static COMPRESS_PROMPT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\s+"#).unwrap());

impl GenerationInfo {
    pub(crate) fn from_text_chunks_comfy(
        text_chunks: &[TEXtChunk],
    ) -> Result<GenerationInfo, InfoExtractorError> {
        let workflow_json = text_chunks
            .iter()
            .find(|chunk| chunk.keyword == "prompt")
            .map(|chunk| &chunk.text)
            .ok_or(InfoExtractorError::WorkflowNotFound)?;

        Self::from_json_str_comfy(workflow_json)
    }

    fn from_json_str_comfy(json_str: &str) -> Result<GenerationInfo, InfoExtractorError> {
        let workflow: HashMap<String, Value> = serde_json::from_str(json_str)?;

        // find 'KSampler' or 'KSampler' like node
        // e.g. 'KSampler', 'ImpactKSamplerBasicPipe'
        let Some(ksampler_node) = workflow.values().find(|node| {
            node.get("class_type")
                .and_then(Value::as_str)
                .is_some_and(|c| c.contains("KSampler"))
        }) else {
            return Ok(GenerationInfo::default());
        };

        let Some(ksampler_inputs) = ksampler_node.get("inputs").and_then(Value::as_object) else {
            return Ok(GenerationInfo::default());
        };

        let unpiped = if ksampler_node
            .get("class_type")
            .and_then(Value::as_str)
            .is_some_and(|c| c.contains("BasicPipe"))
        {
            Self::follow_inputs(&workflow, ksampler_node, None, "basic_pipe")
                .first()
                .cloned()
        } else {
            Some(ksampler_node)
        };

        let model_names = {
            let mut names = Vec::new();
            names.extend(Self::fetch_models(&workflow, &unpiped, "ckpt_name"));
            names.extend(Self::fetch_models(&workflow, &unpiped, "lora_name"));
            names
        };

        let info = GenerationInfo {
            step_count: ksampler_inputs.get("steps").and_then(Value::as_i64),
            cfg_scale: ksampler_inputs.get("cfg").and_then(Value::as_f64),
            seed: ksampler_inputs.get("seed").and_then(Value::as_i64),
            sampler_name: ksampler_inputs
                .get("sampler_name")
                .and_then(Value::as_str)
                .map(String::from),
            scheduler_name: ksampler_inputs
                .get("scheduler")
                .and_then(Value::as_str)
                .map(String::from),
            model_names,
            positive_prompt: unpiped
                .map(|n| {
                    Self::get_linked_text_input(&workflow, n, Some("positive"), "text").join(", ")
                })
                .map(|s| {
                    COMPRESS_PROMPT
                        .replace_all(s.as_str(), " ")
                        .trim()
                        .to_string()
                }),
            negative_prompt: unpiped
                .map(|n| {
                    Self::get_linked_text_input(&workflow, n, Some("negative"), "text").join(", ")
                })
                .map(|s| {
                    COMPRESS_PROMPT
                        .replace_all(s.as_str(), " ")
                        .trim()
                        .to_string()
                }),
        };

        Ok(info)
    }

    fn fetch_models(
        workflow: &HashMap<String, Value>,
        unpiped: &Option<&Value>,
        target_input_key: &str,
    ) -> Vec<String> {
        unpiped
            .map(|n| {
                Self::get_linked_text_input(&workflow, n, None, target_input_key)
                    .into_iter()
                    .map(|name| {
                        name.rsplit_once(&['/', '\\'])
                            .map(|(_, name)| name.to_string())
                            .unwrap_or(name)
                            .trim()
                            .to_string()
                    })
                    .collect::<HashSet<_>>()
            })
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
    }

    fn follow_inputs<'a>(
        workflow: &'a HashMap<String, Value>,
        node: &'a Value,
        required_input_key: Option<&str>,
        final_inputs_key: &str,
    ) -> Vec<&'a Value> {
        Self::follow_inputs_impl(
            workflow,
            node,
            required_input_key,
            final_inputs_key,
            required_input_key.is_none(),
        )
    }

    fn follow_inputs_impl<'a>(
        workflow: &'a HashMap<String, Value>,
        node: &'a Value,
        required_input_key: Option<&str>,
        final_inputs_key: &str,
        has_required_key: bool,
    ) -> Vec<&'a Value> {
        let Some(inputs) = node.get("inputs").and_then(Value::as_object) else {
            return vec![];
        };

        if let Some(value) = inputs.get(final_inputs_key) {
            if !has_required_key {
                return vec![];
            }
            let Some(arr) = value.as_array() else {
                return vec![value];
            };
            let Some(key) = arr.get(0).and_then(Value::as_str) else {
                return vec![value];
            };
            return workflow.get(key).into_iter().collect();
        }

        inputs
            .iter()
            .filter_map(|(key, v)| {
                v.as_array().map(|arr| {
                    (
                        has_required_key || required_input_key.is_some_and(|k| k == key),
                        arr,
                    )
                })
            })
            .filter_map(|(h, arr)| arr.get(0).map(|v| (h, v)))
            .filter_map(|(h, v)| v.as_str().map(|s| (h, s)))
            .filter_map(|(h, link)| workflow.get(link).map(|v| (h, v)))
            .map(|(has_required_key, node)| {
                Self::follow_inputs_impl(
                    workflow,
                    node,
                    required_input_key,
                    final_inputs_key,
                    has_required_key,
                )
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    fn get_linked_text_input(
        workflow: &HashMap<String, Value>,
        node: &Value,
        required_input_key: Option<&str>,
        target_input_key: &str,
    ) -> Vec<String> {
        let values = Self::follow_inputs(workflow, node, required_input_key, target_input_key);
        values
            .into_iter()
            .filter_map(|v| v.as_str())
            .map(ToString::to_string)
            .collect()
    }
}
