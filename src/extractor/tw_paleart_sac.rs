use crate::{analysis_config_file::ConversionConfig, error::AppError};

use super::{ExtractedData, Extractor};

pub struct TwPalertSacExtractor {
    pub unextracted: ConversionConfig,
}

impl Extractor for TwPalertSacExtractor {
    fn extract(&self) -> Result<ExtractedData, Vec<AppError>> {
        // Match文で、Toごとに抽出を切り替える
        todo!()
    }
}

impl TwPalertSacExtractor {
    pub fn new(unextracted: ConversionConfig) -> Self {
        Self { unextracted }
    }
}
