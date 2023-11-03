# Rustylogs

![GitHub](https://img.shields.io/github/license/glazk0/rustylogs)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/glazk0/rustylogs)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/glazk0/rustylogs/CI)

A versatile Discord bot written in Rust that listens to a configured channel and sends condensed changelogs to a targeted channel using ChatGPT.

## Features

- Configurable bot that listens to a designated channel for prompts.
- Utilizes ChatGPT to generate a condensed changelog from the prompt.
- Sends the generated changelog to a specified channel.
- Open-source and highly customizable.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

To get started, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/glazk0/rustylogs.git
```

2. Build the bot

```bash
cargo build --release
```

3. Run the bot

```bash
./target/release/rustylogs
```

The bot The bot will be up and running and initializing the database.sqlite, you will just need to configure it through the administrators commands.


## Configuration

Before using the bot, make sure to configure it by editing the config.toml file. You can set the following parameters:

- token: Your Discord bot token.
- api_key: Your ChatGPT API key.
- prompt: Your customized changelog prompt.

## Usage

1. Invite the bot to your Discord server and ensure it has the necessary permissions.
2. Configure the bot as described in the "Configuration" section.
3. Start the bot using the installation instructions.

## Contributing

I welcome contributions from the community. If you have ideas for improvements or find issues, please open a pull request or submit an issue.

## TODO

- [X] Create the client
- [X] Read the TOML configuration on init (support a hot reload ?)
- [X] SQLX Sqlite support  
- [X] Listen to configured channel
- [X] Create the output with ChatGPT
- [ ] When new message, generate the output (configurable verification prompt ? i.e: Here is the output that will be send to: ...)
  - Using local cache system ? 
  - Stocking message ID in DB so we can delete the output whenever the main message has been deleted
- [ ] Listen to component (such as the validation stuff)
- [X] Send the output to *send_to*
- [ ] Add Docker support
- [ ] Refactor
- [ ] Add better logging