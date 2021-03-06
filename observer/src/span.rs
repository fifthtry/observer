use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize)]
pub struct Span {
    pub id: String,
    key: String,
    pub breadcrumbs: HashMap<String, serde_json::Value>,
    pub success: Option<bool>,
    pub result: Option<serde_json::Value>,
    pub err: Option<String>,
    pub logs: Vec<(DateTime<Utc>, String)>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub sub_frames: Vec<Span>,
}

impl Clone for Span {
    fn clone(&self) -> Self {
        Span::new(&self.id)
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Span")
            .field("id", &self.id)
            .field("key", &self.key)
            .field("breadcrumbs", &self.breadcrumbs)
            .field("success", &self.success)
            .field("result", &self.result)
            .field("err", &self.err)
            .field("logs", &self.logs)
            .field("start_time", &self.start_time)
            .field("end_time", &self.end_time)
            .field("sub_frames", &self.sub_frames)
            .finish()
    }
}

impl Span {
    pub fn new(id: &str) -> Span {
        Span {
            id: id.to_owned(),
            key: uuid::Uuid::new_v4().to_string(),
            breadcrumbs: HashMap::new(),
            success: None,
            result: None,
            err: None,
            logs: vec![],
            start_time: Utc::now(),
            end_time: None,
            sub_frames: vec![],
        }
    }
    pub(crate) fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn start(&mut self) -> &mut Self {
        self.start_time = Utc::now();
        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.end_time = Some(Utc::now());
        self
    }

    pub fn set_result(&mut self, result: impl serde::Serialize) -> &mut Self {
        self.result = Some(json!(result));
        self
    }

    pub fn set_success(&mut self, is_success: bool) -> &mut Self {
        self.success = Some(is_success);
        self
    }

    pub fn set_err(&mut self, err: Option<String>) -> &mut Self {
        self.err = err;
        self
    }

    pub fn add_sub_frame(&mut self, frame: Span) {
        self.sub_frames.push(frame);
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn add_logs(&mut self, log: &str) {
        self.logs.push((Utc::now(), log.to_string()))
    }

    //adding breadcrumbs
    pub fn add_breadcrumbs(&mut self, name: &str, value: serde_json::Value) {
        self.breadcrumbs.insert(name.to_string(), value);
    }
}
