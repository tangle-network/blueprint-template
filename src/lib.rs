use gadget_sdk as sdk;
use gadget_sdk::tangle_subxt::tangle_testnet_runtime::api;
use std::convert::Infallible;

use api::services::events::JobCalled;
use sdk::event_listener::tangle::{
    jobs::{services_post_processor, services_pre_processor},
    TangleEventListener,
};

#[derive(Clone)]
pub struct ServiceContext {
    pub config: sdk::config::StdGadgetConfiguration,
}

/// Returns "Hello World!" if `who` is `None`, otherwise returns "Hello, {who}!"
#[sdk::job(
    id = 0,
    params(who),
    result(_),
    event_listener(
        listener = TangleEventListener::<ServiceContext, JobCalled>,
        pre_processor = services_pre_processor,
        post_processor = services_post_processor,
    ),
)]
pub fn say_hello(who: Option<String>, context: ServiceContext) -> Result<String, Infallible> {
    match who {
        Some(who) => Ok(format!("Hello, {who}!")),
        None => Ok("Hello World!".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = sdk::config::StdGadgetConfiguration::default();
        let context = ServiceContext { config };
        let result = say_hello(None, context).unwrap();
        assert_eq!(result, "Hello World!");
        let result = say_hello(Some("Alice".to_string()), context).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }
}
