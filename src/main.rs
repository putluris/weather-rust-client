use std::collections::HashMap;

mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("username", "putluris");
    map.insert("password", "password");

    let auth_response = client
        .post("http://localhost:3000/v1/auth/")
        .json(&map)
        .send()
        .await?;

    let token_model = auth_response
        .json::<model::Token>()
        .await?;

    println!("\nToken obtained!\n");

    let jwt_key = token_model.jwt;

    let home = client
        .get("http://localhost:8001/v1/hello/")
        .bearer_auth(&jwt_key)
        .send()
        .await?
        .text()
        .await?;
    println!("Welcome Page Content: {:?} ", home);

    let weather_response = client
        .get("http://localhost:8001/v1/weather/")
        .bearer_auth(&jwt_key)
        .send()
        .await?;

    let weather = weather_response
        .json::<model::Weather>()
        .await?;

    println!("\nWeather data collected:\n{:?}",weather);

    Ok(())
}