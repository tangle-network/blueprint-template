use blueprint_sdk::logging;
use blueprint_sdk::testing::tempfile;
use blueprint_sdk::testing::utils::harness::TestHarness;
use blueprint_sdk::testing::utils::tangle::blueprint_serde::to_field;
use blueprint_sdk::testing::utils::tangle::TangleTestHarness;
use blueprint_sdk::tokio;
use {{project-name | snake_case}}::{SayHelloEventHandler, ServiceContext};

#[tokio::test]
async fn test_blueprint() -> color_eyre::Result<()> {
	logging::setup_log();

	// Initialize test harness (node, keys, deployment)
	let temp_dir = tempfile::TempDir::new()?;
	let harness = TangleTestHarness::setup(temp_dir).await?;
	let env = harness.env().clone();

	// Setup service
	let (mut test_env, service_id, _) = harness.setup_services::<1>(false).await?;
	test_env.initialize().await?;

	let handles = test_env.node_handles().await;
	for handle in handles {
		let config = handle.gadget_config().await;
		// Create blueprint-specific context
		let blueprint_ctx = ServiceContext {
			config: config.clone(),
			call_id: None,
		};

		// Initialize event handler
		let handler = SayHelloEventHandler::new(&config, blueprint_ctx)
			.await
			.unwrap();

		handle.add_job(handler).await;
	}

	test_env.start().await?;

	// Execute job and verify result
	let job_inputs = vec![to_field("Alice").unwrap()];
	let expected_outputs = vec![to_field("Hello, Alice!").unwrap()];

	let job = harness.submit_job(service_id, 0, job_inputs).await?;

	let results = harness.wait_for_job_execution(service_id, job).await?;

	assert_eq!(results.service_id, service_id);
	assert_eq!(results.result, expected_outputs);
	Ok(())
}
