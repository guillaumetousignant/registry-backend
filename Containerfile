FROM fedora:latest

LABEL org.opencontainers.image.authors="moi_guillaume@hotmail.com"
LABEL version="1.0"
LABEL description="Runs the registry backend in a container."

RUN dnf -y update && \
    dnf install -y git gcc sqlite sqlite-devel

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN --mount=type=secret,id=registry_secret_key,target=registry_secret_key REGISTRYSECRETKEY_FILE=/registry_secret_key cargo install --path . --root /usr/local

RUN dnf remove git gcc sqlite-devel

VOLUME ["/mnt/storage"]

RUN useradd --uid 1000 --system --shell /sbin/nologin registry

EXPOSE 8000

USER registry

WORKDIR /mnt/storage

CMD ["registry-backend"]
