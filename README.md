![Tangle Network Banner](https://raw.githubusercontent.com/tangle-network/tangle/refs/heads/main/assets/Tangle%20%20Banner.png)

# Hello World Tangle Blueprint

[![Discord](https://img.shields.io/badge/Discord-Join%20Chat-7289da?logo=discord&logoColor=white)](https://discord.gg/cv8EfJu3Tn)
[![Twitter](https://img.shields.io/twitter/follow/tangle_network?style=social)](https://twitter.com/tangle_network)

**Blueprint Template** is a starter template for building verifiable services on Tangle Network. It scaffolds a complete Blueprint project with Router, TangleArg, and x402 payment support out of the box.

## Overview

This Tangle Blueprint provides a simple Hello World job using the Tangle EVM architecture.
Blueprints are specifications for AVS (Actively Validated Services) on the Tangle Network. An AVS is
an off-chain service that runs arbitrary computations for a user-specified period of time.

Blueprints provide a useful abstraction, allowing developers to create reusable service infrastructures as if they were
smart contracts. This enables developers to monetize their work and align long-term incentives with the success of their
creations, benefiting proportionally to their Blueprint's usage.

For more details, please refer to the [project documentation](https://docs.tangle.tools/developers/blueprints/introduction).

## Features

- Tangle EVM-based blueprint architecture
- ABI-compatible request/response types via `alloy-sol-types`
- Custom greeting messages with caller address tracking

## Prerequisites

Before you can run this project, you will need to have the following software installed on your machine:

- [Rust 1.86+](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh) (for smart contract development)

You will also need to install [cargo-tangle](https://crates.io/crates/cargo-tangle), our CLI tool for creating and
deploying Tangle Blueprints:

```bash
cargo install cargo-tangle --git https://github.com/tangle-network/blueprint --branch v2
```

## Getting Started

Once `cargo-tangle` is installed, you can create a new project with the following command:

```sh
cargo tangle blueprint create --name <project-name>
```

and follow the instructions to create a new project.

## Project Structure

```
{{project-name}}/
  Cargo.toml              # Workspace configuration
  {{project-name}}-lib/   # Blueprint library with job definitions
    src/lib.rs            # Job implementation and router
  {{project-name}}-bin/   # Blueprint runner binary
    src/main.rs           # Main entry point
  contracts/              # Solidity smart contracts
```

## Development

Build the project:

```sh
cargo build
```

Run tests:

```sh
cargo test
```

Deploy the blueprint to the Tangle network:

```sh
cargo tangle blueprint deploy tangle --network devnet
```

## Key Concepts

- **Blueprint**: A specification for a verifiable, decentralized service on Tangle Network. Blueprints define jobs, handle results, and manage the operator lifecycle.
- **Operator**: A node runner who registers to provide services defined by a Blueprint. Operators stake assets and earn rewards for honest execution.
- **Router**: The component that maps incoming job calls to handler functions. Routes are defined with typed extractors (`TangleArg`) and return `TangleResult` types.
- **TangleArg**: A typed extractor that deserializes on-chain job call parameters into Rust types for use in handler functions.
- **TangleResult**: The return type for job handlers that serializes Rust values back into on-chain result format.
- **BlueprintRunner**: The runtime that manages the lifecycle of a Blueprint operator, including job polling, execution, and result submission.
- **cargo-tangle**: The CLI tool for creating, deploying, and managing Tangle Blueprints. Run `cargo tangle blueprint create` to scaffold a new project from this template.
- **x402**: An HTTP-native payment protocol (HTTP 402 Payment Required) that enables per-request micropayments for Blueprint services.

## FAQ

### What is this template for?
This template scaffolds a new **Tangle Blueprint** project with the standard directory structure. It includes a Rust library crate for job handlers, a binary runner crate, and Solidity contracts for on-chain registration.

### How do I create a new Blueprint from this template?
Run `cargo tangle blueprint create --name my-blueprint` to generate a new project. The CLI clones this template and replaces placeholder names with your project name throughout the codebase.

### What is the difference between the lib and bin crates?
The **lib crate** defines your job handlers and business logic using `Router`, `TangleArg`, and `TangleResult`. The **bin crate** wires everything together using `BlueprintRunner` and starts the operator service.

### How do I deploy my Blueprint?
Build with `cargo build`, then deploy with `cargo tangle blueprint deploy tangle --network devnet` for testnet or `--network mainnet` for production. The CLI handles contract deployment and on-chain registration.

### Can I add x402 payments to my Blueprint?
Yes. The Blueprint SDK includes `X402Gateway` and `X402Middleware` for adding per-request payment rails. Configure pricing in a TOML file and wire the gateway into your `BlueprintRunner`. See the x402-blueprint example in the main SDK repository.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Feedback and Contributions

We welcome feedback and contributions to improve this blueprint.
Please open an issue or submit a pull request on our GitHub repository.
Please let us know if you fork this blueprint and extend it too!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
