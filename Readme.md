# Keep-Alive Connections
Keep-Alive, also known as persistent connections, is a feature used in networking protocols like HTTP and TCP to maintain an open connection between a client and server for multiple requests and responses, rather than creating a new connection for each interaction.

## Purpose
The main goal of Keep-Alive is to reduce latency and overhead by avoiding the need to repeatedly open and close connections. This is particularly useful in high-performance systems or when handling multiple short-lived requests.

## Key Features
- Connection Reuse: A single TCP connection is reused for multiple requests, eliminating the setup and teardown cost of new connections.
- Reduced Latency: Avoids the round-trip time needed to establish new TCP handshakes, improving overall response time.
- Improved Throughput: Keeps the pipeline open, allowing for more requests to be served efficiently over the same connection.
- Lower Resource Consumption: Reduces CPU and memory usage on both client and server by minimizing connection churn.
- Heartbeat or Ping Mechanism: Many implementations use periodic signals to keep the connection from being considered idle or closed by intermediate network devices.