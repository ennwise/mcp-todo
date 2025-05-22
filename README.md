# RustQuote Service

The RustQuote Service is a web service designed to provide random quotes to users or client applications. It is implemented in Rust to leverage the language's performance and safety features.

## Project Overview

*   **Goal:** Develop a robust and efficient web service for delivering random quotes.
*   **Technology:** Built with Rust, focusing on performance, safety, and modern development practices.
*   **Deployment:** Supports local execution for development and testing, with plans for Docker-based deployment.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

*   **Rust Toolchain:** Ensure you have the latest stable Rust toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).
*   **Editor/IDE:** A code editor or IDE with Rust support (e.g., VS Code with the `rust-analyzer` extension).

### Setup

1.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd rustquote-service
    ```

2.  **Configure Linters and Formatters:**
    The project uses `rustfmt` for code formatting and `clippy` for linting to maintain code quality and consistency.
    *   Ensure `rustfmt` and `clippy` are installed (they usually come with the Rust toolchain):
        ```bash
        rustup component add rustfmt
        rustup component add clippy
        ```
    *   It's recommended to configure your editor to use `rustfmt` on save and run `clippy` checks. Project-specific configurations for these tools might be present in `rustfmt.toml` or `clippy.toml` (if applicable).

### Building the Service

To build the project, navigate to the project root directory and run:

```bash
cargo build
```

For a release build, use:

```bash
cargo build --release
```

### Running Locally

To run the service locally after building, use:

```bash
cargo run
```

The service will start based on the configured server address and port. By default, this is `0.0.0.0:8080`.

### Configuration

The application can be configured using environment variables. For local development, you can create a `.env` file in the project root directory. An example file [` .env.example`](./.env.example:0) is provided.

**Environment Variables:**

*   `RUSTQUOTE_SERVER_ADDRESS`: The address and port for the server to listen on.
    *   Default: `0.0.0.0:8080`
    *   Example: `RUSTQUOTE_SERVER_ADDRESS=127.0.0.1:3000`
*   `RUSTQUOTE_QUOTES_FILE_PATH`: The path to the JSON file containing the quotes.
    *   Default: `data/quotes.json`
    *   Example: `RUSTQUOTE_QUOTES_FILE_PATH=my_custom_quotes.json`

To use a `.env` file for local development:

1.  Copy [` .env.example`](./.env.example:0) to `.env`:
    ```bash
    cp .env.example .env
    ```
2.  Modify the values in `.env` as needed. The application will automatically load these variables when it starts.

## API Endpoints

The service exposes the following API endpoints:

*   **Get a Random Quote**
    *   **Endpoint:** `GET http://localhost:31337/quote`
    *   **Description:** Retrieves a random quote.
    *   **Success Response (200 OK):**
        ```json
        {
          "id": "unique-quote-id-123",
          "quote": "An inspiring quote text.",
          "author": "The Author"
        }
        ```
*   **Get a Specific Quote by ID**
    *   **Endpoint:** `GET http://localhost:31337/quote/{id}`
    *   **Description:** Retrieves a specific quote by its unique ID.
    *   **Success Response (200 OK):**
        ```json
        {
          "id": "unique-quote-id-123",
          "quote": "An inspiring quote text.",
          "author": "The Author"
        }
        ```
    *   **Error Response (404 Not Found):**
        ```json
        {
          "error": "Quote not found"
        }
        ```
*   **Health Check**
    *   **Endpoint:** `GET http://localhost:31337/health`
    *   **Description:** Returns the health status of the service.
    *   **Success Response (200 OK):**
        ```json
        {
          "status": "UP"
        }
        ```
        *(Note: The exact health check response structure might vary; refer to implementation or specific tests.)*


## Deployment (MVP)

For the MVP, deployment focuses on running the application locally using Docker.

### Using the Deployment Script

A deployment script [`scripts/deploy.sh`](scripts/deploy.sh:0) is provided to simplify building the Docker image and running the container locally.

