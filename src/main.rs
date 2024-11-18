use display_info::DisplayInfo;
use std::env;

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn calculate_gcd(x: u32, y: u32) -> u32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let monitor_id = args[0].parse::<usize>().unwrap_or(0);
    let display_infos = DisplayInfo::all().unwrap();
    let gcd: u32;
    let aspect_ratio;
    let display_info = if display_infos.len() >= monitor_id {
        &display_infos[monitor_id]
    } else {
        &display_infos[0]
    };
    gcd = calculate_gcd(display_info.width, display_info.height);
    aspect_ratio = format!("{}:{}", display_info.width / gcd, display_info.height / gcd);
    println!(
        "{},{},{},{}",
        display_info.width, display_info.height, display_info.frequency, aspect_ratio
    );
}
