use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub links: Option<Links>,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
    pub first: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub relationships: Option<Relationships>,
    pub links: Option<Links3>,
    pub id: Option<String>,
    pub attributes: Option<Attributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {
    pub facility: Option<Facility>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Facility {
    pub links: Option<Links2>,
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links2 {
    #[serde(rename = "self")]
    pub self_field: Option<String>,
    pub related: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links3 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub url: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    pub timeframe: Option<String>,
    #[serde(rename = "short_header")]
    pub short_header: Option<String>,
    pub severity: Option<i64>,
    #[serde(rename = "service_effect")]
    pub service_effect: Option<String>,
    pub lifecycle: Option<String>,
    #[serde(rename = "informed_entity")]
    pub informed_entity: Option<Vec<InformedEntity>>,
    #[serde(rename = "image_alternative_text")]
    pub image_alternative_text: Option<String>,
    pub image: Option<String>,
    pub header: Option<String>,
    #[serde(rename = "effect_name")]
    pub effect_name: Option<String>,
    pub effect: Option<String>,
    #[serde(rename = "duration_certainty")]
    pub duration_certainty: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    pub cause: Option<String>,
    pub banner: Option<Option<String>>,
    #[serde(rename = "active_period")]
    pub active_period: Option<Vec<ActivePeriod>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InformedEntity {
    pub trip: Option<String>,
    pub stop: Option<String>,
    #[serde(rename = "route_type")]
    pub route_type: Option<i64>,
    pub route: Option<String>,
    pub facility: Option<String>,
    #[serde(rename = "direction_id")]
    pub direction_id: Option<i64>,
    pub activities: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivePeriod {
    pub start: Option<String>,
    pub end: Option<Option<String>>,
}
