# Releases in the RF workspace

`rfconversions` now belongs to the four-crate Cargo workspace. Follow the root
[release policy](../../../docs/releases.md) for versioning, verification,
dependency order, authorized publication, and recovery.

Run `just check` at the workspace root. Member `just package` verifies all four
package archives with Cargo's temporary registry. The legacy standalone release
script is retained for provenance but exits before mutation in this workspace.
Use the root `release/rfconversions.toml` contract for an explicitly authorized
release. GitHub release creation requests publication; verify the publishing
workflow and crates.io index before considering it complete.
