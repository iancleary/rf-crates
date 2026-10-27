# Releases in the RF workspace

`rfconversions` now belongs to the four-crate Cargo workspace. Follow the root
[release policy](../../../docs/releases.md) for versioning, verification,
dependency order, and the pending GitHub cutover.

Run `just check` at the workspace root. Member `just package` verifies all four
package archives with Cargo's temporary registry. The legacy standalone release
script is retained for provenance but exits before mutation in this workspace.
Publication is not enabled; no registry versions, tags, or GitHub releases have
been created by the migration.
