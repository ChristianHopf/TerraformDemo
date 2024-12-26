FROM nginx:alpine-slim
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY ./index.html /usr/share/nginx/html/
COPY ./CHANGELOG.md /usr/share/nginx/html/