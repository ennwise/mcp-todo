# Deployment Environment Documentation - RustQuote Service

## 1. Target Deployment Platform (MVP)

As per the [`RustQuote_Service_Project_Plan_v2.md`](RustQuote_Service_Project_Plan_v2.md:150) (Section 6.2), the target deployment platform for the Minimum Viable Product (MVP) is one of the following:

*   **Simple Local Docker:** Deployment using Docker containers running on a local machine.
*   **Basic Platform-as-a-Service (PaaS):** Deployment to a simple PaaS provider.

**Current Considerations:**
*   Task `45` (Containerize Application - Dockerfile) is currently blocked due to Docker daemon issues in the development environment.
*   If the Docker daemon issue persists, focusing on a basic PaaS for the initial deployment might be more feasible as it may not strictly require a locally built Docker image immediately (some PaaS providers can build from source or have alternative deployment methods).
*   If local Docker is chosen and the daemon issue is resolved, the application will be packaged into a Docker image and run locally.

Further decisions on the specific PaaS provider (if that route is taken) will be documented here.

## 2. PaaS/Cloud Provider Account and Project Setup (Todo 2)

This step involves setting up an account and a new project/service on the chosen PaaS provider. As an AI, this action cannot be performed directly.

**Considerations for PaaS Selection (if pursued):**

For the RustQuote MVP, a PaaS offering the following would be beneficial:
*   **Ease of Use:** Simple deployment process.
*   **Rust Support:** Either native buildpacks for Rust or easy Docker container deployment.
*   **Free Tier:** Sufficient for an MVP to avoid initial costs.
*   **Scalability:** Potential to scale if the project grows beyond MVP (though less critical for initial setup).

**Recommended PaaS Candidates for MVP:**
*   **Fly.io:** Good Rust support, Dockerfile deployments, free tier.
*   **Render:** Deployment from Git (builds on platform) or Docker images, free tier.
*   **Railway.app:** Similar to Render, Git or Docker deployment, usage-based free tier.

**Action Required (Manual Step):**
If proceeding with a PaaS:
1.  Select a PaaS provider (e.g., Fly.io, Render).
2.  Create an account on the chosen platform.
3.  Create a new project/service specifically for the RustQuote application.
4.  Document the chosen PaaS and project details (e.g., project ID, service name) in this document once completed.

## 3. Network Configuration (Todo 3)

This section outlines the network settings required for the deployment environment to allow access to the RustQuote application. The specifics depend on whether a local Docker setup or a PaaS is used.

**A. Local Docker Deployment:**

*   **Application Port:** The Rust application within the Docker container should be configured to listen on a specific port (e.g., `8080`). This is typically defined in the application's code and exposed in the `Dockerfile` using the `EXPOSE` instruction.
*   **Port Mapping:** When running the Docker container, the internal application port must be mapped to a port on the host machine.
    *   Example: `docker run -p <host_port>:<container_port> rustquote_image`
    *   If the application listens on `8080` inside the container, you might map it to `8000` on the host: `docker run -p 8000:8080 rustquote_image`. The application would then be accessible at `http://localhost:8000`.
*   **Host Firewall:** The firewall on the host machine (where Docker is running) must be configured to allow incoming TCP connections on the chosen `<host_port>`.
    *   Linux (using `ufw`): `sudo ufw allow <host_port>/tcp`
    *   Windows: Add an inbound rule to Windows Defender Firewall for the `<host_port>`.
    *   macOS: The built-in firewall usually allows connections to signed software by default; specific port opening might be needed if it's highly restrictive.

**B. PaaS Deployment:**

*   **Application Port:** The Rust application should be configured to listen on the port specified by the PaaS provider. This is commonly done by reading the `PORT` environment variable.
    *   Example (Rust/Axum):
        ```rust
        // main.rs
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let addr = format!("0.0.0.0:{}", port);
        axum::Server::bind(&addr.parse().unwrap())
            // ...
        ```
