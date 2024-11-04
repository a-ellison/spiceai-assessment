FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=demo
# configure default user to be created by mysql image
ENV MYSQL_USER=demo
ENV MYSQL_PASSWORD=demo
ENV MYSQL_DATABASE=demo

COPY init.sql /docker-entrypoint-initdb.d/
