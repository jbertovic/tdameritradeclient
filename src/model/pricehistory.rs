use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub candles: Vec<Candle>,
    pub symbol: String,
    pub empty: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub datetime: i64,
}

impl History {
    pub fn get_close_series(&self) -> Vec<f64> {
        self.candles.iter().map(|candle| candle.close).collect()
    }
}
