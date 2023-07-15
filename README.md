# cowrie-discord-alert

Simple program I wrote to monitor cowrie logs and send alerts to discord webhook whenever a log matches my criteria[s]

# Build

Change values in main.rs before you build

Prod build: `cargo build --release`

Binary will be in `target/release` after build

# Credits

### Cowrie

Very well made honeypot for ssh/telnet (lots of features)

https://github.com/cowrie/cowrie

### logwatcher

This library hasn't been updated in 5 years but still gets the job done

Thanks for doing most of the work for me

https://github.com/aravindavk/logwatcher
