#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;

use http::StatusCode;
use lambda_http::{lambda, Body, IntoResponse, Request, Response};
use lambda_runtime::{error::HandlerError, Context};
use std::error::Error;
mod domain;


#[derive(Deserialize, Serialize, Debug)]
struct MicroFrontendSerializer {
    name: String,
    version: Option<String>,
}


impl MicroFrontendSerializer {
    pub fn new(entity: domain::MicroFrontend) -> MicroFrontendSerializer {
        MicroFrontendSerializer {
            name: entity.name,
            version: Some(entity.version),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // call the AWS lambda macro: give it our routing function
    lambda!(router);
    // exit once request has been served
    Ok(())
}

// Call handler function based on request method
fn router(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => list_micro_frontends_endpoint(req, c),
        _ => not_allowed(req, c),
    }
}

fn list_micro_frontends_endpoint(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // call the action "list_micro_frontends" and serialize returned objects
    // to JSON api response
    let micro_frontends_vector = domain::list_micro_frontends();
    let serialied_entities:Vec<MicroFrontendSerializer> = micro_frontends_vector.iter().map(|micro_frontend_entity| MicroFrontendSerializer::new(micro_frontend_entity)).collect();
    Ok(serde_json::json!(serialied_entities).into_response())
} 

fn not_allowed(_req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // Return 405 Not allowed response
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from(()))
        .expect("err creating response"))
}
