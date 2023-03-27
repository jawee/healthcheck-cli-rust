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

fn get_urls_for_env(env: &str) -> Vec<&str> {
    let urls = match env {
        "dev" => {
            let urls = vec![
                "https://api-dev.eastcoast-online.net/version",
                "https://dev.eastcoast-online.net",
                "https://app-dev.eastcoastexpress.net/api/v2/version",
                "https://dev-evac-api.eastcoast-online.net/api/version",
                "https://new-dev.eastcoast-online.net/"
            ];
            
            urls
        },
        "stage" => {
            let urls = vec![
                "https://api-stage.eastcoast-online.net/version",
                "https://stage.eastcoast-online.net",
                "https://app-stage.eastcoastexpress.net/api/v2/version",
                "https://stage-evac-api.eastcoast-online.net/api/version",
                "https://new-stage.eastcoast-online.net/"
            ];
            
            urls
        },
        "prod" => {
            let urls = vec![
                "https://api.eastcoast-online.net/version",
                "https://eastcoast-online.net",
                "https://app.eastcoastexpress.net/api/v2/version",
                "https://evac-api.eastcoast-online.net/api/version",
                "https://new.eastcoast-online.net/"
            ];
            
            urls
        }
        _ => {
            panic!("Unknown env");
        }
    };

    return urls;
}

async fn check_url(url: &str) {
    let res = HealthCheckResult::new(url).await;
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

