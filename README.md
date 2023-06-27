# Rustlight

🧪 MacOS Spotlight alternative for Windows written in Rust

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![GitHub Issues](https://img.shields.io/github/issues/enesuzun2002/rustlight.svg)](https://github.com/enesuzun2002/rustlight/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/enesuzun2002/rustlight.svg)](https://github.com/enesuzun2002/rustlight/pulls)

## Overview

Rustlight is a lightweight Windows application written in Rust that aims to provide a similar experience to MacOS Spotlight. It allows users to quickly search for and launch installed applications, making it easier to navigate and access programs on the Windows platform.

## Features

- Cache currently installed programs to an sqlite local db.

## Planned Features

- Search for installed applications on your Windows system
- Launch applications directly from the search results
- Open directory of the application directly from the search results
- Search for files on drives
- Simple and intuitive user interface


## Screenshots

Will be added once the app has an ui.

## Installation

### Prerequisites

- Rust tools

### Clone the Repository

```bash
git clone https://github.com/enesuzun2002/rustlight.git
```

### Build and Run

1. Change to the project directory:

   ```bash
   cd rustlight
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the application:

   ```bash
   cargo run
   ```

## Contributing

Contributions are welcome! If you find a bug or want to add a new feature, please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b my-feature`.
3. Make your changes and commit them: `git commit -am 'Add new feature'`.
4. Push to the branch: `git push origin my-feature`.
5. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.=
