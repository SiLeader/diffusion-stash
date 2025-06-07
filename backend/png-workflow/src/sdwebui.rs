use crate::{GenerationInfo, InfoExtractorError};
use png::text_metadata::TEXtChunk;

impl GenerationInfo {
    pub(crate) fn from_text_chunks_sdwebui(
        text_chunks: &[TEXtChunk],
    ) -> Result<GenerationInfo, InfoExtractorError> {
        let parameters_text = text_chunks
            .iter()
            .find(|chunk| chunk.keyword == "parameters")
            .map(|chunk| &chunk.text)
            .ok_or(InfoExtractorError::ParametersNotFound)?;

        Ok(Self::from_text_sdwebui(parameters_text))
    }

    fn from_text_sdwebui(text: &str) -> GenerationInfo {
        let mut info = GenerationInfo::default();
        let mut lines = text.lines();

        info.positive_prompt = lines.next().map(String::from);

        if let Some(line) = lines.next() {
            info.negative_prompt = line
                .strip_prefix("Negative prompt: ")
                .map(String::from)
                .or_else(|| Some(line.to_string()));
        }

        if let Some(line) = lines.next() {
            let parts = line.split(", ");
            for part in parts {
                if let Some((key, value)) = part.split_once(": ") {
                    match key.trim() {
                        "Steps" => info.step_count = value.trim().parse().ok(),
                        "Sampler" => info.sampler_name = Some(value.trim().to_string()),
                        "CFG scale" => info.cfg_scale = value.trim().parse().ok(),
                        "Seed" => info.seed = value.trim().parse().ok(),
                        "Model" => info.model_names = vec![value.trim().to_string()],
                        _ => {}
                    }
                }
            }
        }

        info
    }
}
