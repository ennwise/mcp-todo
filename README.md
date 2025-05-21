# RustQuote Service

This is a placeholder README for the RustQuote Service.

## CI Pipeline

A Continuous Integration (CI) pipeline is set up using GitHub Actions. The workflow is defined in [`.github/workflows/rust.yml`](.github/workflows/rust.yml:0).

The CI pipeline includes the following steps:
- Checkout code
- Cache Cargo dependencies
- Run `cargo check`
- Run `cargo build --verbose`
- Run `cargo test --verbose`
- Run `cargo clippy -- -D warnings`

The pipeline triggers on:
- Pushes to `main` and `develop` branches
- Pull requests targeting `main` and `develop` branches

You can check the CI status for your commits and pull requests in the "Actions" tab of the GitHub repository.