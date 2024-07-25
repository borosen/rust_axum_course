use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello/Mark").await?.print().await?;

    let login_request = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    login_request.await?.print().await?;

    let create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "First ticket"
        }),
    );
    create_ticket.await?.print().await?;

    let _tickets = hc.do_get("/api/tickets").await?.print().await?;

    //hc.do_get("/src/main.rs").await?.print().await?;

    //let delete_ticket = hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
