// filepath: chat-app/src/main.rs
use std::io::{self, Write};
use reqwest::blocking::Client;
use serde_json::json;

fn main() {
    let client = Client::new();
    let api_url = "http://127.0.0.1:1234/v1/chat/completions"; // عدل المنفذ إذا لزم

    println!("ابدأ الدردشة مع النموذج. اكتب 'خروج' للخروج.");

    loop {
        print!("أنت: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("خروج") {
            break;
        }

        let payload = json!({
            "messages": [
                {"role": "user", "content": user_input}
            ]
        });

        let response = client.post(api_url)
            .json(&payload)
            .send();

        match response {
            Ok(resp) => {
                if let Ok(json_resp) = resp.json::<serde_json::Value>() {
                    if let Some(reply) = json_resp["choices"][0]["message"]["content"].as_str() {
                        println!("نموذج: {}", reply.trim());
                    } else {
                        println!("خطأ في استجابة النموذج.");
                    }
                } else {
                    println!("تعذر قراءة استجابة النموذج.");
                }
            }
            Err(e) => {
                println!("فشل الاتصال بالنموذج: {}", e);
            }
        }
    }
}