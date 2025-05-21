# Deployment Environment Documentation - RustQuote Service

## 1. Target Deployment Platform (MVP)

As per the [`RustQuote_Service_Project_Plan_v2.md`](RustQuote_Service_Project_Plan_v2.md:150) (Section 6.2), the primary target deployment platform for the Minimum Viable Product (MVP) is **Simple Local Docker**. This involves deploying the application using Docker containers running on a local machine.

While a Basic Platform-as-a-Service (PaaS) was also mentioned as a possibility in the project plan, the immediate focus for MVP documentation and preparation is the local Docker environment. PaaS deployment remains a consideration for future scaling or alternative deployment strategies.

**Current Status & Focus:**
*   The primary goal is to enable and document deployment via a local Docker setup.
*   Task `45` (Containerize Application - Dockerfile), which is essential for local Docker deployment, is currently noted as blocked due to Docker daemon issues. This documentation proceeds with the assumption that these issues will be resolved to allow for local Docker image building and execution.
*   This document will outline the prerequisites and steps for a local Docker deployment.

Information regarding PaaS options is retained in this document for future reference but is secondary to the local Docker MVP approach.

## 2. Prerequisites for Local Docker Deployment

To deploy and run the RustQuote service locally using Docker, the following prerequisites must be met on your development machine:

*   **Docker Installed:**
    *   **Windows/macOS:** Docker Desktop must be installed and running. This provides the Docker Engine, Docker CLI, and other necessary tools.
    *   **Linux:** The Docker Engine and Docker CLI must be installed. Ensure the Docker daemon is running and your user has permissions to interact with it (e.g., by being part of the `docker` group or using `sudo`).
*   **Docker CLI Access:** You need to be able to execute `docker` commands from your terminal.
*   **Application Source Code:** The RustQuote service source code, including the `Dockerfile` (to be created in Task `45`), must be available locally.
*   **(Development Container Specific) Docker Socket Access:** If working within a development container (like a VS Code dev container), ensure it's configured to access the host's Docker daemon. This typically involves:
    *   Mounting the Docker socket (e.g., `/var/run/docker.sock`) into the dev container.
    *   Having the Docker CLI installed *inside* the dev container.
    *   The "Context Update (Docker Access)" in Section 5 (formerly Section 4) notes that this has been addressed for this project.

Meeting these prerequisites will allow you to build the Docker image for the RustQuote service and run it as a container on your local machine.
## 3. Local Docker Network Configuration

This section outlines the network settings required for a local Docker deployment to allow access to the RustQuote application.

*   **Application Port (`CONTAINER_PORT`):** The Rust application within the Docker container should be configured to listen on a specific internal port (e.g., `3000` or `8080`). This is typically defined in the application's code and should be exposed in the `Dockerfile` (Task `45`) using the `EXPOSE` instruction. For consistency, let's assume the application will listen on port `3000` internally.
    *   `Dockerfile` example: `EXPOSE 3000`
*   **Port Mapping (`HOST_PORT`):** When running the Docker container, this internal application port must be mapped to a port on your host machine. This allows you to access the application via `localhost` on your browser or through `curl`.
    *   The `docker run` command uses the `-p` flag for this: `docker run -p <HOST_PORT>:<CONTAINER_PORT> rustquote_service`
    *   **Recommended MVP Mapping:** Map the container's port `3000` to the host's port `3000`.
        *   Example command snippet: `-p 3000:3000`
        *   The application would then be accessible at `http://localhost:3000`.
*   **Host Firewall:** The firewall on your host machine (where Docker is running) must be configured to allow incoming TCP connections on the chosen `HOST_PORT` (e.g., `3000`).
    *   **Docker Desktop (Windows/macOS):** Usually handles this automatically or prompts for permission.
    *   **Linux (using `ufw`):** `sudo ufw allow 3000/tcp` (if `HOST_PORT` is `3000`).
    *   **Windows (Manual):** If needed, add an inbound rule to Windows Defender Firewall for the `HOST_PORT`.
    *   **macOS (Manual):** If the built-in firewall is highly restrictive, specific port opening might be needed.

