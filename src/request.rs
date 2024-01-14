use reqwest;

#[derive(Debug)]
pub struct Request {
    url: String,
    body: String,
}

impl Request {
    pub fn new() -> Self {
        Request {
            url: "None".to_string(),
            body: "None".to_string(),
        }
    }

    pub async fn get(&mut self, url: &str) -> Result<&Self, Box<dyn std::error::Error>> {
        let resp = reqwest::get(url)
            .await?
            .text()
            .await?;

        let body = format!("{:#?}", resp);

        self.url = url.to_string();
        self.body = body;

        Ok(self)
    }
}
