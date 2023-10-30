FROM fedora:latest as rustbuild

LABEL org.opencontainers.image.authors="moi_guillaume@hotmail.com"
LABEL version="1.0"
LABEL description="Runs the registry backend in a container."

RUN dnf -y update && \
    dnf install -y gcc sqlite sqlite-devel

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /usr/local/src/registry-backend

COPY src /usr/local/src/registry-backend/src

COPY migrations /usr/local/src/registry-backend/migrations

COPY Cargo.lock /usr/local/src/registry-backend

COPY Cargo.toml /usr/local/src/registry-backend

COPY diesel.toml /usr/local/src/registry-backend

RUN --mount=type=secret,id=registry_secret_key REGISTRYSECRETKEY_FILE=/run/secrets/registry_secret_key $HOME/.cargo/bin/cargo install --path . --root /usr/local

RUN ldd /usr/local/bin/registry-backend

FROM fedora:latest

COPY --from=rustbuild /usr/local/bin/registry-backend /usr/local/bin/registry-backend

RUN useradd --uid 999 --system --shell /sbin/nologin registry

VOLUME ["/mnt/storage"]

EXPOSE 8000

USER registry

WORKDIR /mnt/storage

CMD ["/usr/local/bin/registry-backend"]
