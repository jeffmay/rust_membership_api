FROM scratch

ADD target/x86_64-unknown-linux-musl/release/rust_membership_api /
EXPOSE 8000

CMD ["/rust_membership_api"]
