use api::services::events::JobCalled;
use blueprint_sdk::config::GadgetConfiguration;
use blueprint_sdk::event_listeners::tangle::events::TangleEventListener;
use blueprint_sdk::event_listeners::tangle::services::{
    services_post_processor, services_pre_processor,
};
use blueprint_sdk::macros::contexts::{ServicesContext, TangleClientContext};
use blueprint_sdk::tangle_subxt::tangle_testnet_runtime::api;

use std::convert::Infallible;

#[derive(Clone, TangleClientContext, ServicesContext)]
pub struct ServiceContext {
    #[config]
    pub config: GadgetConfiguration,
    #[call_id]
    pub call_id: Option<u64>,
}

/// Returns "Hello World!" if `who` is `None`, otherwise returns "Hello, {who}!"
#[blueprint_sdk::job(
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
        let config = GadgetConfiguration::default();
        let context = ServiceContext { config, call_id: None };
        let result = say_hello(None, context.clone()).unwrap();
        assert_eq!(result, "Hello World!");
        let result = say_hello(Some("Alice".to_string()), context).unwrap();
        assert_eq!(result, "Hello, Alice!");
    }
}
