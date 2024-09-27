use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use colored::Colorize;

#[derive(Serialize)]
struct RequestBody {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize)]
struct CandidateContent {
    parts: Vec<PartResponse>,
    #[allow(dead_code)]
    role: String,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

async fn get_answer(client: &Client, question: &str) -> Result<String, Box<dyn Error>> {
    let request_body = RequestBody {
        contents: vec![Content {
            parts: vec![Part {
                text: question.to_string(),
            }],
        }],
    };

    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=APIKEYGEMINI") 
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    
    if !status.is_success() {
        println!("{} : Status[{}] - Body[{}]","[ERROR]--->".red(), status, body);
        return Err(Box::from(format!("{} {}","[TIMEOUT]--->".yellow(), status)));
    }

    let response_body: ResponseBody = serde_json::from_str(&body)?;
    let answer = response_body.candidates[0].content.parts[0].text.clone().replace("*", "");

    Ok(answer.trim().to_string())
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    println!("\n");
    println!(r"      _        _    ____      ___    __    __    ");
    println!(r"     / \\     | |  | |__)    / _ \\  \\\\ / /    ");
    println!(r"    / _ \\    | |  |  _ \\  | | | |   \\ V /     ");
    println!(r"   / ___ \\   | |  | |_) |  | |_| |   / . \\     ");
    println!(r"  /_/   \\_\  \_|  |____/   \\___/   /_/ \\\\    ");
    
    println!("{}","\n\nWellcome to AIBOX . . .\nAuthor  : ducanhkhuong\nVersion : 0.1\nRelease : 26/09/2024\n");


    loop {
        print!("{}","\n[AIBOX]---> : Đây là AIBOX , tôi có thể giúp gì ?");
        print!("{}","\n[DUCANHKHUONG]---> : ");
        io::stdout().flush()?;

        let mut question = String::new();
        io::stdin().read_line(&mut question)?;
        let question = question.trim();

        if question.eq_ignore_ascii_case("exit") {
            break;
        }

        match get_answer(&client, question).await {
            Ok(answer) => {
                println!("{} : {}","\n[AIBOX]-->", answer);
            },
            Err(e) => println!("{} : {}","[ERROR] --->", e),
        }
    }

    Ok(())
}
