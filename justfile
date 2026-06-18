current_branch := `git branch --show-current`

# Build the release binary
build:
    cargo build --release

# Run, forwarding args (e.g. `just run file.txt`)
run *args:
    cargo run -- {{ args }}

# Run tests
test:
    cargo test

# Lint
lint:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

# Auto-fix lint issues
fix-lint:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-targets

# Update dependencies
update:
    cargo update

# Remove build artifacts
clean:
    cargo clean

# Git helpers
commit:
    git add -A
    git commit -v

push:
    git push -u origin {{ current_branch }}
    git push --tags --no-verify
