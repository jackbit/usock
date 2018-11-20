# USOCK

Yet another websocket for Rust project. Features:

- Multi Channels
- Authentication with expiration session
- Mongodb integration (save message)
- Redis integration
- Sidekiq client integration (send message to background job)

#### Installation

Install rust from this url: https://www.rust-lang.org/en-US/install.html

#### Usage

Compile or build the application

    $ make build

Remove appliation

    $ make clean

Run application

```
# development
target/release/usock -p /path/to/config

# production
WS_RUN_MODE=production target/release/usock -p /path/to/config

# without log file
target/release/usock -p /path/to/config -l false

# with log path
target/release/usock -p /path/to/config -l tmp/usock.log

# client side
ws://ws.local.host/channel_name?token=79608eee16c5a9d5e8b7217f7eec6d93f4bbb40e&nonce=1516731538
```

To stop it, CTRL+C 

#### Configuration

```
ws:
  ssl:
     key: /path/to/private/key.pem
     cert: /path/to/private/cert.pem
  host: 127.0.0.1
  port: 3012
  max_connections: 10000
auth:
  private_key: YourSecretKey
  keep_alive: 129
mongo:
  uri: mongodb://username:password@localhost:27017
  db: app
  table: messages
rd:
  uri: redis://:_password_@localhost:6379/2
  ns: anything
```
Note:
`ws.ssl` is optional setting

#### Authorization

Used HMAC with sha-1 ( https://www.freeformatter.com/hmac-generator.html )
In your config file mandatory value is `private_key`.
But you can also set:

`time_name` - string, name of the `GET` parameter with public key the timestamp, by default `nonce`

`token_name` - string, name of the `GET` parameter with token, by default `token`. Encrypt your nonce with private key by using HMAC SHA1 to get the token.

`keep_alive` - integer, value in seconds between current value of the timestamp and `GET` parameter (field in config `time_name`), by default 120

## TODO

- Async Redis
- Async WS
- Logstash (filebeat) options instead mongodb
- Error notifier


## Development

Please report any issues to the GitHub issue tracker