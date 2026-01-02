use std::{env, fs};

use dotenv::dotenv;

use crate::{
    dtos::quests::Category,
    services::{get_data::get_data, telegram::TelegramDto},
};

pub async fn scrape(event_name: String) -> Result<(), String> {
    dotenv().ok();

    let link = format!(
        "https://api-v1.zealy.io/communities/{}/questboard/v2",
        &event_name
    );
    let file_path = format!("{}.json", &event_name);

    let result = match get_data(link).await {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e.to_string());
            return Err(e.to_string());
        }
    };

    let text = match result.text().await {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e.to_string());
            return Err(e.to_string());
        }
    };

    let current_payload = match serde_json::from_str::<Vec<Category>>(&text) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e.to_string());
            return Err(e.to_string());
        }
    };

    let text_json = match serde_json::to_string_pretty(&current_payload) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e.to_string());
            return Err(e.to_string());
        }
    };

    let file_content = fs::read_to_string(&file_path);

    match file_content {
        Ok(x) => {
            let stored_payload = match serde_json::from_str::<Vec<Category>>(&x) {
                Ok(x) => x,
                Err(e) => {
                    println!("{:?}", e.to_string());
                    return Err(e.to_string());
                }
            };

            for i in current_payload {
                let stored_category = stored_payload.iter().find(|p| p.id == i.id);

                match stored_category {
                    Some(y) => {
                        for o in &i.quests {
                            match y.quests.iter().find(|ee| ee.id == o.id) {
                                Some(u) => {
                                    if u.claimed != o.claimed
                                        || u.locked != o.locked
                                        || u.opened != o.opened
                                        || u.completed != o.completed
                                    {
                                        let body = match TelegramDto::new(&format!(
                                        "A quest has been updated: \n https://zealy.io/cw/{}/questboard/{}/{}",
                                        &event_name,i.id, o.id
                                    )).serialize_body() {
                                        Ok(x) => x,
                                        Err(e) => {
                                            println!("{:?}", e.to_string());
                                            return Err(e.to_string());
                                        }
                                    };

                                        let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
                                            Ok(x) => x,
                                            Err(e) => {
                                                eprintln!("{}", e.to_string());
                                                return Err(e.to_string());
                                            }
                                        };

                                        match TelegramDto::send_message(&bot_token, body).await {
                                            Err(e) => {
                                                eprintln!("{}", e.to_string());
                                                return Err(e.to_string());
                                            }
                                            Ok(p) => {
                                                if p.status() != 200 {
                                                    match p.text().await {
                                                        Ok(x) => {
                                                            return Err(x);
                                                        }
                                                        Err(e) => {
                                                            return Err(e.to_string());
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                        fs::write(&file_path, &text_json).unwrap();
                                    }
                                }
                                None => {
                                    let body = match TelegramDto::new(&format!(
                                        "A new quest has been added: \n https://zealy.io/cw/{}/questboard/{}/{}",
                                        event_name,i.id, o.id
                                    )).serialize_body() {
                                        Ok(x) => x,
                                        Err(e) => {
                                            println!("{:?}", e.to_string());
                                            return Err(e.to_string());
                                        }
                                    };

                                    let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
                                        Ok(x) => x,
                                        Err(e) => {
                                            eprintln!("{}", e.to_string());
                                            return Err(e.to_string());
                                        }
                                    };

                                    match TelegramDto::send_message(&bot_token, body).await {
                                        Err(e) => {
                                            eprintln!("{}", e.to_string());
                                            return Err(e.to_string());
                                        }
                                        Ok(p) => {
                                            if p.status() != 200 {
                                                match p.text().await {
                                                    Ok(x) => {
                                                        return Err(x);
                                                    }
                                                    Err(e) => {
                                                        return Err(e.to_string());
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    fs::write(&file_path, &text_json).unwrap();
                                }
                            };
                        }
                    }
                    _ => {
                        let body = match TelegramDto::new(&format!(
                            "A new category has been added: \n https://zealy.io/cw/{}/questboard/{}",
                            &event_name,i.id
                        )).serialize_body() {
                            Ok(x) => x,
                            Err(e) => {
                                println!("{:?}", e.to_string());
                                return Err(e.to_string());
                            }
                        };

                        let bot_token = match env::var("TELEGRAM_BOT_TOKEN") {
                            Ok(x) => x,
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                return Err(e.to_string());
                            }
                        };

                        match TelegramDto::send_message(&bot_token, body).await {
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                return Err(e.to_string());
                            }
                            Ok(p) => {
                                if p.status() != 200 {
                                    match p.text().await {
                                        Ok(x) => {
                                            return Err(x);
                                        }
                                        Err(e) => {
                                            return Err(e.to_string());
                                        }
                                    };
                                }
                            }
                        }
                        fs::write(&file_path, &text_json).unwrap();
                    }
                }
            }
        }
        _ => {
            fs::write(&file_path, text_json).unwrap();
            println!("created file: {}", &file_path)
        }
    }

    Ok(())
}