**Action Required (Post-Dockerfile):**
*   Once the `Dockerfile` (Task `45`) is available and specifies the `EXPOSE`d port (e.g., `3000`), confirm it aligns with this documentation.
*   Ensure the example `docker run` command in Section 6 reflects this port mapping.

---
*(For Future PaaS Consideration: PaaS providers typically manage networking differently, often by reading a `PORT` environment variable and handling external exposure automatically. This section would be expanded if/when PaaS deployment is actively pursued.)*

## 4. Local Docker Environment Variables

This section details the setup of environment variables for the RustQuote application when run via local Docker. For the MVP, complex variables like API keys or database URLs are not anticipated.

*   **`RUST_LOG` (Optional):**
    *   Used to control the logging level of the Rust application (e.g., `info`, `debug`, `warn`, `error`).
    *   This can be set when running the Docker container using the `-e` flag.
    *   Example: `docker run -e RUST_LOG=info rustquote_service ...`
    *   If not set, the application will likely use a default logging level (e.g., `info`) as configured in its `main.rs` or logging setup.
*   **`PORT` (Generally Not Used for MVP Local Docker):**
    *   For the local Docker MVP, the application is expected to listen on a hardcoded internal port (e.g., `3000`, as discussed in Section 3), which is then exposed in the `Dockerfile` and mapped during `docker run`.
    *   While the application *could* be designed to read a `PORT` environment variable, it adds complexity not strictly needed for a simple local Docker deployment. If it were, you would set it via `docker run -e PORT=3000 ...`.
    *   The primary mechanism for port configuration in this MVP context is the `Dockerfile`'s `EXPOSE` instruction and the `docker run -p` mapping.

**Action Required:**
*   **Application Code:** Ensure the application has a sensible default logging level if `RUST_LOG` is not provided. For the port, ensure it listens on the agreed internal port (e.g., `3000`).
*   **Dockerfile (Task `45`):** Confirm the `EXPOSE`d port. No specific `ENV` instructions for `PORT` are expected in the `Dockerfile` for this MVP approach.

---
*(For Future PaaS Consideration: PaaS environments typically require applications to read a `PORT` environment variable injected by the platform. The application code should ideally support this for future flexibility, even if not the primary method for local Docker.)*

## 5. Local Docker Container Image Access & Building

This section covers how the RustQuote application's container image is built and accessed for local Docker deployment.

**Prerequisite Reminder (Docker Access in Dev Container):**
As noted in Section 2 (Prerequisites) and confirmed by previous updates, the development environment (e.g., VS Code dev container) should be configured to use the host's Docker daemon. This typically involves mounting `/var/run/docker.sock` and having the Docker CLI available within the dev container. This setup is crucial for the local build and run commands.

**A. Building the Docker Image Locally:**

*   **`Dockerfile`:** A `Dockerfile` (to be created/finalized in Task `45`) must exist in the root of the RustQuote service project directory. This file contains the instructions to build the application into a Docker image.
*   **Build Command:** To build the image, navigate to the project's root directory (containing the `Dockerfile`) in your terminal and run the `docker build` command.
    *   **Placeholder Command:**
        ```bash
        docker build -t rustquote_service .
        ```
    *   **Explanation:**
        *   `docker build`: The command to build an image from a Dockerfile.
        *   `-t rustquote_service`: Tags the image with the name `rustquote_service` (and implicitly the `latest` tag, so `rustquote_service:latest`). You can use other names/tags like `yourname/rustquote_service:v0.1.0`. For MVP, `rustquote_service` is sufficient.
        *   `.`: Specifies that the build context (including the `Dockerfile`) is the current directory.
