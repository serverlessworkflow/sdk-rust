# Serverless Workflow Rust SDK

The official Rust SDK for the [Serverless Workflow DSL](https://github.com/serverlessworkflow/specification/blob/main/dsl.md).

The SDK is composed of three crates:

- [Core](#), which contains the models of the Serverless Workflow DSL.
- [Builders](#), which contains services to build workflow definitions programmatically.
- [IO](#), which contains services to read and write workflow definitions.

## Installation

Add the desired crate(s) to your `Cargo.toml`:

### Core:

```toml
[dependencies]
serverless-workflow-core = "0.1"
```

### Builders:

```toml
[dependencies]
serverless-workflow-builders = "0.1"
```

### IO:

```toml
[dependencies]
serverless-workflow-io = "0.1"
```

## Features

- **Core Models**: The SDK provides comprehensive support for the [Serverless Workflow DSL](https://github.com/serverlessworkflow/specification/blob/main/dsl.md) models, enabling validation and manipulation of workflow definitions.
- **Builder API**: Build workflow definitions programmatically with a fluent and ergonomic API.
- **IO Services**: Read and write workflow definitions in multiple formats, including YAML and JSON.

## Contributing

Contributions are welcome! Please refer to the [contribution guidelines](./CONTRIBUTING.md) for more information.