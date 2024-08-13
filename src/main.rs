use gadget_sdk as sdk;
use std::convert::Infallible;

use color_eyre::{eyre::OptionExt, Result};
use sdk::{
    events_watcher::tangle::TangleEventsWatcher, events_watcher::SubstrateEventWatcher,
    keystore::backend::GenericKeyStore, keystore::Backend, tangle_subxt::*,
};

/// Returns "Hello World!" if `who` is `None`, otherwise returns "Hello, <who>!"
#[sdk::job(id = 0, params(who), result(_), verifier(evm = "HelloBlueprint"))]
pub fn say_hello(who: Option<String>) -> Result<String, Infallible> {
    match who {
        Some(who) => Ok(format!("Hello, {}!", who)),
        None => Ok("Hello World!".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize the logger
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_env_filter(env_filter)
        .init();

    // Initialize the environment
    let env = sdk::env::load()?;
    let keystore = env.keystore()?;
    let signer = extract_signer_from_keystore(&keystore)?;
    let client = subxt::OnlineClient::from_url(&env.tangle_rpc_endpoint).await?;

    // Create the event handler from the job
    let say_hello_job = SayHelloEventHandler {
        service_id: env.service_id,
        signer,
    };

    tracing::info!("Starting the event watcher ...");

    SubstrateEventWatcher::run(
        &TangleEventsWatcher,
        client,
        // Add more handler here if we have more functions.
        vec![Box::new(say_hello_job)],
    )
    .await?;
    Ok(())
}

fn extract_signer_from_keystore(
    keystore: &GenericKeyStore,
) -> Result<subxt_signer::sr25519::Keypair> {
    let sr25519_pubkey = keystore
        .iter_sr25519()
        .next()
        .ok_or_eyre("No sr25519 keys found in the keystore")?;
    let sr25519_secret = keystore
        .expose_sr25519_secret(&sr25519_pubkey)?
        .ok_or_eyre("No sr25519 secret found in the keystore")?;

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&sr25519_secret.to_bytes()[0..32]);
    subxt_signer::sr25519::Keypair::from_secret_key(seed).map_err(Into::into)
}
