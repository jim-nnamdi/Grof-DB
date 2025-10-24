use anyhow::Ok;
use reqwest::StatusCode;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

#[derive(Clone)]
struct NClient {
    inner: reqwest::Client
}

impl NClient {
    pub fn new() -> Self {
        let inner = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3)).build().unwrap();
        NClient { inner }
    }

    pub async fn get<T:serde::de::DeserializeOwned>(&self, url: &str)
    -> Result<T, anyhow::Error> {
        let res = self.inner.get(url).send().await?;
        let sta = res.status();
        let refs = res.bytes().await?;
        if sta != StatusCode::OK && !sta.is_success(){
            anyhow::bail!("{}", String::from_utf8_lossy(&refs))
        }
        let dat:T = serde_json::from_slice(&refs)?;
        Ok(dat)
    }

    pub async fn stream<T:serde::de::DeserializeOwned>(&self, url: &str, p:&str)->Result<(), anyhow::Error>{
        let mut res = self.inner.get(url).send().await?;
        let mut fil = tokio::fs::File::create(p).await?;
        
        let sta = res.status();
        if !sta.is_success() && sta != StatusCode::OK { return Ok(());}
        while let Some(f) = res.chunk().await? {
            fil.write_all(&f).await?; }
        Ok(())
    }

    pub async fn post<T:Serialize>(&self, url:&str, dat:&T) -> Result<String,anyhow::Error> {
        let res = self.inner.post(url).form(dat).send().await?;
        let sta = res.status();
        if !sta.is_success() && sta != StatusCode::OK { 
            return Ok("post failed".to_owned());}
        let bod = res.text().await?;
        Ok(bod)
    }
}