use rand::Rng;
use serde_json::json;
use std::time::Instant;
use clap::{App, Arg};

fn get_matches() -> clap::ArgMatches<'static> {
    let matches = App::new("redis-generate")
        .version("1.0.0")
        .author("h13317136163@163.com")
        .about("redis数据生成工具")
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("IP_ADDRESS")
                .help("Redis数据库地址")
                .default_value("redis://127.0.0.1:6379/0"),
        )
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("num")
                .value_name("number of pairs")
                .help("生成数量")
                .default_value("10000"),
        )
        .get_matches();
    matches
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

fn generate_key_value_pairs<'a>(num_pairs: usize) -> impl Iterator<Item = (String, String)> + 'a {
    (1..=num_pairs).map(move |i| {
        (
            format!("key{}", i),
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
    let matches = get_matches();
    let ip_address = matches.value_of("ip").unwrap();
    let number = matches.value_of("num").unwrap().parse::<usize>()?;
    let client = redis::Client::open(ip_address)?;
    let mut con = client.get_connection()?;
    
    let mut pipe = redis::pipe();
    for (key, value) in generate_key_value_pairs(number) {
        pipe.set(key, value);
    }
    pipe.query(&mut con)?;
    let elapsed_time = start_time.elapsed();
    println!("用时： {:?}", elapsed_time);
    Ok(())
}
