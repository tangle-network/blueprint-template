use blueprint_sdk::logging;
use blueprint_sdk::runners::core::runner::BlueprintRunner;
use blueprint_sdk::runners::tangle::tangle::TangleConfig;
use {{project-name | snake_case}} as blueprint;

#[blueprint_sdk::main(env)]
async fn main() {
    // Create your service context
    // Here you can pass any configuration or context that your service needs.
    let context = blueprint::ServiceContext {
        config: env.clone(),
        call_id: None,
    };

    // Create the event handler from the job
    let say_hello_job = blueprint::SayHelloEventHandler::new(&env, context).await?;

    logging::info!("Starting the event watcher ...");
    let tangle_config = TangleConfig::default();
    BlueprintRunner::new(tangle_config, env)
        .job(say_hello_job)
        .run()
        .await?;

    logging::info!("Exiting...");
    Ok(())
}

