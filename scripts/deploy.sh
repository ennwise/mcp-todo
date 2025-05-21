#!/bin/bash
set -e

IMAGE_NAME="rustquote-service"
IMAGE_TAG="latest"
CONTAINER_NAME="rustquote-container"
DOCKERFILE_PATH="./rust_quote_service/Dockerfile"
BUILD_CONTEXT="./rust_quote_service"

echo "Building Docker image ${IMAGE_NAME}:${IMAGE_TAG}..."
# Ensure the script is run from the root of the project for correct paths
docker build -t "${IMAGE_NAME}:${IMAGE_TAG}" -f "${DOCKERFILE_PATH}" "${BUILD_CONTEXT}"

echo "Stopping and removing existing container ${CONTAINER_NAME} if it exists..."
docker stop "${CONTAINER_NAME}" >/dev/null 2>&1 || true
docker rm "${CONTAINER_NAME}" >/dev/null 2>&1 || true

echo "Running Docker container ${CONTAINER_NAME}..."
docker run -d -p 8080:8080 --name "${CONTAINER_NAME}" "${IMAGE_NAME}:${IMAGE_TAG}"

echo ""
echo "Deployment script finished."
echo "Application should be accessible at http://localhost:8080"
echo "To view logs: docker logs ${CONTAINER_NAME}"
echo "To stop the container: docker stop ${CONTAINER_NAME}"
echo "To make this script executable, run: chmod +x scripts/deploy.sh"