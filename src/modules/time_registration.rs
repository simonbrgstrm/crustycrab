extern crate reqwest;
extern crate tokio;

use reqwest::Error;

pub async fn make_get_request() -> Result<(), reqwest::Error> {
    // Specify the URL you want to send a GET request to
    let url = "https://jsonplaceholder.typicode.com/posts/1"; // Replace with your desired URL

    let response = reqwest::get(url).await?;
    // reqwest::get(url).await?.text().await?;
    //
    if response.status() == reqwest::StatusCode::OK {
        let body = response.text().await?;
        println!("Response body:\n{}", body);
    } else {
        println!("Request failed with status code: {:?}", response.status());
    }
    Ok(()) // Return a Result indicating success
}

fn main() {
    make_get_request();
}
