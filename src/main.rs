mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/data")
        .send()
        .await?;

    let weather2 = response
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from openweathermap.org:\n {:?}", weather2);

    Ok(())
}
