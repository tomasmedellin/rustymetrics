
# Rusty Metrics 📊

Welcome to the Rusty Metrics! This simple yet powerful tool provides you with detailed metrics on your system's Battery, Memory, and CPU. 

## Table of Contents 📜

- [Features](#features-🌟)
- [Installation](#installation-🛠)
- [Usage](#usage-🚀)
- [Contributing](#contributing-🤝)
- [License](#license-📝)

## Features 🌟

- **Battery Metrics** 🔋:
  - Percentage: Know how much juice you have left!
  - Charging Status: Are you plugged in or running on battery?
  - Temperature: Keep an eye on your battery's health.
  - Energy: Total energy consumption and remaining.

- **Memory Metrics** 🧠:
  - Total Used: How much memory is currently being used.
  - Total Free: How much memory is still available.
  - Total: Overall memory capacity of your system.

- **CPU Metrics** 🖥:
  - Percentage Use: Monitor your CPU's workload.
  - Frequency: *This feature is currently a work in progress (WIP)*.
  - Details: Additional information about your CPU.

## Installation 🛠

1. Ensure you have Rust and Cargo installed on your machine.
   
   If not, install them using:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:

   ```bash
   git clone https://github.com/tomasmedellin/rustymetrics.git
   ```

3. Navigate to the cloned directory and build the project:

   ```bash
   cd BasicRustMetrics
   cargo build --release
   ```

4. You can now find the binary in the `target/release` directory.

## Usage 🚀

To run the program, simply execute:

```bash
./target/release/basic_rust_metrics
```

You will then see a detailed report of your system's metrics.

## Contributing 🤝

We welcome contributions! If you'd like to help improve the Basic Rust Metrics Program, please:

1. Fork the repository.
2. Create a new branch with your changes.
3. Open a pull request.

For more details, please refer to the CONTRIBUTING.md file.

## License 📝

This project is licensed under the MIT License. For more information, please see the LICENSE.md file.

---

Thank you for using or contributing to the Basic Rust Metrics Program! If you have any questions or feedback, please feel free to open an issue or reach out to the maintainers. 🙌
