# Changelog

All notable changes to this project are documented here, together with the
reasoning behind them.

## [Unreleased] — branch `ci/pin-github-actions-to-sha`

This branch does one thing — make CI trustworthy and green again — but it took
three groups of changes to get there. They are listed in the order they became
necessary.

### 1. Pin every GitHub Action to a full commit SHA

**What changed:** every `uses:` in `.github/workflows/` now references a
40-character commit SHA with the human-readable tag kept as a trailing comment,
e.g.

```yaml
- uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
```

Files touched: `ci.yml`, `coverage.yml`, `post-merge.yml`, `verify-deps.yml`.

**Why:**

- **Mutable refs are a supply-chain hole.** A tag like `@v4` and a branch like
  `@master` / `@main` are pointers, not contents. Whoever controls the action's
  repository can move them at any time, and our next CI run would silently
  execute the new code. Two actions here were on outright branch refs
  (`dtolnay/rust-toolchain@master`, `bnjbvr/cargo-machete@main`), which is the
  worst case: no release process at all between an upstream push and our runner.
- **Our workflows handle real secrets.** `ci.yml` and `post-merge.yml` pass
  `secrets.COMMITER_PRODUCTS_EXT_WRITER_JSON` to `google-github-actions/auth`,
  and `coverage.yml` passes `secrets.CODECOV_TOKEN`. A compromised action in
  those jobs gets GCS write credentials for `committer-products-external`, the
  same bucket `gcs-push` uploads release binaries to. Pinning is what keeps a
  third-party repo compromise from becoming our credential leak.
- **Reproducibility.** With SHAs, re-running an old workflow run executes the
  same action code it originally did. With tags, CI can change behaviour with no
  commit on our side — which makes "it passed yesterday, it fails today" both
  possible and unexplainable.
- **This is the documented GitHub hardening guidance** and what `zizmor` /
  Dependabot's `pinned-dependencies` check expect. Dependabot still understands
  SHA pins and will open update PRs against them, so we keep automated updates —
  the trailing `# vX.Y.Z` comment is what it reads to know the current version.

The tag comment is not decorative: it is the only human-readable record of what
a SHA points at, and it is what makes review of future bump PRs possible.

### 2. Upgrade the actions that were too old to run

**What changed:** while pinning, several actions were moved forward at the same
time rather than pinned at their existing major:

| Action | Before | After |
| --- | --- | --- |
| `actions/checkout` | v4 | v7.0.1 |
| `actions/setup-python` | v2 | v7.0.0 |
| `actions/github-script` | v6 | v9.0.0 |
| `codecov/codecov-action` | v3 | v7.0.0 |
| `google-github-actions/auth` | v2 | v3.0.0 |
| `google-github-actions/setup-gcloud` | v2 | v3.0.1 |
| `google-github-actions/upload-cloud-storage` | v2 | v3.0.0 |

Pinned at their existing version (no upgrade): `Swatinem/rust-cache` (v2),
`baptiste0928/cargo-install` (v3), `re-actors/alls-green` (v1.2.2),
`dtolnay/rust-toolchain`, `bnjbvr/cargo-machete`, `taiki-e/install-action`.

**Why:** these were the actions CI was actually failing on. They were several
majors behind — old enough that their bundled Node runtimes and, for the
`google-github-actions/*` family, their auth behaviour are no longer supported
on current hosted runners. Pinning them at the old SHA would have frozen a
broken CI in place, which defeats the point of the exercise. Pinning and
upgrading had to land together.

> Reviewer note: the exact failure text per action came from the failing runs,
> not from this working tree. Worth a second look during review that each bump
> matches the error it was meant to fix.

### 3. Fix the source so it builds on the current toolchain

**What changed:** small, mechanical changes across five files plus `Cargo.lock`.

**Why:** the workspace sets `warnings = "deny"` and `unused = "deny"` in
`[workspace.lints.rust]` (`Cargo.toml`), so on a newer toolchain every new lint
is a hard build error, not a warning. On rustc/cargo 1.97.1, `cargo clippy
--all-targets --all-features` failed outright. Each fix below is the minimum
needed to compile:

- **`Cargo.lock` — `ethnum` 1.5.0 → 1.5.3.** Not cosmetic; this was a *build
  failure*, and the first thing that had to be fixed before anything else was
  even visible:

  ```
  error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
    --> ethnum-1.5.0/src/error.rs:16:14
     |
  16 |     unsafe { mem::transmute(()) }
  ```

  `Cargo.toml` already declares `ethnum = "1.5.0"` (caret range), so 1.5.3 needs
  no manifest change.

- **`Cargo.lock` — lockfile format `version = 3` → `version = 4`.** Written
  automatically by the newer cargo when it updated the lock. Requires Cargo
  1.78+ to read. Everything in CI is on `dtolnay/rust-toolchain@stable`, so this
  is fine, but it is the one change here that could bite anyone still building
  with an older local cargo.

- **`filled_tree/node_serde.rs` — added `#[allow(dead_code)]` to
  `LeafCompiledClassToSerialize`.** `error: struct
  LeafCompiledClassToSerialize is never constructed` (`dead_code`, via
  `-D unused`). The struct genuinely has no remaining constructor — it is only
  defined, never used. Its own doc comment says it is a *temporary* struct kept
  "to comply to existing storage layout", so it was suppressed rather than
  deleted, to avoid quietly dropping something that documents a storage
  contract. **This one is a deliberate deferral, not a fix** — the real question
  is whether the struct should still exist at all, and that belongs to whoever
  owns the storage layout.

- **`filled_tree/tree.rs` — `.iter().map(|(_, node)| …)` → `.values().map(|node|
  …)`.** `clippy::iter_kv_map` ("iterating on a map's values"). Clippy's own
  suggested fix; the key was being discarded anyway.

- **`original_skeleton_tree/create_tree.rs` — dropped `.into_iter()` from
  `.zip(db_keys.into_iter())`.** `clippy::useless_conversion` — `zip` already
  accepts any `IntoIterator`, so the call was redundant.

- **`updated_skeleton_tree/create_tree_helper.rs` — added a blank `///` line
  before the `Note that …` sentence.** `clippy::doc_lazy_continuation` ("doc list
  item without indentation"). Without the blank line the sentence was being
  parsed as a continuation of the preceding bullet, so the rendered docs were
  wrong, not just lint-noisy.

- **`committer_cli/src/tests/python_tests.rs` — removed `use log::error;`.**
  `error: unused import` (`unused_imports`, via `-D unused`). The one call site
  (line 201) uses the fully-qualified `log::error!`, so nothing depended on the
  import.

Nothing here changes behaviour: no logic, no serialization format, no public
API. The `.values()` and `.zip()` rewrites are semantically identical to what
they replaced.

### Verification

```
$ cargo clippy --all-targets --all-features
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
```

Clean, with `warnings = "deny"` in force. Each of the five source fixes was
confirmed necessary by reverting it individually and observing the specific
error it resolves.

Not yet run locally: `cargo test`, `scripts/rust_fmt.sh --check`,
`scripts/clippy.sh`, and the regression/benchmark jobs (those need GCS
credentials). The upgraded actions themselves can only really be validated by a
CI run on this branch.

### Known issue, not addressed here

`ci.yml` line 183 — the `gcs-push` job still pins `runs-on: ubuntu-20.04`. That
runner image has been retired by GitHub, so this job is expected to fail
independently of anything on this branch. Every other job uses
`ubuntu-latest`. Left alone deliberately: it is a runner change, not an action
pin, and it deserves its own commit rather than being smuggled into this one.
