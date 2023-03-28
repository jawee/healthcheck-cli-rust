use std::{time::Instant, env};

use anyhow::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided");
        return Ok(());
    }

    let env = &args[1];
    let urls = get_urls_for_env(env);

    for url in urls {
        check_url(url).await;
    }

    return Ok(());
}

fn generate_vec(env: &str) -> Vec<String> {
    let env_str = match env {
        "dev" => "-dev",
        "stage" => "-stage",
        _ => ""
    };

    let dot = match env_str {
        "" => "",
        _ => "."
    };

    let dash = match env_str {
        "" => "",
        _ => "-"
    };
    let urls = vec![
        format!("https://api{}.eastcoast-online.net/version", env_str),
        format!("https://{}{}eastcoast-online.net", env_str.replace("-", ""), dot),
        format!("https://app{}.eastcoastexpress.net/api/v2/version", env_str),
        format!("https://{}{}evac-api.eastcoast-online.net/api/version", env_str.replace("-",""), dash),
        format!("https://new{}.eastcoast-online.net/", env_str)
    ];
    return urls;
}
fn get_urls_for_env(env: &str) -> Vec<String> {
    let urls = generate_vec(env);
    return urls;
}

async fn check_url(url: String) {
    let res = HealthCheckResult::new(&url).await;
    if res.is_err() {
        println!("{} Failed.", url);
        return;
    }

    let result = res.unwrap();
    println!("{} Status: {} Response time: {}ms", url, result.response_code, result.time);
}

#[derive(Debug)]
struct HealthCheckResult {
    pub time: u128,
    pub response_code: u16
}

impl HealthCheckResult {
    pub async fn new(url: &str) -> Result<HealthCheckResult> {
        let now = Instant::now();
        
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let resp = client.get(url).send().await?;
        let time = now.elapsed().as_millis();

        return Ok(HealthCheckResult { time, response_code: resp.status().as_u16() });
    }
}

