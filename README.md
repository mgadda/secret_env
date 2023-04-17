# Secret Env
Populate environment variables with secrets

## Installation

```bash
cargo install --path ./
```

Before using, ensure that `~/.cargo/bin` is your shell path.

## Usage 

```bash
$ secret_env PASSWORD some_command
PASSWORD: 
```

`some_command` now has access to an environment variable named PASSWORD whose contents 
is helpfully not logged in your shell history.
