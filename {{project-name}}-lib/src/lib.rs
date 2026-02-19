//! {{project-description}}

use blueprint_sdk::Router;
use blueprint_sdk::tangle::extract::{Caller, TangleArg, TangleResult};
use serde::{Deserialize, Serialize};

/// Job ID for the hello job.
pub const HELLO_JOB_ID: u8 = 0;

/// Input payload sent from the Tangle contract.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HelloRequest {
    pub name: String,
}

/// Output payload returned back to the caller.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HelloResponse {
    pub message: String,
    pub operator: String,
}

/// A simple job that greets the caller.
pub async fn hello(
    Caller(caller): Caller,
    TangleArg(request): TangleArg<HelloRequest>,
) -> TangleResult<HelloResponse> {
    let message = format!("Hello, {}!", request.name);

    TangleResult(HelloResponse {
        message,
        operator: caller.to_string(),
    })
}

/// Router that maps job IDs to handlers.
#[must_use]
pub fn router() -> Router {
    Router::new().route(HELLO_JOB_ID, hello)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello() {
        let caller = [0u8; 32].into();
        let request = HelloRequest {
            name: "World".to_string(),
        };

        let result = hello(Caller(caller), TangleArg(request)).await;
        assert!(result.0.message.contains("Hello, World!"));
    }
}
