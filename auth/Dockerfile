FROM rust:buster as builder
WORKDIR /app
COPY . .
EXPOSE 4000
CMD ["sh","./entrypoint.sh"]
# CMD ["cargo","run"]