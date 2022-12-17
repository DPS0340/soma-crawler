use once_cell::sync::Lazy;

static BASE_URL: &str = "https://swmaestro.org";
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Reqwest client build failed")
});

pub mod env {
    pub fn parse_env(file_path: &str) -> Vec<Vec<String>> {
        use std::fs;

        fs::read_to_string(file_path)
            .expect(&format!("env file {file_path} not exists"))
            .split("\n")
            .map(|line| {
                let splitted: Vec<String> = line.split('=').map(|word| word.to_string()).collect();

                match splitted.len() {
                    2 => splitted,
                    _ => panic!(".env contents must follow key=val format, but {line} given"),
                }
            })
            .collect::<Vec<_>>()
    }
}

pub mod http {
    use crate::{BASE_URL, HTTP_CLIENT};

    pub async fn get_csrf_token() -> String {
        let res = HTTP_CLIENT
            .get(format!(
                "{BASE_URL}/sw/member/user/forLogin.do?menuNo=200025"
            ))
            .send()
            .await
            .expect("Request failed");

        let body = res.text().await.expect("Failed to get payload");

        let dom = tl::parse(body.as_str(), tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let element = dom
            .query_selector("#csrfToken")
            .expect("Failed to parse query selector")
            .next()
            .expect("Failed to find CSRF Token");

        let input = element
            .get(parser)
            .expect("Failed to resolve node")
            .as_tag()
            .expect("Failed to cast Node to HTMLTag");

        let attributes = input.attributes();

        attributes
            .get("value")
            .flatten()
            .expect("Value not found")
            .as_utf8_str()
            .to_string()
    }

    async fn sign_in() {
        // let res = HTTP_CLIENT
        //     .post(format!("{BASE_URL}/sw/login.do"))
        //     .header("Content-Type", "application/x-www-form-urlencoded")
        //     .header("Authorization", "Basic ".to_owned() + &secret)
        //     .send()
        //     .await?;
    }
}

pub mod parse {}

pub mod json_dump {}

fn main() {
    let env_path = "./.env";

    let a = env::parse_env(env_path);
}

mod abiria {
    pub(super) fn parse_env<P: AsRef<std::path::Path>>(
        file: P,
    ) -> std::collections::HashMap<String, String> {
        std::io::BufRead::lines(std::io::BufReader::new(std::fs::File::open(file).unwrap()))
            .map(Result::unwrap)
            .map(|l| {
                let (a, b) = l.split_once("=").unwrap();

                (a.to_owned(), b.to_owned())
            })
            .collect()
    }
}
