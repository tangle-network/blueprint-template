# Hello World Tangle Blueprint

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
