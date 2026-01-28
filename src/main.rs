use std::{env, sync::Arc, time::Duration};

use dotenv::dotenv;

use crate::{config::start::start, services::scrape::scrape};

pub mod config;
pub mod dtos;
pub mod services;

#[actix_web::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    let raw_links = match env::var("EVENTS") {
        Ok(x) => x,
        Err(e) => {
            println!("{:#?}", e.to_string());
            return Err(e.to_string());
        }
    };

    let links: Arc<Vec<String>> =
        Arc::new(raw_links.split(',').map(|s| s.trim().to_string()).collect());

    for link in links.iter() {
        let link = link.clone();
        tokio::spawn(async move {
            loop {
                match scrape(link.to_string()).await {
                    Err(e) => {
                        println!("{:#?}", e.to_string());
                    }
                    _ => {}
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    start().await
}
