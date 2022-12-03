mod solutions;
use crate::solutions::{day_1, day_2, day_3};

fn main() {
    let problem = "3_2";

    match problem {
        "1_1" => day_1::run_first().unwrap(),
        "1_2" => day_1::run_second().unwrap(),
        "2_1" => day_2::run_first().unwrap(),
        "2_2" => day_2::run_second().unwrap(),
        "3_1" => day_3::run_first().unwrap(),
        "3_2" => day_3::run_second().unwrap(),
        _ => println!("No such solution found."),
    };

}
