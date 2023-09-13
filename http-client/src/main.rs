#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    eprintln!("-------------PING-------------");
    let res = reqwest::get("http://127.0.0.1:3000/ping").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    eprintln!("-------------GET-------------");
    let res = reqwest::get("http://127.0.0.1:3000/value?key=zju").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    eprintln!("-------------SET-------------");
    let mut map = std::collections::HashMap::new();
    map.insert("key", "zju");
    map.insert("value", "123");
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3000/value")
        .json(&map)
        .send()
        .await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    eprintln!("-------------GET-------------");
    let res = reqwest::get("http://127.0.0.1:3000/value?key=zju").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    eprintln!("-------------DELETE-------------");
    let res = client.delete("http://127.0.0.1:3000/value?key=zju").send().await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    eprintln!("-------------GET-------------");
    let res = reqwest::get("http://127.0.0.1:3000/value?key=zju").await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}