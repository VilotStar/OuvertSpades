set shell := ["bash", "-uc"]
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias bd := dbuild

default: build

build:
    cargo build --release

dbuild:
    cargo build

fmt:
    cargo fmt

clippy:
    cargo clippy