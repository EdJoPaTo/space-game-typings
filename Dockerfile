FROM docker.io/library/nginx:1-alpine
RUN apk --no-cache upgrade

COPY nginx.conf /etc/nginx/conf.d/default.conf
RUN nginx -t

WORKDIR /usr/share/nginx/html
COPY typescript/generated.ts typings.ts
COPY static ./
