FROM node:alpine AS builder

WORKDIR /work

COPY . .

RUN --mount=type=cache,target=/work/node_modules \
    --mount=type=cache,target=/work/.angular/cache \
    npm install && \
    npm run build

# Server
FROM nginx:alpine
LABEL authors="SiLeader"

COPY --from=builder /work/dist/frontend/browser /var/www/html/
COPY nginx/default.conf /etc/nginx/conf.d/

EXPOSE 80
