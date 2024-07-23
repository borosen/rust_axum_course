#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello/Mark").await?.print().await?;

    let login_request = hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    }));
    login_request.await?.print().await?;

    Ok(())

}