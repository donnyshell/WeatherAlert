use reqwest::header::HeaderMap;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Url;
use serde_json::json;
use serde_json::{Value, Error};

struct GeographicCoordinate{
    latitude: f32,
    longitude: f32,
}

struct GridLocation{
    office: String,
    gridX: f64,
    gridY: f64,
}

str


fn get_grid_location(client: &Client, GeoCoordinate: GeographicCoordinate) -> Result<GridLocation, reqwest::error> {
    //TODO cleanup the unwraps
    let url = format!("https://api.weather.gov/points/{}%2c{}", GeoCoordinate.latitude, GeoCoordinate.longitude);
    let url = Url::parse(&url)?;
    let response = client
        .get(url)
        .send()?
        .text()?;

    let parsed: serde_json::Value = serde_json::from_str(&response)?;
    GridLocation {
        office: parsed["gridId"].to_string(),
        gridX: parsed["gridX"].as_f64()?,
        gridY: parsed["gridY"].as_f64()?,
    }
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

    let apiResponse = get_grid_location(&client, coordinate);

    println!("{}", apiResponse.office);

}
