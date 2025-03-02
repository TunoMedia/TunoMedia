# Tuno Media

Everything you need to build and deploy the Tuno application

# Single Page Application (SPA)

#### Development

Once you've installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev -- --open
```

#### Building with npm

To create a production version of the app:

```bash
npm run build
```

#### Deployment on Nginx

Make sure the production files are readable by nginx:

```bash
sudo cp -r ./build /opt/tuno
```

Nginx can be configured as such:

```
server {
    server_name tuno.media;

    access_log /var/log/nginx/tuno-access.log;
    error_log /var/log/nginx/tuno-error.log

    root /opt/tuno;

    location / {
        try_files $uri $uri/ $uri.html /index.html;
    }

    listen 80 ;
    listen [::]:80 ;
}
```

# Android mobile application

#### Prerequisites

Following [Tauri's Anddroid prerequisites](https://v2.tauri.app/start/prerequisites/#android) instructions:

1. Download Android Studio

2. Use the SDK Manager in Android Studio ("More Actions" button) to install the following:

    - SDK Platforms: latest SDK
    - SDK Tools: NDK (Side by side)

3. Set environment variables:

```bash
export JAVA_HOME=/opt/android-studio/jbr
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
```

#### Development

Generate an Android Studio project and start a development instance:

```bash
cargo tauri android init
cargo tauri android dev
```

#### Building as APK

To create a production version of the app:

```bash
cargo tauri android build --apk
```
