# Tuno Media

Everything you need to build and deploy the Tuno web application

# Development

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev -- --open
```

## Building Single Page Application (SPA) with npm

To create a production version of the app:

```bash
npm run build
```

## Deployment on Nginx

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
