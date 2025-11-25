#![allow(dead_code)]




pub fn get_act(level_code: u32) -> i32 {
    if level_code < 40 {
        0
    } else if level_code < 75 {
        1
    } else if level_code < 103 {
        2
    } else if level_code < 109 {
        3
    } else if level_code < 200 {
        4
    } else {
        -1
    }
}
