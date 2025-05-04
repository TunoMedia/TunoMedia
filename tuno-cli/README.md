# Tuno Socket Server

Dependencies for Ubuntu:
```sh
sudo apt install libssl-dev pkg-config protobuf-compiler
```

In case it doesn't work with letsencrypt auto-generated files:
```sh
sudo openssl pkcs8 -topk8 -nocrypt -in /etc/letsencrypt/live/tuno.media/privkey.pem -out /etc/letsencrypt/live/tuno.media/privkey-pkcs8.pem
```

#### Manual Testing

Check logs with `sudo journalctl -u tuno-distributor.service`

Test grpc implementation with `grpcui`:
```sh
grpcui -plaintext "localhost:4114"
```

## Deploy

Publish package:
```sh
iota client publish tuno
```

Export created PackageID from Published Objects:
```sh
export PKG="<PackageID>"
```