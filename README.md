# codedetection-telegram
[![GitHub code size](https://img.shields.io/github/languages/code-size/ZaMaZaN4iK/codedetection-telegram?style=flat)](https://github.com/ZaMaZaN4iK/codedetection-telegram)
### About
Detect C++ code in Telegram messages and warn about proper code formatting.

### Dependencies
* [Rust](https://www.rust-lang.org/) 1.44 or newer
* Cargo

### How to build
* Clone this repository
* `cargo build --release`

### How to run
I recommend to run this bot as a service(e.g. as systemd service) on a machine.
Also Docker images are available here: https://hub.docker.com/repository/docker/zamazan4ik/codedetection-telegram

### Configuration
The bot can be configured only with environment variables. For now there are we support the following variables:

| Name | Description | Values | Default value | Required |
|------|-------------|--------|---------------|----------|
| TELOXIDE_TOKEN | Telegram bot token | Any valid and registered Telegram bot token | None | All mods |
| WEBHOOK_MODE | Run bot in webhook mode or long-polling mode | `true` for webhook, 'false' for long-polling | `false` | All mods |
| THRESHOLD | Sets a threshold for code detection algorithm | Any valid `u8` | `3` | All mods |
| BIND_ADDRESS | Address for binding the web-service | Any valid IP address | `0.0.0.0` | Webhook mode |  
| BIND_PORT | Port for binding the web-service | Any valid port | `8080` | Webhook mode |
| WEBHOOK_URI | This variable allows you to set your hook path example **https://example.org/api/v1/Te3_@#ge** |Any valid host+path | None | Webhook mode |

If for any variable there is no default value and you didn't provide any value - the bot won't start.
Bot automatically registers webhook (if is launched in webhook mode) with address `https://$HOST/$TELOXIDE_TOKEN/api/v1/message`.

### How to use
Just add the bot tou your chat. If any user will write a message, which possibly is a C++ code - the bot will warn about it.

### Code detection algorithm
For now bot detects C++ code with very simple method: match with regular expression some keywords and if match count > `THRESHOLD` - send a warn.
So this algorithm has some false positives (detects some non-C++ code as C++ code) and false negatives (doesn't detect some 
C++ code). Maybe in the future the algorithm will be improved.

### Feedback
If you have any suggestions or want to report a bug - feel free to create in issue in this repo. Thank you!
