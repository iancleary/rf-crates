# RF crates

Four public RF engineering crates in one Cargo workspace. Package names remain
unchanged; each crate keeps its own version and MIT license.

| Crate | Responsibility | Source and API guide |
| --- | --- | --- |
| `rfconversions` | Scalar RF conversions and numerical operations | [README](crates/rfconversions/README.md) |
| `touchstone` | S-parameter files and network analysis | [README](crates/touchstone/README.md) |
| `gainlineup` | Ordered RF hardware cascade analysis | [README](crates/gainlineup/README.md) |
| `linkbudget` | Terrestrial and satellite link budgets | [README](crates/linkbudget/README.md) |

Only these four are workspace members. Private applications and other public
crates are outside this repository. Existing standalone demonstration and fuzz
packages remain excluded from workspace membership.

## Development

Use an installed stable Rust toolchain with Clippy and rustfmt, plus `just` and
`uv`. No dependency or tool installation is performed by these instructions.

```bash
just check                    # Membership, format, lint, tests, numerics, docs, packages
just ci                       # All checks plus release builds
cargo test -p rfconversions    # Focused package checks
just numerics                 # Independent Decimal/Fraction comparisons
cargo test --workspace --no-default-features --locked
```

For cached, disconnected builds, prefix with `CARGO_NET_OFFLINE=true`. A missing
cached dependency is a blocked check, not permission to replace a component or
install unrelated tooling. Do not share target directories between worktrees.

There is one root `Cargo.lock`. Cross-crate dependencies use workspace paths and
registry version requirements, so local verification exercises local code while
Cargo can produce normal crates.io artifacts. `cargo package --workspace`
verifies the package archives together through Cargo's temporary registry.

## Guides

- [Workspace architecture, history, and migration decisions](docs/workspace.md)
- [rfconversions PR review and numerical evidence](docs/rfconversions-review.md)
- [Release policy and pending GitHub cutover](docs/releases.md)
- [Reusable workspace verification procedure](.agents/skills/verify-rf-workspace/SKILL.md)

The local monorepo preserves source histories and the reviewed `rfconversions`
PR commits. It has not changed upstream repository settings, merged upstream
PRs, published crates, or activated a new GitHub repository. Release publication
is blocked pending the cutover described above.
