use std::{collections::HashMap, env, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceApiResponse {
    pub api: String,
    #[serde(rename = "api_compatibility")]
    pub api_compatibility: Vec<String>,
    pub space: String,
    pub logo: String,
    pub url: String,
    #[serde(rename = "ext_ccc")]
    pub ext_ccc: String,
    pub location: Location,
    pub state: State,
    pub contact: Contact,
    #[serde(rename = "issue_report_channels")]
    pub issue_report_channels: Vec<String>,
    pub sensors: Sensors,
    pub feeds: Feeds,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub address: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub open: bool,
    pub lastchange: i64,
    pub icon: Icon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub open: String,
    pub closed: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub phone: String,
    pub twitter: String,
    pub email: String,
    pub matrix: String,
    pub ml: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensors {
    pub temperature: Vec<Temperature>,
    #[serde(rename = "network_connections")]
    pub network_connections: Vec<NetworkConnection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub value: f64,
    pub unit: String,
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnection {
    #[serde(rename = "type")]
    pub type_field: String,
    pub location: String,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feeds {
    pub wiki: Wiki,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub url: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RoomStatus {
    Open,
    Closed,
}

impl fmt::Display for RoomStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
        }
    }
}

pub fn send_room_status(matrix_url: &str, status: RoomStatus) -> Result<(), &'static str> {
    let s = fmt::format(format_args!("Der Raum ist aktuell {}!", status));

    let mut map = HashMap::new();
    map.insert("text", s);

    let client = reqwest::blocking::Client::new();
    let response = client.post(matrix_url).json(&map).send();
    if response.is_ok() {
        let status_code = response.unwrap().status();
        if status_code != 202 {
            return Err("failed to send room status: {response:?}");
        }

        return Ok(());
    }

    return Err("failed to send room status: {}");
}

pub fn fetch_room_status(url: &str) -> RoomStatus {
    let response: SpaceApiResponse = reqwest::blocking::get(url).unwrap().json().unwrap();

    match response.state.open {
        true => {
            println!("Der Raum ist aktuell offen!");
            return RoomStatus::Open;
        }
        false => {
            println!("Der Raum ist aktuell geschlossen!");
            return RoomStatus::Closed;
        }
    }
}

pub struct Config {
    pub webhook_url: String,
    pub spaceapi_url: String,
    pub fetch_interval: u64,
    pub language: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            webhook_url: "".to_string(),
            spaceapi_url: "".to_string(),
            fetch_interval: 5,
            language: "".to_string(),
        }
    }

    pub fn get_from_env(&mut self) {
        let webhook_url_key = "WEBHOOK_URL";
        match env::var(webhook_url_key) {
            Ok(val) => self.webhook_url = val,
            Err(e) => panic!(
                "failed to fetch env variable {:?}: {:?}",
                webhook_url_key, e
            ),
        }

        let spaceapi_url_key = "SPACEAPI_URL";
        match env::var(spaceapi_url_key) {
            Ok(val) => self.spaceapi_url = val,
            Err(e) => panic!(
                "failed to fetch env variable {:?}: {:?}",
                spaceapi_url_key, e
            ),
        }

        let fetch_interval_key = "FETCH_INTERVAL";
        match env::var(fetch_interval_key) {
            Ok(val) => self.fetch_interval = val.parse::<u64>().unwrap(),
            Err(e) => panic!(
                "failed to fetch env variable {:?}: {:?}",
                fetch_interval_key, e
            ),
        }

        let language_key = "LANGUAGE";
        match env::var(language_key) {
            Ok(val) => self.language = val,
            Err(e) => panic!("failed to fetch env variable {:?}: {:?}", language_key, e),
        }
    }
}
