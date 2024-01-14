#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        
        let url = "https://www.youtube.com/watch?v=W13Ydr_AcjI";

            let resp = reqwest::get(url)
                        .await?
                                .text()
                                        .await?;
        println!("{:#?}", resp);
            Ok(())
}
