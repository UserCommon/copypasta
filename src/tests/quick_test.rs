#![allow(unused)]

use crate::CONFIG;

use anyhow::Result;
use once_cell::sync::Lazy;

static HOST: Lazy<String> = Lazy::new(|| format!("http://{}", CONFIG.to_string()));
#[tokio::test]
async fn quick_test() -> Result<()> {
    let hc = httpc_test::new_client(HOST.clone())?;

    hc.do_get("/hello_world").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}