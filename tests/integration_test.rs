use serde_json::json;

//#[tokio::test]
async fn register() -> Result<(), Box<dyn std::error::Error>> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("http://localhost:8899/user/register")
        .json(&json!({
            "username": "testname",
            "password": "1234567",
        }))
        .send()
        .await?
        .json()
        .await?;

    assert!(echo_json["code"] == json!("OK"), "{}", echo_json);

    Ok(())
}

#[tokio::test]
async fn login() -> Result<(), Box<dyn std::error::Error>> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("http://localhost:8899/user/login")
        .json(&json!({
            "username": "testname",
            "password": "1234567",
        }))
        .send()
        .await?
        .json()
        .await?;

    assert!(echo_json["code"] == json!("OK"), "{}", echo_json);

    Ok(())
}
