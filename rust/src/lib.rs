mod bindings;

use crate::bindings::exports::request::sender::api::*;
use std::cell::RefCell;

use reqwest::Client;
use serde::{Deserialize, Serialize};

struct Component;

impl Guest for Component {
    fn get(url: String) -> Result<String, String> {
        println!("Sending a GET request to {} via HTTP", url);
        let client = Client::builder()
            .build()
            .map_err(|err| format!("Failed to create the client!"))?;

        let response = client
            .get(url)
            .send()
            .map_err(|err| format!("Failed to send: {}", err))?;

        let response_body = response
            .text()
            .map_err(|err| format!("Failed to convert to text: {}", err))?;

        println!("Result: {:?}", response_body);

        Ok(response_body)
        // Ok(String::from("Result from 'get'"))
    }

    fn post(url: String) -> Result<String, String> {
        println!("Sending a POST request to {} via HTTP", url);
        let client = Client::builder()
            .build()
            .map_err(|err| format!("Failed to create the client!"))?;

        let response = client
            .get(url)
            .send()
            .map_err(|err| format!("Failed to send: {}", err))?;

        let response_body = response
            .text()
            .map_err(|err| format!("Failed to convert to text: {}", err))?;

        println!("Result: {:?}", response_body);

        Ok(response_body)
        // Ok(String::from("Result from 'post'"))
    }
}