*   **Image Availability:** Once successfully built, the image (`rustquote_service:latest`) will reside in your local Docker daemon's image cache. It's then available to be run as a container.

**B. Accessing the Image for `docker run`:**

*   The `docker run` command (detailed in Section 6) will directly use this locally built image by its name and tag.
*   No external container registry (like Docker Hub or GHCR) is strictly required for this local MVP deployment, simplifying the setup.

**Action Required:**
*   **Dockerfile (Task `45`):** Ensure Task `45` results in a functional `Dockerfile` in the project root.
*   **Build Verification:** Once the `Dockerfile` is ready, test the `docker build -t rustquote_service .` command.

---
*(For Future PaaS Consideration: If deploying to a PaaS, the image might be built by the PaaS from a Git repository, or built locally/via CI and pushed to a container registry like Docker Hub or GitHub Container Registry (GHCR). The PaaS would then pull the image from this registry.)*

## 6. Running and Accessing the Application Locally

This section describes how to run the locally built Docker image as a container and how to access the RustQuote application.

**A. Running the Docker Container:**

*   **Prerequisite:** You must have successfully built the Docker image (e.g., `rustquote_service:latest`) as described in Section 5.
*   **Run Command:** To run the container, use the `docker run` command.
    *   **Placeholder Command (Recommended for MVP):**
        ```bash
        docker run -d -p 3000:3000 --name rustquote_app rustquote_service
        ```
    *   **Explanation:**
        *   `docker run`: The command to run a new container from an image.
        *   `-d`: Detached mode. Runs the container in the background and prints the container ID.
        *   `-p 3000:3000`: Port mapping. Maps port `3000` on the host to port `3000` inside the container (assuming the application inside the container listens on `3000`, as per Section 3). Adjust if your internal container port or desired host port is different.
        *   `--name rustquote_app`: Assigns a name (`rustquote_app`) to the running container. This makes it easier to manage (e.g., `docker stop rustquote_app`).
        *   `rustquote_service`: The name of the image to run (implicitly `rustquote_service:latest`).
*   **Verify Container is Running:**
    *   You can check if the container is running using:
        ```bash
        docker ps
        ```
    *   You should see `rustquote_app` listed.
*   **Viewing Logs (if not detached or for troubleshooting):**
    *   If you run without `-d`, logs will stream to your terminal.
    *   To view logs of a detached container:
        ```bash
        docker logs rustquote_app
        ```

**B. Accessing the Locally Deployed Application:**

*   **Access URL:** With the recommended port mapping (`-p 3000:3000`), the application will be accessible at:
    *   `http://localhost:3000`
    *   or `http://127.0.0.1:3000`
*   **Key Endpoints (Examples):**
    *   **Health Check:** `GET http://localhost:3000/api/health` (Note: The project plan implies API routes might be under `/api/`. Adjust if base URL is different.)
    *   **Get a Random Quote:** `GET http://localhost:3000/api/quote` (Adjust path as necessary based on actual API design).
*   **Testing with `curl` or Browser:**
    *   Open a web browser and navigate to `http://localhost:3000/api/health`.
    *   Or, use `curl` in your terminal:
        ```bash
        curl http://localhost:3000/api/health
        curl http://localhost:3000/api/quote
        ```

**C. Stopping and Removing the Container:**

*   **Stop the container:**
    ```bash
    docker stop rustquote_app
    ```
*   **Remove the container (optional, after stopping):**
    ```bash
    docker rm rustquote_app
    ```

**Action Required (Post-Deployment):**
*   Once the application is successfully built and run locally:
    1.  Verify the exact access URL and key API endpoints.
    2.  Test these endpoints.
    3.  Update this section with confirmed details if they differ from these placeholders (especially the API paths like `/api/health` and `/api/quote`).

---
*(For Future PaaS Consideration: Accessing a PaaS-deployed application involves using the public URL provided by the PaaS provider, typically over HTTPS. The PaaS manages port exposure.)*