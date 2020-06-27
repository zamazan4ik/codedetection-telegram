# codedetection-telegram
Detect C++ code in Telegram messages.

### Dependencies
* Rust compiler (1.44)
* cargo

### How to build
* Clone this repository
* `cargo build --all --all-targets`

### How to run
You must provide Telegram Bot API token to the `codedetection-telegram` with `TELOXIDE_TOKEN` environment variable.

I recommend to run this bot as a service(e.g. as systemd service) on a machine.
Also Docker images are available here: https://hub.docker.com/repository/docker/zamazan4ik/codedetection-telegram
