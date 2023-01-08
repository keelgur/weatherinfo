use chrono::prelude::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

// Tells Rust to generate a command line parser, and the various structopt attributes are simply used for additional parameters.
#[derive(StructOpt)]
struct Cli {
    command: String,
    operand: String,
    addition: Option<String>,
}

//This attribute is just so that there will be no annoying warnings
#[allow(non_snake_case)]
//Attributes for correct parsing
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    queryCost: i32,
    latitude: f64,
    longitude: f64,
    resolvedAddress: String,
    address: String,
    timezone: String,
    tzoffset: f32,
    days: Vec<Days>,
    stations: Option<HashMap<String, Station>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Days {
    datetime: String,
    datetimeEpoch: i64,
    tempmax: f32,
    tempmin: f32,
    temp: f32,
    feelslikemax: f32,
    feelslikemin: f32,
    feelslike: f32,
    dew: f32,
    humidity: f32,
    precip: f64,
    precipprob: f32,
    precipcover: f32,
    preciptype: Option<Vec<String>>,
    snow: Option<f32>,
    snowdepth: f32,
    windgust: Option<f32>,
    windspeed: f32,
    winddir: f32,
    pressure: f64,
    cloudcover: f32,
    visibility: f32,
    solarradiation: f32,
    solarenergy: f32,
    uvindex: f32,
    sunrise: String,
    sunriseEpoch: i64,
    sunset: String,
    sunsetEpoch: i64,
    moonphase: f64,
    conditions: String,
    description: String,
    icon: String,
    stations: Option<Vec<String>>,
    source: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Station {
    distance: f64,
    latitude: f64,
    longitude: f64,
    useCount: i32,
    id: String,
    name: String,
    quality: f32,
    contribution: f32,
}

impl Forecast {
    async fn get(ct: &str, dt: Option<String>, provider: String) -> Result<Self, ExitFailure> {
        let date = match dt {
            Some(s) => s,
            None => Local::now().format("%Y-%m-%d").to_string(),
        };
        //Matching configured provider to format url respectively
        let url = match provider.as_str() {
            "vc" => format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{}/{}/{}?unitGroup=metric&include=days&key=V4XLHDYZE9XTNU8B28HC23HQ6&contentType=json", ct, date, date),
            other => {println!("Provider {} not yet implemented. Using only implemented provider vc.", other);
            format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{}/{}/{}?unitGroup=metric&include=days&key=V4XLHDYZE9XTNU8B28HC23HQ6&contentType=json", ct, date, date)},
        };
        let url = Url::parse(&url)?;

        //Parsing acquired JSON
        let resp = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(resp)
    }
}

//Attribute for getting fast responses from API
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let cargs = Cli::from_args();
    let response;
    //Opening a file to read configured provider
    let filn = "cfg.txt";
    let mut s = String::new();
    let mut ff = File::open(filn)?;
    ff.read_to_string(&mut s)?;
    println!("API provider is set to: {}", &s);
    //Matching command from user input
    match cargs.command.as_str() {
        "get" => {
            response = Forecast::get(&cargs.operand, cargs.addition, s).await?;
            println!("{}, Temp: {} °C, Precipitation probability: {}%, Humidity: {}%, Wind speed: {}km/h, {}", cargs.operand, response.days[0].temp, response.days[0].precipprob, response.days[0].humidity, response.days[0].windspeed, response.days[0].description);
        }
        "configure" => {
            let mut fff = File::create(filn)?; //Clearing the file by creating it again(possible due to File::create() definition)
            fff.write_all(cargs.operand.as_bytes())?;
            println!("Changed provider to {}", &cargs.operand)
        }
        other => {
            println!("Invalid command: {}", other)
        }
    }

    Ok(())
}
