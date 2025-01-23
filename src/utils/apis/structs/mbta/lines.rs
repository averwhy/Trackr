use serde::Deserialize;
use serde::Serialize;

/// Wrapper Struct for Lines
pub struct Lines {
    pub count: i32,
    pub long_names: Vec<String>,
    pub short_names: Vec<String>,
    pub subways: Vec<Daum>,
}

impl Lines {
    pub fn new(data: Root) -> Self {
        let subways: Vec<Daum> = data
            .data
            .iter()
            .filter(|line| {
                let long_name = &line.attributes.long_name;
                (long_name.contains("Red")
                    || long_name.contains("Orange")
                    || long_name.contains("Blue")
                    || long_name.contains("Green"))
                    // make sure that there arent any stragglers
                    && !long_name.contains("Greenbush")
                    && !long_name.contains("Oak Grove")
                
            })
            .cloned()
            .collect();
        // get names of all lines
        let long_names: Vec<String> = data
            .data
            .iter()
            .map(|line| line.attributes.long_name.clone())
            .collect();
        let short_names: Vec<String> = data
            .data
            .iter()
            .map(|line| line.attributes.short_name.clone())
            .collect();
        let count: i32 = data.data.len().try_into().unwrap();
        Self {
            count,
            long_names,
            short_names,
            subways,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub links: Option<Links>,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub prev: String,
    pub next: String,
    pub last: String,
    pub first: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    #[serde(rename = "type")]
    pub type_field: String,
    pub relationships: Option<Relationships>,
    pub links: Option<Links2>,
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationships {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub text_color: String,
    pub sort_order: i64,
    pub short_name: String,
    pub long_name: String,
    pub color: String,
}
