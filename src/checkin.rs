extern crate chrono;
use std::io::Read;
use curl::easy::{Easy, List};

pub fn checkin_time() {
    let mut data = "{\"userID\": 11440, \"date\": \"2023-11-02\", \"startTime\": \"2023-11-02 07:00\", \"hours\": null, \"minutes\": null, \"activityID\": 25734, \"timeCodeID\": 41468, \"projectID\": 87916, \"comment\": \"test\", \"isInvoiced\": false}".as_bytes();

    let mut easy = Easy::new();
    easy.url("https://api2.timekeeper.se/api/v1/TimeRegistration").unwrap();
    let mut list = List::new();
    list.append("Authorization: Basic token==").unwrap();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list).unwrap();
    easy.perform().unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();




}
