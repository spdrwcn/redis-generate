use rand::Rng;
use serde_json::json;
use std::time::Instant;
use clap::Parser;
use regex::Regex;
use rayon::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("redis://127.0.0.1:6379/0"), value_parser = redis_ip_address_parser)]
    ip_address: String,
    #[arg(short, long, default_value_t = 100, value_parser = parse_positive_int)]
    num: usize,
}

fn redis_ip_address_parser(s: &str) -> Result<String, String> {
    let ip_pattern = Regex::new(r"^redis://((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?):([1-9]\d{0,4}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])/([0-9]|1[0-5])$").unwrap();

    if !ip_pattern.is_match(s) {
        return Err("错误: 请输入正确的Redis地址格式, 例如 'redis://127.0.0.1:6379/0'".to_string());
    }
    Ok(s.to_string()) 
}

fn parse_positive_int(s: &str) -> Result<usize, String> {
    match s.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err("必须是正整数".to_string()),
    }
}

fn generate_random_mac() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255)
    )
}

fn generate_key_value_pairs<'a>(num: usize) -> impl ParallelIterator<Item = (String, String)> + 'a {
    (1..=num).into_par_iter().map(move |_i| {
        (
            ulid::Ulid::new().to_string(),
            json!({
                "wired_mac": generate_random_mac(),
                "wireless_mac": generate_random_mac(),
                "bluetooth_mac": generate_random_mac(),
            }).to_string(),
        )
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let cli = Cli::parse();

    let client = redis::Client::open(cli.ip_address)?;
    let mut con = client.get_connection()?;
    let key_value_pairs: Vec<_> = generate_key_value_pairs(cli.num).collect();

    let mut pipe = redis::pipe();
    for (key, value) in key_value_pairs {
        pipe.set(key, value);
    }
    pipe.query(&mut con)?;
    let elapsed_time = start_time.elapsed();
    println!("用时： {:?}", elapsed_time);
    Ok(())
}
