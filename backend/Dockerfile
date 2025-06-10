FROM rust:alpine AS builder
LABEL authors="SiLeader"

WORKDIR /work

COPY . .

RUN --mount=type=cache,target=/work/target \
    --mount=type=cache,target=/work/.cargo \
    apk add --no-cache musl-dev && \
    cargo build --release && \
    cp target/release/diffusionstash-backend /

FROM scratch
LABEL authors="SiLeader"

COPY --from=builder /diffusionstash-backend /bin/diffusionstash-backend

CMD ["/bin/diffusionstash-backend"]
