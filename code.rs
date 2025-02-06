use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::env;

#[derive(Serialize)]
struct LoginData<'a> {
    email: &'a str,
    password: &'a str,
    undelete: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
}

fn login(email: &str, password: &str) {
    let client = Client::new();
    let data = LoginData {
        email,
        password,
        undelete: "false",
    };

    let url = "https://discord.com/api/v10/auth/login";
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.135 Safari/537.36"),
    );

    let res = client
        .post(url)
        .json(&data)
        .headers(headers)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json: Result<TokenResponse, _> = response.json();
                match json {
                    Ok(token_response) => {
                        println!("\x1b[32m>\x1b[37m Token: {}", token_response.token);
                        println!("\n\x1b[31m>\x1b[37m Successfully Fetched Token");
                        Command::new("cmd")
                            .args(&["/C", "pause"])
                            .output()
                            .expect("Failed to execute pause command");
                        std::process::exit(0);
                    }
                    Err(_) => {
                        println!("\x1b[32m>\x1b[37m An error occurred while parsing the token");
                    }
                }
            } else {
                let text = response.text().unwrap_or_else(|_| String::from("Unknown error"));
                if text.contains("PASSWORD_DOES_NOT_MATCH") {
                    println!("\x1b[32m>\x1b[37m Invalid Password");
                } else if text.contains("captcha-required") {
                    println!("\x1b[32m>\x1b[37m Discord Returned Captcha, Try Again Later");
                } else {
                    println!("\x1b[32m>\x1b[37m An Error Has Occurred");
                }
                Command::new("cmd")
                    .args(&["/C", "pause"])
                    .output()
                    .expect("Failed to execute pause command");
                std::process::exit(0);
            }
        }
        Err(_) => {
            println!("\x1b[32m>\x1b[37m An Error Has Occurred");
            Command::new("cmd")
                .args(&["/C", "pause"])
                .output()
                .expect("Failed to execute pause command");
            std::process::exit(0);
        }
    }
}

fn main() {
    println!("\x1b[31m>\x1b[37m Email\x1b[31m: \x1b[37m");
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).expect("Failed to read email");
    let email = email.trim();

    println!("\x1b[31m>\x1b[37m Password\x1b[31m: \x1b[37m");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = password.trim();

    println!();

    login(email, password);
}
