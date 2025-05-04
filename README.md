# Tuno Media Platform

This repository contains the complete Tuno Media platform, consisting of a socket server backend and a multi-platform frontend application.

## Prerequisites

### IOTA Wallet Requirement

The Tuno platform requires a working IOTA wallet. To install and set up the IOTA wallet:

1. Follow the official [IOTA installation guide](https://docs.iota.org/developer/getting-started/install-iota)
2. Configure your wallet credentials before using the deployment commands

Without a properly configured IOTA wallet, you won't be able to publish or deploy the Tuno packages.

## Repository Structure

- [Tuno Socket Server](tuno-cli/README.md) - Backend gRPC socket server
- [Tuno Media Applications](webapp/README.md) - Frontend applications (Web SPA, Android)

## Quick Start

### Socket Server

The backend server provides gRPC endpoints for the Tuno platform.

```sh
# Install dependencies (Ubuntu)
sudo apt install libssl-dev pkg-config protobuf-compiler

# Test gRPC implementation
grpcui -plaintext "localhost:4114"

# Deploy with iota
iota client publish tuno
```

For more details, see the [server documentation](tuno-cli/README.md).

### Client Applications

The Tuno client is available as:

1. **Web SPA**: Single page application for browser access
2. **Android App**: Native mobile application built with Tauri

#### Web Development

```sh
# Install dependencies
npm install

# Start development server
npm run dev -- --open

# Build for production
npm run build
```

#### Android Development

```sh
# Set required environment variables
export JAVA_HOME=/opt/android-studio/jbr
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"

# Run development build
cargo tauri android dev

# Build production APK
cargo tauri android build --apk
```

For complete frontend documentation, see the [client documentation](webapp/README.md).

## Protobuf Generation

To generate TypeScript client code from protobuf definitions:

```sh
npx protoc \
--ts_out src/lib/proto \
--ts_opt long_type_string \
--ts_opt optimize_code_size \
--proto_path ../proto \
../proto/tuno.proto
```

## Deployment

See the respective documentation files for detailed deployment instructions:
- [Server deployment](tuno-cli/README.md#deploy)
- [Web SPA deployment](webapp/README.md#deployment-on-nginx)
- [Android app distribution](webapp/README.md#building-as-apk)