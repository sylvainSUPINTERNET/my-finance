use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct VantageMetadata {

    #[serde(rename = "1: Symbol")]
    pub symbol: String,

    #[serde(rename = "6: Series Type")]
    pub series_type: String, // open high low close 

    #[serde(rename = "3: Last Refreshed")]
    pub last_refresh: String, // "2023-02-15 16:00:01"

    #[serde(rename = "4: Interval")]
    pub daily: String, // "daily"
 
    #[serde(rename = "5: Time Period")]
    pub time_period: i32, // 10
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VantageEMA {

    #[serde(rename = "Meta Data")]
    pub meta_data: VantageMetadata,

    // dynamic key => use serde_json::from_str(data)? instead !
    // #[serde(rename = "Technical Analysis: EMA")]
    // technical_analysis_ema:TechnicalAnalysisEMA,
}