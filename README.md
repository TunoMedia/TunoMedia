# Tuno Socket Server

Dependencies for Ubuntu:
```sh
sudo apt install libssl-dev pkg-config
```

Run debug version:
```sh
sudo openssl pkcs8 -topk8 -nocrypt -in /etc/letsencrypt/live/tuno.media/privkey.pem -out /etc/letsencrypt/live/tuno.media/privkey-pkcs8.pem
cargo build
sudo ./target/debug/tuno-cli
```