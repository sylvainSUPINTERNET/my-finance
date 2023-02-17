use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct VantageMetadata {

    #[serde(rename = "1: Symbol")]
    symbol: String,

    #[serde(rename = "6: Series Type")]
    seriesType: String, // open high low close 

    #[serde(rename = "3: Last Refreshed")]
    lastRefresh: String, // "2023-02-15 16:00:01"

    #[serde(rename = "4: Interval")]
    daily: String, // "daily"

    #[serde(rename = "5: Time Period")]
    timePeriod: i32, // 10
}


#[derive(Serialize, Deserialize, Debug)]
pub struct VantageEMA {

    #[serde(rename = "Meta Data")]
    metaData: VantageMetadata,

    // dynamic key => use serde_json::from_str(data)? instead !
    // #[serde(rename = "Technical Analysis: EMA")]
    // technical_analysis_ema:TechnicalAnalysisEMA,
}