*   **Port Exposure & Mapping:** PaaS providers typically handle external port exposure (usually port `80` for HTTP and `443` for HTTPS, often with automatic SSL/TLS) and map it to the internal port your application listens on (as dictated by the `PORT` environment variable or Dockerfile `EXPOSE`).
*   **Firewall Management:** Firewall rules are generally managed by the PaaS provider, ensuring the application is accessible from the internet. Advanced network policies (e.g., IP whitelisting) might be available depending on the PaaS and service plan. No manual firewall configuration is usually needed on the application side for basic access.

**Action Required:**
*   **For Local Docker:** Once the `Dockerfile` is available (Task `45`), document the `EXPOSE`d port and the recommended `docker run` port mapping command. Document the steps for host firewall configuration.
*   **For PaaS:** Ensure the application code is prepared to listen on the `PORT` environment variable. Document the specific PaaS provider's networking details once chosen and the service is provisioned.

## 4. Environment Variables (Todo 4)

This section details the setup of required environment variables for the deployed RustQuote application. For the MVP, complex variables like API keys or database URLs are not applicable.

**A. Local Docker Deployment:**

*   **`PORT` (Optional):**
    *   The application can be hardcoded to listen on a specific port (e.g., `8080`), which is then exposed in the `Dockerfile` (Task `45`) and mapped during `docker run`. This is acceptable for MVP simplicity.
    *   Alternatively, if the application is designed to read a `PORT` environment variable, it can be set during `docker run`:
        *   Example: `docker run -e PORT=8080 rustquote_image ...`
*   **`RUST_LOG` (Optional):**
    *   To control logging levels (e.g., `info`, `debug`).
    *   Example: `docker run -e RUST_LOG=info rustquote_image ...`
    *   This can also be configured within the application's logging setup.

**B. PaaS Deployment:**

*   **`PORT` (Required):**
    *   PaaS providers typically inject a `PORT` environment variable that the application **must** listen on. The application code (e.g., in `main.rs`) needs to read this variable to bind the server correctly.
    *   Refer to the example in Section 3.B for reading `PORT` in Rust.
*   **`RUST_LOG` (Optional):**
    *   Can usually be set via the PaaS provider's dashboard or CLI to control application log verbosity (e.g., `RUST_LOG=info`).
*   **Other PaaS-specific variables:** Some PaaS providers might inject other standard environment variables. Consult the chosen provider's documentation.

**Action Required:**
*   **Application Code:** Ensure the Rust application can correctly read the `PORT` environment variable, especially if targeting PaaS deployment. Provide a sensible default if `PORT` is not set (e.g., for local development without Docker).
*   **Dockerfile (Task `45`):** If `PORT` is read from the environment, the `Dockerfile` might not need to `EXPOSE` a hardcoded port, or it can `EXPOSE` the default port.
*   **PaaS Configuration:** When deploying to a PaaS, configure any necessary environment variables (like `RUST_LOG`) through the provider's interface.

## 5. Container Image Access (Todo 5)

This section covers how the deployment environment will access the RustQuote application's container image. The approach depends on the chosen deployment platform and the resolution of Docker daemon access.

**Context Update (Docker Access):**
Information has been provided to unblock Docker usage within the dev container by mounting the host's Docker socket (`/var/run/docker.sock`) into the dev container via `devcontainer.json` and ensuring `docker-cli` is installed in the dev container. This makes local image building (Task `45`) and local Docker deployment more feasible.

**A. Local Docker Deployment:**

*   **Image Source:** The Docker image will be built locally using the `Dockerfile` created in Task `45` (e.g., `docker build -t rustquote_image .`).
*   **Image Availability:** Once built, the image resides in the local Docker daemon's image cache on the host machine, accessible by the Docker CLI running within the dev container (due to the socket mount).
*   **Access Method:** The `docker run` command will directly use the locally available image by its name and tag (e.g., `docker run rustquote_image:latest`). No external registry is strictly needed for this scenario.

**B. PaaS Deployment:**

