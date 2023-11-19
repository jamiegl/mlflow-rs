use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    api::{opt_str_int, str_int},
    id::{ExperimentId, RunId}
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric<'a> {
    pub key: Cow<'a, str>,
    pub value: f64,
    pub timestamp: i64,
    pub step: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Run {
    pub info: RunInfo,
    pub data: RunData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RunData {
    pub metrics: Option<Vec<Metric<'static>>>,
    pub params: Option<Vec<Param>>,
    pub tags: Option<Vec<RunTag>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunInfo {
    pub run_id: RunId,
    #[deprecated = "This field will be removed in a future FLflow version"]
    pub run_uuid: String,
    pub experiment_id: ExperimentId,
    #[deprecated = "This field will be removed in a future FLflow version"]
    pub user_id: String,
    pub status: RunStatus,
    #[serde(with = "str_int")]
    pub start_time: i64,
    #[serde(default, with = "opt_str_int")]
    pub end_time: Option<i64>,
    pub artifact_uri: String,
    pub lifecycle_stage: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RunStatus {
    Running,
    Scheduled,
    Finished,
    Failed,
    Killed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunTag {
    pub key: String,
    pub value: String,
}
