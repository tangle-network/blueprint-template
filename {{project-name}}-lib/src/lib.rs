//! {{project-description}}

use blueprint_sdk::alloy::primitives::Address;
use blueprint_sdk::alloy::sol;
use blueprint_sdk::Router;
use blueprint_sdk::tangle::extract::{Caller, TangleArg, TangleResult};

/// Job ID for the hello job.
pub const HELLO_JOB_ID: u8 = 0;

// Define ABI-compatible structs for on-chain interaction.
sol! {
    /// Input payload sent from the Tangle contract.
    struct HelloRequest {
        string name;
    }

    /// Output payload returned back to the caller.
    struct HelloResponse {
        string message;
        string operator;
    }
}

/// A simple job that greets the caller.
pub async fn hello(
    Caller(caller): Caller,
    TangleArg(request): TangleArg<HelloRequest>,
) -> TangleResult<HelloResponse> {
    let caller_address = Address::from_slice(&caller);
    let message = format!("Hello, {}!", request.name);

    TangleResult(HelloResponse {
        message,
        operator: format!("{caller_address:#x}"),
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
        let caller = [0u8; 20];
        let request = HelloRequest {
            name: "World".to_string(),
        };

        let result = hello(Caller(caller), TangleArg(request)).await;
        assert!(result.0.message.contains("Hello, World!"));
    }
}
