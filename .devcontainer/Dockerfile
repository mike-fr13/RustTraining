FROM rust:latest

# Installer les outils de développement et les dépendances nécessaires
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Optionnel : Installer des outils Rust supplémentaires
RUN cargo install cargo-edit cargo-audit