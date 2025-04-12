use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Agency Creation"]
pub struct AgencyInfoModal {
    #[name = "Agency Abbreviation"]
    #[placeholder = "e.g. MBTA"]
    short_name: String,

    #[name = "Full Agency Name"]
    #[placeholder = "e.g. Metro Transit Authority"]
    long_name: String,

    #[name = "API Url"]
    #[placeholder = "Include https://"]
    api_url: String,

    #[name = "API Key Environment Variable"]
    #[placeholder = "e.g. MBTA_API_KEY"]
    key_env_name: String,

    #[name = "Authorization Header Name"]
    #[placeholder = "e.g. 'Authorization'"]
    auth_header_name: String,
}

#[derive(Debug, Modal)]
#[name = "Agency Endpoint Creation"]
pub struct AgencyEndpointModal {
    #[name = "Endpoint Type"]
    #[placeholder = "=dev endpoints for a list"]
    endpoint_type: String,

    #[name = "Endpoint Path"]
    #[placeholder = "e.g. /lines/"]
    endpoint_path: String,
}

#[derive(Debug, Modal)]
#[name = "Agency Pointer Creation"]
pub struct AgencyPointerModal {
    #[name = "JSON Pointer Path"]
    #[placeholder = "e.g. /data/{index}/attributes/updated_at"]
    endpoint_path: String,
}
