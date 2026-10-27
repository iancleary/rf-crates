# Show workspace commands.
help:
    just --list

# Verify the approved package boundary and local dependency resolution.
workspace-check:
    uv run --offline --no-project scripts/verify_workspace.py

fmt:
    cargo fmt --all

format: fmt

fmt-check:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

test:
    cargo test --workspace --all-features --locked

# Preserve library-only configurations as well as default CLI features.
test-library:
    cargo test --workspace --no-default-features --locked

doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked

# Cargo stages the workspace packages in a temporary registry before verification.
package:
    cargo package --workspace --locked

build:
    cargo build --workspace --release --all-features --locked

# Rerun the retained independent numerical references against this workspace.
numerics:
    cd crates/rfconversions && uv run --offline --no-project .agents/skills/rf-literature-review/scripts/verify_numerics.py

check: workspace-check fmt-check lint test test-library numerics doc-check package

ci: check build

# Publication remains disabled until the GitHub and registry cutover is approved.
cut-release *args:
    @echo "Release publication is disabled during migration. Read docs/releases.md." >&2
    @exit 2
