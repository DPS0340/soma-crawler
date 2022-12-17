use soma_crawler::{env::parse_env, http::get_csrf_token};

#[tokio::test]
async fn get_csrf_token_test() {
    let result = get_csrf_token().await;

    assert!(!result.is_empty());
}

#[test]
fn parse_env_test() {
    let env_file_path = "./.example.env";

    let result = parse_env(env_file_path);

    assert_eq!(result.len(), 2);

    assert_eq!(result[0], vec!["soma-email", "optional.int@kakao.com"]);
    assert_eq!(result[1], vec!["soma-password", "1234"]);
}
