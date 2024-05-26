use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use redis::Commands;
use serde_json::json;
use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let pb = ProgressBar::new(1_000_000);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )?
            .progress_chars("#>-"),
    );

    let mut errors = Vec::new();

    for i in 0..1_000_000 {
        let key = format!("key{}", i);
        let value = json!({
            "wired_mac": generate_random_mac(),
            "wireless_mac": generate_random_mac(),
            "bluetooth_mac": generate_random_mac()
        });

        if let Err(error) = con.set::<&str, &str, String>(&key, &*value.to_string()) {
            errors.push(format!(
                "Unable to set value in Redis for key {}: {}",
                key, error
            ));
        } else {
            pb.inc(1);
        }
    }

    pb.finish_with_message("");

    if !errors.is_empty() {
        eprintln!("The following errors occurred:");
        for error in &errors {
            eprintln!("{}", error);
        }
    }

    Ok(())
}
