mod comfy;
mod sdwebui;

use png::text_metadata::TEXtChunk;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum InfoExtractorError {
    Io(std::io::Error),
    Png(png::DecodingError),
    Json(serde_json::Error),
    WorkflowNotFound,
    ParametersNotFound,
}

impl fmt::Display for InfoExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfoExtractorError::Io(e) => write!(f, "I/O Error: {}", e),
            InfoExtractorError::Png(e) => write!(f, "PNG Parsing Error: {}", e),
            InfoExtractorError::Json(e) => write!(f, "JSON Parsing Error: {}", e),
            InfoExtractorError::WorkflowNotFound => {
                write!(f, "Could not find ComfyUI workflow in the PNG tEXt chunk.")
            }
            InfoExtractorError::ParametersNotFound => {
                write!(
                    f,
                    "Could not find ComfyUI parameters in the PNG tEXt chunk."
                )
            }
        }
    }
}

impl Error for InfoExtractorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InfoExtractorError::Io(e) => Some(e),
            InfoExtractorError::Png(e) => Some(e),
            InfoExtractorError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for InfoExtractorError {
    fn from(err: std::io::Error) -> Self {
        InfoExtractorError::Io(err)
    }
}

impl From<png::DecodingError> for InfoExtractorError {
    fn from(err: png::DecodingError) -> Self {
        InfoExtractorError::Png(err)
    }
}

impl From<serde_json::Error> for InfoExtractorError {
    fn from(err: serde_json::Error) -> Self {
        InfoExtractorError::Json(err)
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationInfo {
    pub model_names: Vec<String>,
    pub positive_prompt: Option<String>,
    pub negative_prompt: Option<String>,
    pub step_count: Option<i64>,
    pub cfg_scale: Option<f64>,
    pub seed: Option<i64>,
    pub sampler_name: Option<String>,
    pub scheduler_name: Option<String>,
}

impl GenerationInfo {
    pub fn from_bytes(bytes: &[u8]) -> Result<GenerationInfo, InfoExtractorError> {
        let decoder = png::Decoder::new(bytes);
        Self::from_decoder(decoder)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<GenerationInfo, InfoExtractorError> {
        let file = File::open(path)?;
        let decoder = png::Decoder::new(file);
        Self::from_decoder(decoder)
    }

    fn from_decoder<R: Read>(
        decoder: png::Decoder<R>,
    ) -> Result<GenerationInfo, InfoExtractorError> {
        let reader = decoder.read_info()?;

        let text_chunks = reader.info().uncompressed_latin1_text.as_slice();
        Self::from_text_chunks(text_chunks)
    }

    fn from_text_chunks(text_chunks: &[TEXtChunk]) -> Result<GenerationInfo, InfoExtractorError> {
        match Self::from_text_chunks_comfy(text_chunks) {
            Ok(info) => Ok(info),
            Err(InfoExtractorError::WorkflowNotFound) => {
                match Self::from_text_chunks_sdwebui(text_chunks) {
                    Ok(info) => Ok(info),
                    Err(e) => Err(e),
                }
            }
            e => e,
        }
    }
}
