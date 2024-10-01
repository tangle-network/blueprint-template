use color_eyre::Result;
use gadget_sdk as sdk;
use {{project-name}} as blueprint;
use sdk::{
    config::ContextConfig, events_watcher::substrate::SubstrateEventWatcher,
    events_watcher::tangle::TangleEventsWatcher, tangle_subxt::*,
};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    color_eyre::install()?;

    // Initialize the environment
    let config = ContextConfig::from_args();
    let env = sdk::config::load(config)?;
    let signer = env.first_sr25519_signer()?;
    let client = subxt::OnlineClient::from_url(&env.rpc_endpoint).await?;

    if env.should_run_registration() {
        todo!();
    }

    let service_id = env.service_id.expect("should exist");

    // Create the event handler from the job
    let say_hello_job = blueprint::SayHelloEventHandler { service_id, signer };

    tracing::info!("Starting the event watcher ...");

    SubstrateEventWatcher::run(
        &TangleEventsWatcher {
            span: env.span.clone(),
        },
        client,
        // Add more handler here if we have more functions.
        vec![Box::new(say_hello_job)],
    )
    .await?;
    Ok(())
}

fn init_logger() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_env_filter(env_filter)
        .init();
}
