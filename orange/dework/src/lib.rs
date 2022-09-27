use oscore::runtime;
use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Dework {
    total_tasks:u32,
    total_rewards_usd:f32,
    total_points:f32,

    latest_task_timestamp:u32,
    tasks_per_month:f32,

    oldest_task_timestamp:u32,
    average_points:f32,

}
#[derive(Serialize, Deserialize)]
pub struct Request {
    data :Dework,
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

    let mut basic_score:u32 = 50 * calc_contribution(calc_num_of_task_completed(p.total_tasks),
                                                     calc_amount_of_tokens_received(p.total_rewards_usd),
                                                     calc_number_of_points_awarded(p.total_points));
    basic_score += 25 * calc_activeness(calc_number_of_days_since_most_recent(p.latest_task_timestamp),
                                        calc_average_tasks_in_one_month(p.tasks_per_month));
    basic_score += 25 * calc_proficiency(calc_number_of_days_since_first_task(p.oldest_task_timestamp),
                                         calc_average_points_award_per_task(p.average_points));
    basic_score = basic_score / 100;


    let score = ScoreResult {
        score: basic_score
    };

    let result = serde_json::to_string(&score).unwrap();
    runtime::ret(result.as_ref());
}

fn calc_num_of_task_completed(total_tasks:u32)->u32{
    if total_tasks == 0{
        0
    } else if total_tasks == 1 {
        50
    } else if total_tasks == 2 {
        70
    } else{
        100
    }
}

fn calc_amount_of_tokens_received(total_rewards_usd:f32)->u32 {
    if total_rewards_usd == 0 as f32 {
        0
    }else{
        100
    }
}

fn calc_number_of_points_awarded(total_points:f32) -> u32{
    if total_points == 0 as f32 {
        0
    }else if total_points >= 1 as f32 && total_points <= 10 as f32 {
        50
    }else if total_points >= 11 as f32 && total_points <= 30 as f32 {
        70
    }else if total_points >= 31 as f32 && total_points <= 50 as f32 {
        90
    }else {
        100
    }
}

fn calc_contribution(num_of_task_completed:u32,amount_of_tokens_received:u32,number_of_points_awarde:u32) -> u32{
    (num_of_task_completed * 80 + amount_of_tokens_received * 10 + number_of_points_awarde * 10)/100
}

fn calc_number_of_days_since_most_recent(latest_task_timestamp:u32) -> u32{
    if latest_task_timestamp == 0{
        100
    }else if latest_task_timestamp>=1 && latest_task_timestamp <=30{
        90
    }else if latest_task_timestamp >=31 && latest_task_timestamp <=60{
        70
    }else if latest_task_timestamp >= 61 && latest_task_timestamp <= 90{
        50
    }else{
        0
    }
}

fn calc_average_tasks_in_one_month(tasks_per_month:f32) -> u32{
    if tasks_per_month == 0 as f32{
        0
    }else if tasks_per_month >0 as f32 && tasks_per_month < 1 as f32 {
        50
    }else if tasks_per_month >= 1 as f32 && tasks_per_month<3 as f32 {
        70
    }else if tasks_per_month >=3 as f32 && tasks_per_month < 5 as f32 {
        90
    }else{
        100
    }
}

fn calc_activeness(number_of_days_since_most_recent:u32,average_tasks_in_one_month:u32)->u32{
    (number_of_days_since_most_recent * 50 + average_tasks_in_one_month * 50)/100
}

fn calc_number_of_days_since_first_task(oldest_task_timestamp:u32)->u32{
    if oldest_task_timestamp == 0{
        0
    }else if oldest_task_timestamp >=1 && oldest_task_timestamp <=30{
        50
    }else if oldest_task_timestamp >=31 && oldest_task_timestamp <=60{
        70
    }else if oldest_task_timestamp >= 61 && oldest_task_timestamp <= 90{
        90
    }else {
        100
    }
}

fn calc_average_points_award_per_task(average_points:f32) -> u32{
    if average_points == 0 as f32{
        0
    }else if average_points > 0 as f32 && average_points < 2 as f32{
        50
    }else if average_points >=2 as f32 && average_points <5 as f32 {
        70
    }else if average_points >= 5 as f32 && average_points < 10 as f32{
        90
    }else{
        100
    }
}
fn calc_proficiency(number_of_days_since_first_task:u32,average_points_award_per_task:u32)->u32{
    (number_of_days_since_first_task * 50 + average_points_award_per_task*50)/100
}