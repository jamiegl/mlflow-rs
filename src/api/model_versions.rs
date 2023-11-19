use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    name: String,
    version: String,
    creation_timestamp: i64,
    last_updated_timestamp: i64,
    user_id: String,
    current_stage: String,
    description: String,
    source: String,
    run_id: String,
    status: ModelVersionStatus,
    status_message: String,
    tags: Vec<ModelVersionTag>,
    run_link: String,
    aliases: String
}

// TODO - Impl a deserializer for tags
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelVersionTag {
    key: String,
    value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelVersionStatus {
    PENDING_REGISTRATION,
    FAILED_REGISTRATION,
    READY
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SearchModelVersions<'a> {
    pub filter: &'a str,
    pub max_results: i32,
    pub order_by: Option<&'a str>,
    pub page_token: Option<&'a str>, 
}

// #[derive(Deserialize)]
// pub struct SearchModelResponse {
//     pub model_versions: Vec<Run>,
//     pub next_page_token: PageToken
// }

// impl Endpoint for SearchModelVersions<'_> {
//     const PATH: &'static str = "2.0/mlflow/mode-versions/search";
//     const METHOD: RestMethod = RestMethod::Get;
//     type Response = SearchModelResponse;
//     type Value = SearchModelResponse
//     fn extract(response: Self::Response) -> Self::Value {
//         response
//     }
// }