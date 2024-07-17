# shorten-cli

[![License](https://img.shields.io/badge/license-MIT-orange.svg)](https://github.com/CloudVEX/shorten-cli/blob/main/LICENSE)
[![Created](https://img.shields.io/github/created-at/CloudVEX/shorten-cli?color=orange)](https://github.com/CloudVEX/shorten-cli)
[![Activity](https://img.shields.io/github/commit-activity/m/CloudVEX/shorten-cli?color=orange)](https://github.com/CloudVEX/shorten-cli/graphs/contributors)

### A CLI for my url shortening api with a redirect peeker

### Usage

- `shorten cr "<url>"` (Creates a new shortcode and returns you the url to share)
- `shorten rm <short-code>` (Removed a shortcode when auth is set)
- `shorten get <short-code>` (Returns you the redirection url to peek where you are going)

### Config
This part is only really neccessary if you self host [url-short](https://github.com/CloudVEX/url-short) and plan to delete shortcodes you created there.
- File Locations
  - Linux: /home/username/.config/shorten-cli/cli.toml
  - Windows: C:\Users\Username\AppData\Roaming\shorten-cli\cli.toml
  - MacOS: /Users/Username/Library/Application Support/shorten-cli/cli.toml
- You can find the example config [here](https://github.com/CloudVEX/shorten-cli/blob/main/cli.toml.example).