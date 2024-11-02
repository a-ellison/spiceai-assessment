FROM postgres:latest

# configure default user to be created by postgres image
ENV POSTGRES_USER=demo
ENV POSTGRES_PASSWORD=demo

COPY init.sql /docker-entrypoint-initdb.d/
