extern crate rocket;


use rocket::{catch, catchers, get, build, routes};
use rocket::tokio::time::{sleep, Duration};
use rocket::serde::json::{json, Value};

use std::path::PathBuf;
use std::collections::HashMap;

#[catch(default)]
fn default_catcher() -> Value {
    json!({"error": "An error occurred"})
}

#[catch(404)]
fn notfound_catcher() -> Value {
    json!({"error": "Not found"})
}

#[catch(401)]
fn unauthorized_catcher() -> Value {
    json!({"error": "Unauthorized"})
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> Value {
    // Return error if seconds > 5
    if seconds < 5 {
        sleep(Duration::from_secs(seconds)).await;
        json!({"message": format!("Delay completed, waited for {} seconds", seconds)})
    }
    else {
        json!({"error": "Delay time is too long", "details": "Seconds must be less than 5"})
    }
    
}

#[get("/display-info/<path..>?<query..>")]
fn display_info(path: PathBuf, query: HashMap<String, String>) -> Value {
    json!({"request_path": path, "query_params": query})
}

#[get("/")]
fn index() -> Value {
    json!({ 
        "routes": [
            "/",
            "/delay/<seconds>",
            "/display-info/<path..>?<query..>"
        ],
        "catchers": [
            "404 - Not found",
            "401 - Unauthorized",
            "default - An error occurred"
        ]
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = build()
        .register("/", catchers![default_catcher, notfound_catcher, unauthorized_catcher])
        .mount("/", routes![index, delay, display_info])
        .launch()
        .await?;

    Ok(())
}