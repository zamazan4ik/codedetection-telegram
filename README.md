# codedetection-telegram
Detect C++ code in Telegram messages.

### Dependencies
* C++ compiler with C++17 support
* CMake
* Conan

### How to build
* Clone this repository
* `cmake -B build`
* `cmake --build build --target install`

### How to run
You must provide Telegram Bot API token to the `codedetection-telegram` with `--token` option. `codedetection-telegram` has other command line options but only `--token` is mandatory - other options have some reasonable defaults.
Since `libcurl` can be built without bundled certificates you need to provide them explicitly via `--ca-info` bot option.

So your command line for running `codedetection-telegram` can be like this one:
`codedetection_telegram --token ${TOKEN} --log-path ${LOG_PATH}`

I recommend to run this bot as a service(e.g. as systemd service) on a machine.
Also Docker images are available here: https://hub.docker.com/repository/docker/zamazan4ik/codedetection-telegram
