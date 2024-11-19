use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::Duration;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

fn main() {
    let event = Event {
        name: "Event 1".to_owned(),
        date: "2024-11-14".to_owned(),
    };

    let json = serde_json::to_string(&event).unwrap();
    println!("\n{}", json);

    let des_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", des_event);
}
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn request_test() {
        let json_str = std::fs::read_to_string("request.json").unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();

        assert_eq!(request.request_type, RequestType::Success);

        assert_eq!(
            request.stream.user_id,
            uuid!("8d234120-0bda-49b2-b7e0-fbd3912f6cbf")
        );

        assert_eq!(request.stream.is_private, false);
        assert_eq!(request.stream.settings, 45345);
        assert_eq!(
            request.stream.shard_url,
            "https://n3.example.com/sapi".parse().unwrap()
        );

        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.public_tariff.price, 100);
        assert_eq!(
            request.stream.public_tariff.duration,
            Duration::from_secs(3600)
        );
        assert_eq!(
            request.stream.public_tariff.description,
            "test public tariff"
        );

        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(
            request.stream.private_tariff.duration,
            Duration::from_secs(60)
        );
        assert_eq!(
            request.stream.private_tariff.description,
            "test private tariff"
        );

        assert_eq!(request.gifts.len(), 2);

        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[0].price, 2);
        assert_eq!(request.gifts[0].description, "Gift 1");

        assert_eq!(request.gifts[1].id, 2);
        assert_eq!(request.gifts[1].price, 3);
        assert_eq!(request.gifts[1].description, "Gift 2");

        assert_eq!(request.debug.duration, Duration::from_millis(234));
        let expected_at: DateTime<Utc> = "2019-06-28T08:35:46+00:00".parse().unwrap();
        assert_eq!(request.debug.at, expected_at);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: String,
}

fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}
