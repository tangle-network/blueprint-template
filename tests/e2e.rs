use blueprint_sdk::logging;
use blueprint_sdk::testing::tempfile;
use blueprint_sdk::testing::utils::harness::TestHarness;
use blueprint_sdk::testing::utils::runner::TestEnv;
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

	// Create blueprint-specific context
	let blueprint_ctx = ServiceContext {
		config: env.clone(),
		call_id: None,
	};

	// Initialize event handler
	let handler = SayHelloEventHandler::new(&env.clone(), blueprint_ctx)
		.await
		.unwrap();

	// Setup service
	let (mut test_env, service_id) = harness.setup_services().await?;
	test_env.add_job(handler);

	tokio::spawn(async move {
		test_env.run_runner().await.unwrap();
	});

	// Execute job and verify result
	let job_inputs = vec![to_field("Alice").unwrap()];
	let expected_outputs = vec![to_field("Hello, Alice!").unwrap()];

	let results = harness
		.execute_job(
			service_id,
			0,
			job_inputs,
			expected_outputs,
		)
		.await?;

	assert_eq!(results.service_id, service_id);
	Ok(())
}
