
# vidcat

Vidcat is a command-line tool designed to display images and videos directly in your terminal. It uses ANSI escape sequences and Unicode block characters to render media content in a terminal.

## Features

- Display images in the terminal using half-block rendering for higher resolution.
- Scale images to fit the terminal dimensions.
- Support for basic video frame rendering (future feature).

## Installation

Ensure you have Rust installed on your system. You can install Rust via [rustup](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/yourgithub/vidcat.git
cd vidcat
```

```bash
# Install binary
cargo install --path .
```

## Usage

To display an image in your terminal, simply run:

```bash
vidcat /path/to/your/image.png
```