use tw_paleart_sac::TwPalertSacExtractor;

use crate::{
    analysis_config_file::{ConversionConfig, From},
    error::AppError,
};

pub mod jp_nied_knet;
pub mod nz_geonet_v1a_v2a;
pub mod tk_afad_asc;
pub mod tw_paleart_sac;
pub mod us_scsn_v2;

pub trait Extractor {
    fn extract(&self) -> Result<ExtractedData, Vec<AppError>>;
}

pub fn create_extractor(conversion: ConversionConfig) -> Box<dyn Extractor> {
    // fromに対応するextractorを呼び出す
    match &conversion.from {
        From::JpNiedKnet => todo!(),
        From::UsScsnV2 => todo!(),
        From::NzGeonetV1a => todo!(),
        From::NzGeonetV2a => todo!(),
        From::TwPalertSac => Box::new(TwPalertSacExtractor::new(conversion)),
        From::TkAfadAsc => todo!(),
    }
}

pub enum ExtractedData {
    JpStera3dTxt(JpStera3dTxtData),
    JpJmaCsv(JpJmaCsvData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct JpStera3dTxtData {
    num_of_elements: u32,
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
