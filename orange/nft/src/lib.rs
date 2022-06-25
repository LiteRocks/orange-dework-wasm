use oscore::runtime;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct NFTInfo {
    latest_transfer_days_till_now:u32,
    transfer_count_for_last_year:u32,
    earliest_transfer_days_till_now:u32,
    owned_nft_kinds_count:u32,
    owned_nft_count:u32,
    current_nft_count:u32,
    current_nft_value_in_eth:String
}
#[derive(Serialize, Deserialize)]
pub struct Request {
    data :NFTInfo,
    sig:String,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreResult {
    score: u32,
}

#[no_mangle]
pub fn invoke() {
    oscore::set_panic_handler();
    let input = runtime::input();
    let r: Request = serde_json::from_slice(input.as_slice()).expect("invalid param");
    let p = r.data;

    let mut basic_score:u32 = 0;
    basic_score += calc_ealiest_transfer_days_till_now(p.earliest_transfer_days_till_now);
    basic_score += calc_transfer_count_for_last_year(p.transfer_count_for_last_year);
    basic_score += calc_latest_transfer_days_till_now(p.latest_transfer_days_till_now);
    basic_score += calc_owned_nft_kinds_count(p.owned_nft_kinds_count);
    basic_score += calc_owned_nft_count(p.owned_nft_count);
    basic_score += calc_current_nft_count(p.current_nft_count);

    let value = p.current_nft_value_in_eth.parse::<f64>().unwrap();
    basic_score += calc_current_nft_value_eth(value);
    let score = ScoreResult {
        score: basic_score
    };

    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}

fn calc_ealiest_transfer_days_till_now(days:u32) -> u32 {
    if days == 0{
        0
    } else if days >= 1 && days <= 30 {
        2
    } else if days >= 31 && days <= 90 {
        4
    } else if days >= 91 && days <= 180 {
        6
    } else if days >= 181 && days < 360 {
        8
    } else {
        10
    }
}

fn calc_transfer_count_for_last_year(count:u32) -> u32 {
    if count == 0 {
        0
    }else if count >= 1 && count <= 10{
        3
    }else if count >= 11 && count <= 100{
        6
    }else{
        10
    }
}

fn calc_latest_transfer_days_till_now(days:u32) -> u32 {
    if days == 0{
        0
    } else if days >= 1 && days <= 30 {
        2
    } else if days >= 31 && days <= 90 {
        4
    } else if days >= 91 && days <= 180 {
        6
    } else if days >= 181 && days < 360 {
        8
    } else {
        10
    }
}

fn calc_owned_nft_kinds_count(count:u32) -> u32 {
    if count == 0 {
        0
    }else if count == 1{
        3
    }else if count >= 2 && count <= 3{
        5
    }else if count >= 4 && count <= 10{
        7
    }else{
        10
    }
}

fn calc_owned_nft_count(count:u32) -> u32 {
    if count == 0 {
        0
    }else if count >= 1 && count <=10 {
        3
    }else if count >= 11 && count <= 100 {
        6
    }else{
        10
    }
}

fn calc_current_nft_count(count:u32) -> u32 {
    if count == 0 {
        0
    }else if count >= 1 && count <=10 {
        3
    }else if count >= 11 && count <= 100 {
        6
    }else{
        10
    }
}

fn calc_current_nft_value_eth(value:f64) -> u32 {
    if value == 0 as f64 {
        0
    }else if value >= 1 as f64 && value <=10 as f64 {
        3
    }else if value >= 11 as f64 && value <= 100 as f64 {
        6
    }else{
        10
    }
}