use crate::{GenerationInfo, InfoExtractorError};
use png::text_metadata::TEXtChunk;
use serde_json::Value;
use std::collections::HashMap;

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
            Self::follow_link(&workflow, ksampler_inputs, "basic_pipe")
                .and_then(|n| n.get("inputs").and_then(Value::as_object))
        } else {
            Some(ksampler_inputs)
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
            model_names: unpiped
                .and_then(|n| Self::get_linked_text_input(&workflow, n, "model", "ckpt_name"))
                .map(|name| {
                    name.rsplit_once(&['/', '\\'])
                        .map(|(_, name)| name.to_string())
                        .unwrap_or(name)
                })
                .into_iter()
                .collect(),
            positive_prompt: unpiped
                .and_then(|n| Self::get_linked_text_input(&workflow, n, "positive", "text")),
            negative_prompt: unpiped
                .and_then(|n| Self::get_linked_text_input(&workflow, n, "negative", "text")),
        };

        Ok(info)
    }

    fn follow_link<'a>(
        workflow: &'a HashMap<String, Value>,
        ksampler_inputs: &serde_json::Map<String, Value>,
        link_key: &str,
    ) -> Option<&'a Value> {
        let link_info = ksampler_inputs.get(link_key)?.as_array()?;
        let source_node_id = link_info.get(0)?.as_str()?;

        let source_node = workflow.get(source_node_id)?;

        Some(source_node)
    }

    fn get_linked_text_input(
        workflow: &HashMap<String, Value>,
        ksampler_inputs: &serde_json::Map<String, Value>,
        link_key: &str,
        target_input_key: &str,
    ) -> Option<String> {
        Self::follow_link(workflow, ksampler_inputs, link_key)?
            .get("inputs")?
            .get(target_input_key)?
            .as_str()
            .map(String::from)
    }
}
