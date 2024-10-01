use gadget_sdk as sdk;
use sdk::job;
use std::convert::Infallible;

/// Returns "Hello World!" if `who` is `None`, otherwise returns "Hello, <who>!"
#[job(id = 0, params(who), result(_), verifier(evm = "HelloBlueprint"))]
pub fn say_hello(who: Option<String>) -> Result<String, Infallible> {
    match who {
        Some(who) => Ok(format!("Hello, {}!", who)),
        None => Ok("Hello World!".to_string()),
    }
}
