# Git Commit Message Generator

A CLI tool that generates commit messages based on git changes using Claude's AI API.

## Installation

```bash
cargo install --path .
```

## Configuration

Create a `.env` file in the same directory where you run the command with your Anthropic API key:

```
ANTHROPIC_API_KEY=your_api_key_here
```

Alternatively, you can set the environment variable directly:

```bash
export ANTHROPIC_API_KEY=your_api_key_here
```

## Usage

Run the tool in a git repository:

```bash
commit-msg
```

For verbose output, use the `-v` or `--verbose` flag:

```bash
commit-msg --verbose
```

## Features

- Automatically detects git changes (staged or unstaged)
- Sends the diff to Claude API
- Generates a concise and descriptive commit message
- Follows git commit message best practices

## Example Output

```
Suggested commit message:

Add user authentication functionality

Implement login, registration, and password reset endpoints with JWT token generation and validation
```

## Requirements

- Rust 1.63 or later
- Git
- Anthropic API key
