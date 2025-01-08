use crate::error::AnalysisErr;

pub mod jp_jma_csv;
pub mod jp_stera3d_txt;

pub trait Extractor {
    fn extract(&self) -> Result<ExtractedData, AnalysisErr>;
}

pub enum ExtractedData {
    JpStera3dTxt(JpStera3dTxtData),
    JpJmaCsv(JpJmaCsvData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JpStera3dTxtData {
    num_of_elements: i32,
    acc_values: Acceleration,
    common: CommonValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JpJmaCsvData {
    site_code: String,
    lat: f64,
    lon: f64,
    unit_type: String,
    initial_time: String,
    common: CommonValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommonValue {
    sampling_rate: f32,
    acc_values: Acceleration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Acceleration {
    ns: Vec<f64>,
    ew: Vec<f64>,
    ud: Vec<f64>,
}
