# RustQuote Service API Endpoints

## Version: v1

### Quote Endpoint

*   **HTTP Method:** `GET`
*   **Path:** `/api/v1/quote`
*   **Description:** Retrieves a random quote.
*   **Parameters:** None
*   **Request Body:** None
*   **Success Response (200 OK):**
    *   **Content-Type:** `application/json`
    *   **Body:**
        ```json
        {
          "quote": "An example quote.",
          "author": "The Author"
        }
        ```
*   **Error Responses:**
    *   `500 Internal Server Error`: If the service encounters an unexpected error.

### Health Check Endpoint

*   **HTTP Method:** `GET`
*   **Path:** `/api/health`
*   **Description:** Checks the health status of the service.
*   **Parameters:** None
*   **Request Body:** None
*   **Success Response (200 OK):**
    *   **Content-Type:** `application/json`
    *   **Body:**
        ```json
        {
          "status": "UP"
        }
        ```
    *   *(Note: The exact content of the health check response can be basic for MVP, e.g., just an HTTP 200 status, or a simple JSON like above. More detailed health checks can be added later.)*
*   **Error Responses:**
    *   `503 Service Unavailable`: If the service is not healthy.