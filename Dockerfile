FROM rust:latest AS builder

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY Dioxus.toml Dioxus.toml

RUN mkdir src && echo 'fn main() { println!("Hello, world!"); }' > src/main.rs
RUN cargo install --path .

COPY package.json .
COPY package-lock.json .
COPY tailwind.config.js .
COPY tailwind.css .
COPY clippy.toml .
COPY assets assets
COPY .cargo .cargo
COPY src src

RUN dx bundle --release


# fix for missed mime-type for wasm
RUN echo ".wasm:application/wasm\n" > httpd.conf

# httd doesn't support brotly
RUN rm /app/target/dx/wordgame/release/web/public/assets/*.br

# compress assets with gz
RUN find /app/target/dx/wordgame/release/web/public/assets/ -type f -exec gzip -9 "{}" \;

#https://github.com/lipanski/docker-static-website
FROM lipanski/docker-static-website:latest

COPY --from=builder /app/target/dx/wordgame/release/web/public .
COPY --from=builder /app/httpd.conf .

EXPOSE 8080/tcp
CMD ["/busybox-httpd", "-f", "-v", "-p", "8080", "-c", "httpd.conf"]
