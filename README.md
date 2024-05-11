# F1-Telemetry

## Overview

F1-Telemetry is a Rust application designed to capture and process telemetry data from F1 video games via UDP. The data is deserialized from UDP packets and serialized into JSON format for further processing or storage.

## Features

-   **Data Capture**: Listens for UDP packets on a configurable port.
-   **Support for Multiple Packet Types**: Handles various data types including motion, session, lap data, event data, etc.
-   **Error Handling**: Robust error handling and logging for failed serialization tasks.

## Installation

To set up the F1-Telemetry tool, follow these steps:

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/F1-Telemetry.git
    ```
2. **Navigate into the project directory**:
    ```bash
    cd F1-Telemetry
    ```

## Configuration

Before running the application, ensure the `config.json` file in the `src` directory is set up correctly with the desired port number. Here's an example of what the configuration file might look like:

```json
{
    "port": 20777
}
```

## Running the Application

To run the application, use the following command from the root of the project directory:

```bash
cargo run
```

## Dependencies

This project relies on several external crates, including `serde` for serialization and deserialization, and Rust's standard library for networking. Ensure that your `Cargo.toml` is up-to-date with all necessary dependencies.
