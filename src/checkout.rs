extern crate chrono;
extern crate reqwest;

pub async fn checkout_time() -> Result<(), reqwest::Error> {
    // Specify the URL you want to send a GET request to
    let url = "https://api2.timekeeper.se/api/v1/TimeRegistration/";

    let response = reqwest::get(url).await?;

    if response.status() == reqwest::StatusCode::OK {
        let body = response.text().await?;
        println!("Response body:\n{}", body);
        println!("Checked in at: {:?}", chrono::Local::now());
    } else {
        println!("Request failed with status code: {:?}", response.status());
    }
    Ok(())
}
