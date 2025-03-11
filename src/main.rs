use reqwest::header::HeaderMap;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Url;
use serde_json::json;
use serde_json::{Value, Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkOrParseError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Serde_Json(#[from] serde_json::Error),
}

struct GeographicCoordinate{
    latitude: f32,
    longitude: f32,
}

struct GridLocation{
    office: String,
    gridX: f64,
    gridY: f64,
}



fn get_grid_location(client: &Client, GeoCoordinate: GeographicCoordinate) -> Result<GridLocation, NetworkOrParseError> {
    //TODO cleanup the unwraps
    let url = format!("https://api.weather.gov/points/{}%2c{}", GeoCoordinate.latitude, GeoCoordinate.longitude);
    let url = Url::parse(&url).unwrap();
    let response = client
        .get(url)
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{0}", response);

    let parsed: serde_json::Value = serde_json::from_str(&response)?;
    Ok(
    GridLocation {
        office: parsed["gridId"].to_string(),
        gridX: parsed["gridX"].as_f64().unwrap(),
        gridY: parsed["gridY"].as_f64().unwrap(),
    }
    )
}

//fn get_forecast(client: &Client, location: Grid_Location) -> Result<String,Error>

fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "rust-test".parse().unwrap());
    headers.insert("Accept", "application/ld+json".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
   let coordinate = GeographicCoordinate{
        latitude: 39.9318,
        longitude: -105.28121,
    };


    let apiResponse = get_grid_location(&client, coordinate).unwrap();

   println!("{}", apiResponse.office);

}
