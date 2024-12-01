FROM mcr.microsoft.com/devcontainers/rust:latest

ARG RUST_VERSION=1.82.0

# Install specific Rust version
RUN rustup default ${RUST_VERSION} \
    && rustup override set ${RUST_VERSION}

# Install additional packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
        cmake \
        clang \
        llvm \
        libelf-dev \
        linux-headers-generic \
        gdb \
        lldb \
        strace \
        perf-tools-unstable \
        bpftrace \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install additional Rust components
RUN rustup component add \
    rustfmt \
    clippy \
    rust-analysis \
    rust-src \
    llvm-tools-preview

# Set up git configuration
RUN git config --global core.autocrlf input \
    && git config --global pull.rebase true \
    && git config --global rebase.autoStash true \
    && git config --global init.defaultBranch main \
    && git config --global commit.gpgsign true \
    && git config --global core.editor "code --wait"

# Verify installations
RUN rustc --version \
    && cargo --version \
    && clang --version \
    && llvm-config --version