use oscore::runtime;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct XdaySum {
    amount: String,
    days: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    token_name: String,
    balance: String,
    xday_sum: XdaySum,
    price: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetInfoData {
    asset_infos: Vec<TokenInfo>,
    user_did :String,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreResult {
    score: u32,
}

#[no_mangle]
pub fn invoke() {
    oscore::set_panic_handler();
    let input = runtime::input();
    let p: AssetInfoData = serde_json::from_slice(input.as_slice()).expect("hhhh");

    let len = p.asset_infos.len();

    let score = ScoreResult { score: len as u32};

    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}
