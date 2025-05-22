#!/bin/bash
set -e

IMAGE_NAME="rustquote-service"
IMAGE_TAG="latest"
CONTAINER_NAME="rustquote-container"
DOCKERFILE_PATH="./Dockerfile"
BUILD_CONTEXT="."

echo "Building Docker image ${IMAGE_NAME}:${IMAGE_TAG}..."
# Ensure the script is run from the root of the project for correct paths
docker build -t "${IMAGE_NAME}:${IMAGE_TAG}" -f "${DOCKERFILE_PATH}" "${BUILD_CONTEXT}"

echo "Starting Docker Compose services..."
docker-compose up -d

echo ""
echo "Deployment script finished."
echo "Application should be accessible at http://localhost:31337"
echo "To view logs: docker-compose logs -f"
echo "To stop the services: docker-compose down"
echo "To make this script executable, run: chmod +x scripts/deploy.sh"