//! {{project-description}}

use alloy_sol_types::sol;
use blueprint_sdk::Router;
use blueprint_sdk::macros::debug_job;
use blueprint_sdk::tangle::extract::{Caller, TangleArg, TangleResult};
use std::fmt::Write as _;

/// Job ID for the hello job.
pub const HELLO_JOB_ID: u8 = 0;

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
#[debug_job]
pub async fn hello(
    Caller(caller): Caller,
    TangleArg(request): TangleArg<HelloRequest>,
) -> TangleResult<HelloResponse> {
    let message = format!("Hello, {}!", request.name);

    TangleResult(HelloResponse {
        message,
        operator: format_caller(&caller),
    })
}

fn format_caller(caller: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(42);
    encoded.push_str("0x");
    for byte in caller {
        write!(&mut encoded, "{byte:02x}").expect("writing to a String cannot fail");
    }
    encoded
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
        let caller = [0u8; 20].into();
        let request = HelloRequest {
            name: "World".to_string(),
        };

        let result = hello(Caller(caller), TangleArg(request)).await;
        assert!(result.0.message.contains("Hello, World!"));
        assert_eq!(
            result.0.operator,
            "0x0000000000000000000000000000000000000000"
        );
    }
}
