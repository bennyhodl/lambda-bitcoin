use lambda_http::{service_fn, Body, Error, Request, RequestExt, Response};
use no_node_bitcoin::address::is_address_valid;

async fn check_address_valid(event: Request) -> Result<Response<Body>, Error> {
    let req = event.query_string_parameters();

    let address = match req.first("address") {
        Some(name) => name,
        None => "not a valid address",
    };

    let is_valid = is_address_valid(address);

    let body = format!("Address is valid: {}", is_valid);

    let res = Response::builder()
        .status(200)
        .body(body.into())
        .map_err(Box::new)?;

    Ok(res)
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(|event: Request| check_address_valid(event))).await
}
