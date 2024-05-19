use std::collections::HashMap;

use axum::{extract::Query, response::AppendHeaders, Json};
use nidrs::{version, Inject, Meta};
use nidrs_macro::{controller, get, meta, post};

use crate::AppResult;

use super::{dto::Status, service::AppService};

// #[uses(LogInterceptor)]
#[version("v1")]
#[meta(role = "admin", auth = "true")]
#[meta(test = true)]
// #[meta(disable_default_prefix)]
#[controller()]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[meta(arr = ["user"])]
    // #[uses(LogInterceptor)]
    #[version("v2")]
    #[get("/hello")]
    pub async fn get_hello_world(
        &self,
        meta: Meta,
        Query(q): Query<HashMap<String, String>>,
    ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
        println!("Query {:?}", q);
        println!("Meta {:?}", meta.get::<&str>("role"));
        // fn_test()?;

        Ok((
            AppendHeaders([("X-Custom-Header".to_string(), "hello".to_string()), ("X-Custom-Header".to_string(), "world".to_string())]),
            Status { db: "ok".to_string(), redis: "ok".to_string() },
        ))
    }

    // #[uses(LogInterceptor)]
    #[get("/hello2")]
    pub async fn get_hello_world2(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        Ok(self.app_service.get_hello_world())
    }

    // #[uses(LogInterceptor)]
    #[post("/hello")]
    pub async fn post_hello_world(&self, Query(q): Query<HashMap<String, String>>, Json(j): Json<serde_json::Value>) -> AppResult<String> {
        println!("Query {:?}", q);
        println!("Json {:?}", j);

        Ok("Hello, World2!".to_string())
    }
}