*   **Image Source Options:**
    1.  **Build from Git Repository (Recommended for Simplicity/CI):**
        *   Many PaaS providers (e.g., Render, Fly.io, Railway.app) can connect to a Git repository (e.g., GitHub).
        *   They pull the source code, including the `Dockerfile`, and build the image on their infrastructure.
        *   **Access:** Requires granting the PaaS read access to the Git repository.
    2.  **Pull from a Container Registry:**
        *   The image is first built (either locally once Docker is unblocked, or by a CI/CD pipeline) and then pushed to a container registry.
        *   **Suitable Registries for MVP:**
            *   **Docker Hub:** Public repositories are free. Private repositories require a paid plan.
            *   **GitHub Container Registry (GHCR):** Free for public and private repositories associated with GitHub accounts/organizations (subject to storage/transfer limits). This is a strong candidate if the project's source code is hosted on GitHub.
            *   Other cloud provider registries (AWS ECR, Google Artifact Registry, Azure Container Registry) are also options but might be overkill for the MVP unless already in use.
        *   **Access:** The PaaS needs credentials (e.g., access token, username/password) to pull the image from the chosen registry, especially if it's private. These are configured as secrets in the PaaS project.

**Action Required:**
*   **Dev Environment:** Implement the Docker socket mounting solution in `devcontainer.json` and ensure `docker-cli` is available in the dev container to unblock local Docker operations (relates to Task `45`'s blocker).
*   **If Local Docker Deployment:** Proceed with local image building (Task `45`) once Docker is unblocked.
*   **If PaaS Deployment:**
    *   Decide on the image sourcing strategy: build from Git or pull from a registry.
    *   If building from Git: Configure the PaaS to connect to the project's Git repository.
    *   If pulling from registry:
        1.  Choose a container registry (GHCR is recommended if using GitHub).
        2.  Set up the registry and push the image (likely as part of Task `45` or a subsequent CI task).
        3.  Configure the PaaS with access credentials for the chosen registry.
*   Document the chosen image access method and any relevant registry URLs or setup steps here.

## 6. Accessing the Deployed Application (Todo 6)

This section documents how to access the RustQuote application once it's deployed.

**A. Local Docker Deployment:**

*   **Access URL:** The application will be accessible via `http://localhost:<host_port>` or `http://127.0.0.1:<host_port>`.
    *   `<host_port>` is the port on the host machine that the container's application port is mapped to during the `docker run` command (e.g., if container port `8080` is mapped to host port `8000`, use `http://localhost:8000`).
*   **Key Endpoints:**
    *   Get a random quote: `GET http://localhost:<host_port>/quote`
    *   Health check: `GET http://localhost:<host_port>/health`
*   **Example (assuming host port `8000`):**
    *   Quote: `http://localhost:8000/quote`
    *   Health: `http://localhost:8000/health`
    *   You can test these in a web browser or using a tool like `curl`:
        ```bash
        curl http://localhost:8000/quote
        curl http://localhost:8000/health
        ```

**B. PaaS Deployment:**

*   **Access URL:** The PaaS provider will assign a unique public URL to the deployed service (e.g., `https://rustquote-service-alpha.onrender.com`, `https://my-rustquote-app.fly.dev`). This URL will typically use HTTPS.
    *   This URL will be available in the PaaS provider's dashboard for the deployed service.
*   **Port:** Standard HTTPS port (`443`) is generally used and managed by the PaaS. You do not need to specify a port in the URL.
*   **Key Endpoints:**
    *   Get a random quote: `GET https://<assigned_paas_url>/quote`
    *   Health check: `GET https://<assigned_paas_url>/health`
*   **Example (placeholder URL):**
    *   Quote: `https://your-app-name.paas-provider.com/quote`
    *   Health: `https://your-app-name.paas-provider.com/health`
    *   Test using a web browser or `curl`:
        ```bash
        curl https://your-app-name.paas-provider.com/quote
        curl https://your-app-name.paas-provider.com/health
        ```

**Action Required:**
*   Once the application is successfully deployed (either locally via Docker or on a PaaS):
    1.  Verify the exact access URL and port (if applicable).
    2.  Test the `/quote` and `/health` endpoints.
    3.  Update this section with the confirmed access details, replacing placeholders.