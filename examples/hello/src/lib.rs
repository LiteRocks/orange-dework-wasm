use oscore::runtime;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

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
pub struct DefiInfo {
    chain:String,
    defi_name:String,
    net_balance:String,
    xday_sum:XdaySum
}


#[derive(Serialize, Deserialize)]
pub struct DefiAssets {
    asset_infos: Vec<TokenInfo>,
    defi_infos:Vec<DefiInfo>
}

#[derive(Serialize, Deserialize)]
pub struct AssetInfoData {
//    asset_infos: Vec<TokenInfo>,
    user_did :String,
    defi_assets:DefiAssets,
    sig:String
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

//    let mut worthy = 0.0;
    let token_assets = p.defi_assets.asset_infos;
    let defi_assets = p.defi_assets.defi_infos;

    let mut token_total_value = 0.0;
    token_assets.iter().for_each(|asset|{
        let price = asset.price.parse::<f64>().unwrap();
        let balance = asset.balance.parse::<f64>().unwrap();
        let amount =  asset.xday_sum.amount.parse::<f64>().unwrap();
        let days = asset.xday_sum.days;

        token_total_value += (balance * 0.3 + amount  / days as f64 * 0.7) * price
    });

    let mut defi_total_value = 0.0;
    defi_assets.iter().for_each(|asset|{
        let balance = asset.net_balance.parse::<f64>().unwrap();
        let amount =  asset.xday_sum.amount.parse::<f64>().unwrap();
        let days = asset.xday_sum.days;

        defi_total_value += balance * 0.3 + amount / days as f64 * 0.7
    });

    let token_score = calc_token_score(token_total_value);
    let defi_score =calc_defi_score(defi_total_value);
    let basic_score = 400;


    let score = ScoreResult { score: basic_score + token_score + defi_score};

    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}

fn calc_token_score(worthy:f64)->u32 {

    let w = worthy as u32;
    let score =
        if w < 2000{
            0
        }else if  w >= 2000 && w < 5000 {
            (w - 2000)/30
        }else if  w >= 5000 && w < 10000{
            (w-5000)/50 + 50
        }else if  w >= 10000 && w < 20000{
            (w-10000)/100+100
        } else if  w >= 20000 && w < 50000{
            (w-20000)/300+150
        }else if  w >= 50000 && w < 100000{
            (w-50000)/500+200
        }else{
            (w-50000)/500+200 //??
        };
    score
}

fn calc_defi_score(worthy:f64)->u32 {
    let w = worthy as u32;
    let score =
        if w < 1000{
            0
        }else if w >= 1000 && w <10000{
            w / 100
        }else if w >= 10000 && w <100000{
            w/1000 + 50
        }else if w >= 100000 && w < 1000000{
            w/10000 + 100
        }else {
            w/100000+150
        };
    score
}