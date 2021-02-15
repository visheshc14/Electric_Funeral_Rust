FROM rust:1.31

WORKDIR /C:/Users/visha/OneDrive/Desktop/electric-funeral-rust
COPY . .

RUN cargo install --path .

CMD ["electric-funeral-rust"]