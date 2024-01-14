use MusicLink::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.youtube.com/watch?v=W13Ydr_AcjI";

    let mut request = Request::new();
    println!("{:?}", request);
    request.get(url).await?;
    println!("{:?}", request);

    Ok(())
}
