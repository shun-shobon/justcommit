# JustCommit

## Overview

JustCommit is an AI-powered commit message generator.
Based on the changes in your Git repository, it automatically suggests a commit message for you.
All you have to do is **Just Commit**.
This project is inspired by opencommit.

## Installation

You can either clone this repository and build it yourself using Rust or download the latest binary from the Release page.
Make sure to place the binary in a directory included in your system's path.

## Usage

Before committing with Git, execute JustCommit
It will automatically detect the staged changes in your Git repository and output a suggested commit message to the standard output.
While it currently only provides the output, future plans include supporting Git-hooks and launching an editor automatically.

## Configuration

JustCommit requires an OpenAI token to function. Place the token in `config.toml`. This configuration can be located in one of the following locations:

- `$XDG_CONFIG_HOME/justcommit` (For non-Windows users)
- `$APP_DATA/justcommit` (For Windows users)
- `$HOME/.config/justcommit` (If the above environment variables are not defined)

The configuration format is as follows:

```toml
[openai_token]
type = "plain"
token = "<Your OpenAI Token>"
```

If you're uncomfortable storing your token in plain text, you can use a secret reference from 1Password.
In this case, replace "plain" with "1password" for the type.
