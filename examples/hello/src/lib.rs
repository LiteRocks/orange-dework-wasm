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
    let p: AssetInfoData = serde_json::from_slice(input.as_slice()).expect("invalid param");

//    let len = p.asset_infos.len();
    let mut worthy = 0.0;
    p.asset_infos.iter().for_each(|asset|{
        let price = asset.price.parse::<f64>().unwrap();
        let amount = asset.xday_sum.amount.parse::<f64>().unwrap();
        worthy = worthy + amount * price / asset.xday_sum.days as f64
    });



    let score = ScoreResult { score: calcScore(worthy)};

    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}

fn calcScore(worthy:f64)->u32 {
    let w = worthy as u32;
    if  w < 100 {
        100
    }else if w < 1000{
        300
    }else if w < 5000{
        500
    }else{
        800
    }
}