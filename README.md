# secret_env

`secret_env` is a simple Rust utility that allows you to securely pass environment variables to a command without exposing the secret in your shell history or process list. This is particularly useful when running commands that require sensitive information like API keys, passwords or tokens.

## Features

- Securely prompts for a secret value at runtime
- Passes the secret as an environment variable to a specified command
- Keeps sensitive information out of shell history and process lists

## Prerequisites

- Rust 1.46 or higher
- Unix-based operating system (macOS, Linux, BSD)

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/secret_env.git
   ```

2. Change into the project directory:

   ```
   cd secret_env
   ```

3. Build and install the binary:

   ```
   cargo build --release
   sudo cp target/release/secret_env /usr/local/bin/
   ```

## Usage

```
Usage: secret_env <ENV_VAR_NAME> <COMMAND> [ARGS...]
```

- `ENV_VAR_NAME`: Name of the environment variable containing the secret
- `COMMAND`: Command to execute with the secret environment variable
- `ARGS`: Additional command line arguments for the specified command

### Example

If you have a command `deploy_app` that requires an API key passed as an environment variable named `API_KEY`, you can use `secret_env` as follows:

```
secret_env API_KEY deploy_app --target production
```

`secret_env` will securely prompt you for the `API_KEY` value, then run the `deploy_app` command with the provided environment variable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