1.  **Navigate to the project root directory.**
2.  **Make the script executable (if you haven't already):**
    ```bash
    chmod +x scripts/deploy.sh
    ```
3.  **Run the script:**
    ```bash
    ./scripts/deploy.sh
    ```
    This script will:
    *   Build the Docker image named `rustquote-service:latest` using [`rust_quote_service/Dockerfile`](rust_quote_service/Dockerfile:0).
    *   Stop and remove any existing container named `rustquote-container`.
    *   Run a new container named `rustquote-container` from the built image, exposing port `8080` on the host.

    The application should then be accessible at `http://localhost:8080`.
    Logs can be viewed using `docker logs rustquote-container`.
    The container can be stopped using `docker stop rustquote-container`.

### Manual Docker Build and Run

Alternatively, you can build and run the Docker image manually:

1.  **Build the Docker Image:**
    Navigate to the project root directory. The Dockerfile is located at [`rust_quote_service/Dockerfile`](rust_quote_service/Dockerfile:0) and the build context is [`rust_quote_service/`](rust_quote_service/:0).
    ```bash
    docker build -t rustquote-service:latest -f rust_quote_service/Dockerfile ./rust_quote_service
    ```

2.  **Run the Docker Container:**
    ```bash
    docker run -d -p 8080:8080 --name rustquote-container rustquote-service:latest
    ```
    *   This command runs the container in detached mode (`-d`).
    *   It maps port `8080` on the host to port `8080` in the container.
    *   The container is named `rustquote-container`.

---

## CI Pipeline

A Continuous Integration (CI) pipeline is set up using GitHub Actions. The workflow is defined in [`.github/workflows/rust.yml`](.github/workflows/rust.yml:0).

The CI pipeline includes the following jobs and key steps:

**1. `build_and_test` Job:**
    *   Checks out the code.
    *   Sets up the Rust stable toolchain.
    *   Builds the project (`cargo build --verbose`).
    *   Runs linters (`cargo clippy -- -D warnings`).
    *   Checks formatting (`cargo fmt --check`).
    *   Runs tests (`cargo test --verbose`).
    *   Generates a code coverage report using `cargo-tarpaulin`.
    *   Uploads the coverage report as an artifact.

**2. `build_docker_image` Job:**
    *   Depends on the successful completion of the `build_and_test` job.
    *   Checks out the code.
    *   Sets up Docker Buildx.
    *   Builds the Docker image using the [`rust_quote_service/Dockerfile`](rust_quote_service/Dockerfile:0) and tags it as `rustquote-service:latest`.
    *   For the MVP, this job does *not* push the image to a container registry.

The pipeline triggers on:
- Pushes to the `main` branch.
- Pull requests targeting the `main` branch.

You can check the CI status for your commits and pull requests in the "Actions" tab of the GitHub repository.

## API Endpoint Testing

API endpoint testing is conducted as part of the integration test suite located in [`tests/integration_tests.rs`](tests/integration_tests.rs:0). These tests ensure the reliability and correctness of the API endpoints.

The tests cover the following aspects:
- **Successful responses:** Verifying correct status codes (e.g., `200 OK`) and payload structure for valid requests to endpoints like `GET /quote` (or `GET /api/v1/quote` as per test implementation) and `GET /health`.
- **Error handling:** Ensuring appropriate error responses (e.g., `404 Not Found`, `500 Internal Server Error`) and informative error messages for various failure scenarios, such as:
    - Non-existent data files.
    - Empty data files.
    - Malformed or invalid JSON data in files.
    - Individual data entries with missing or corrupt fields.
- **HTTP methods:** Currently, `GET` requests are tested as per the API design.
- **Response payloads:** Validation of JSON response structures and data types against expected formats.
- **Edge cases:** Testing for conditions like empty data sources or corrupted data.

These tests are automatically executed by the CI pipeline via the `cargo test` command, as defined in the [`.github/workflows/rust.yml`](.github/workflows/rust.yml:0) workflow.

## Code Coverage

Code coverage for this project is tracked using [`cargo-tarpaulin`](https://crates.io/crates/cargo-tarpaulin). It helps in understanding how much of the codebase is covered by the automated tests.

### Generating Reports Locally

To generate a code coverage report locally, you first need to install `cargo-tarpaulin` if you haven't already:

```bash
cargo install cargo-tarpaulin
```

Then, run the following command from the root of the project:

```bash
cargo tarpaulin --out Html --output-dir ./coverage
```

This will generate an HTML report in the `coverage` directory. You can open the `index.html` file in that directory with a web browser to view the report. Other output formats like LCOV can also be generated by changing the `--out` parameter (e.g., `--out Lcov`).

### CI Pipeline Integration

The CI pipeline, defined in [`.github/workflows/rust.yml`](.github/workflows/rust.yml:1), automatically generates an LCOV code coverage report after the tests are run.
The generated report (`lcov.info`) is then uploaded as an artifact named `coverage-report`. You can download this report from the "Summary" page of the specific workflow run in the "Actions" tab of the GitHub repository. This LCOV report can be used with services like Codecov or Coveralls for more detailed coverage analysis and tracking over time.