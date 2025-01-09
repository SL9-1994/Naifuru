use crate::{
    analysis_config_file::{Config, ConversionConfig},
    error::{AnalysisErr, AppError},
};

use super::{ExtractedData, Extractor};

pub struct TwPalertSacExtractor {
    // 抽出前ファイル内容
    unextracted: ConversionConfig,
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

    fn extract_acc_values(&self) {
        todo!()
    }
}
