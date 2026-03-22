# Changelog

All notable changes to [UltraSearch](https://github.com/Dicklesworthstone/ultrasearch) are documented in this file.

UltraSearch is a high-performance, memory-efficient desktop search engine for Windows. It combines NTFS MFT enumeration (Everything-style instant filename search) with full-text content indexing via Tantivy, using a multi-process architecture: an always-on service, short-lived index workers, and a GPU-accelerated GPUI desktop UI.

Versioning note: tags v0.2.0 through v0.2.37 were rapid CI iteration tags (many are CI-only fixes). The v1.4.5 release is the first stable, published release with downloadable binaries. Tags without a corresponding GitHub Release are marked **(tag only)**.

---

## [Unreleased]

Post-v1.4.5 changes not yet tagged or released.

### Infrastructure
- Refactor CI into focused jobs and add release workflow with SHA-256 checksums ([`9439154`](https://github.com/Dicklesworthstone/ultrasearch/commit/9439154f4ce9831e3a2cedf7077f8c88fad3d5d1)) — 2026-01-17
- Relax clippy to warnings instead of errors in CI ([`0f2042b`](https://github.com/Dicklesworthstone/ultrasearch/commit/0f2042b6513eb68e9d0f0680d01b92c5c9788b2c)) — 2026-01-24

### Licensing
- Update license to MIT with OpenAI/Anthropic Rider ([`1468c63`](https://github.com/Dicklesworthstone/ultrasearch/commit/1468c63ec2afb751e7720af5e3db8a51489112dd)) — 2026-02-21

### Documentation
- Add GitHub social preview image ([`eb35227`](https://github.com/Dicklesworthstone/ultrasearch/commit/eb35227d371d1c7c5d8153841e05c8eebb229ce0)) — 2026-02-21
- Add CASS (Cross-Agent Session Search) tool reference to AGENTS.md ([`5067e25`](https://github.com/Dicklesworthstone/ultrasearch/commit/5067e254eebfebb03a7ccb8f93a18832b947c1fe)) — 2026-02-25

---

## [v1.4.5] — 2025-12-09

**First stable release with published binaries.** GitHub Release with Windows x64 executables: `service.exe`, `cli.exe`, `index-worker.exe`.

This release represents the culmination of all v0.2.x development plus significant post-v0.2.37 feature work on the installer, launcher, UI polish, and indexing pipeline.

### Release Assets
| Binary | Size | SHA-256 |
|--------|------|---------|
| `service.exe` | 6.4 MB | `6D6721FDCC9BB12B1470AC7B55F98BB33D6F8F6423427424E17F044612FC7710` |
| `cli.exe` | 1.2 MB | `634AC90652AB29FD00B887A3E10EC89307293EC93E74A785B4D2ED47538DFEF8` |
| `index-worker.exe` | 4.3 MB | `A3D9C0FFE100BDBD7B2E63D33E1F772FE8D8867182B54C86E3074FD9C1C70B83` |

### Service & Indexing (since v0.2.37)
- Fix worker spawn paths, config `%PROGRAMDATA%` env expansion, and worker logging ([`4ef1c14`](https://github.com/Dicklesworthstone/ultrasearch/commit/4ef1c143a41ebbd6a6f8a4653bc308a900296b57))
- Fix clippy let-and-return in `default_config_path` ([`c535d21`](https://github.com/Dicklesworthstone/ultrasearch/commit/c535d217d873343fd76d8633059e781afdc1d97e))
- Track content job sizes and expose progress counters to UI/IPC ([`199826c`](https://github.com/Dicklesworthstone/ultrasearch/commit/199826c34a6c22f398d1387fb1d30255fb1b4c49))
- Hard-cap indexing workers and drop process priorities for background-respectful behavior ([`bd96a2a`](https://github.com/Dicklesworthstone/ultrasearch/commit/bd96a2a4885b6dc85702c9348549db4e958e8c0c))
- Fix idle detection when no volumes are selected ([`96f926f`](https://github.com/Dicklesworthstone/ultrasearch/commit/96f926ff50f18c6224b18f8f590baa37b70c0ec9))

### IPC
- Add rescan request plumbing for on-demand re-indexing ([`2b5d1ed`](https://github.com/Dicklesworthstone/ultrasearch/commit/2b5d1ed18ef37d98470898956999be09d6a0e635))

### UI
- Richer indexing progress display with stronger primary styling ([`a7cd6cb`](https://github.com/Dicklesworthstone/ultrasearch/commit/a7cd6cbe84b66976dd6ee8344c56609c914698d7))
- Add tray minimize, progress surfaces, and clearer onboarding flow ([`304515d`](https://github.com/Dicklesworthstone/ultrasearch/commit/304515dff89539df8f3db33e7912fd19d2059f1c))
- Strengthen accent color and keep onboarding feedback ([`78b45d7`](https://github.com/Dicklesworthstone/ultrasearch/commit/78b45d758bfef1778fe244a15db24b08aeec4c33))
- Normalize onboarding source ending ([`160c5e2`](https://github.com/Dicklesworthstone/ultrasearch/commit/160c5e2ba072f834c0ce0494d5fd42184193695d))

### Installer & Launcher
- Add product icon and clean shortcut handling ([`0996cc7`](https://github.com/Dicklesworthstone/ultrasearch/commit/0996cc765fbbfb7bb2aa17f0ad45e7093ac055fb))
- Hide consoles by default, add `--show-console` flag to launcher ([`86b4ca6`](https://github.com/Dicklesworthstone/ultrasearch/commit/86b4ca606da14b0ecba3441e4c014bb9ef339a6f))
- Clean background priority cast for clippy ([`c923b43`](https://github.com/Dicklesworthstone/ultrasearch/commit/c923b43b1c44766158bfff95dc0a7a7abdeb0484))

---

## v0.2.23 through v0.2.37 — 2025-11-24 to 2025-11-26 (tag only)

Rapid CI iteration focused on getting MSI (Windows Installer) packaging working with `cargo-wix` inside a Cargo workspace. These tags have no GitHub Releases and contain only CI/build pipeline changes. They are grouped here for brevity.

### MSI Packaging Saga
The core challenge: `cargo-wix` does not natively support Cargo workspaces with wildcard dependency versions. Over 15 iterations refined the approach from naive invocation to a working strategy of building a temporary flattened manifest with explicit dependency versions.

Key milestones:
- **v0.2.23** ([`cf96041`](https://github.com/Dicklesworthstone/ultrasearch/commit/cf9604195901073966c1615a918035fc164e4702)): Fail fast when `ui.exe` missing; run cargo wix inside copied manifest
- **v0.2.31** ([`b5b5734`](https://github.com/Dicklesworthstone/ultrasearch/commit/b5b5734d94b9e46ca685c44b9e7dbc92c8528634)): Patch `Cargo.toml` deps in-place during wix run; install cargo-wix from git
- **v0.2.35** ([`096cb69`](https://github.com/Dicklesworthstone/ultrasearch/commit/096cb69be3ce5066051d7ce7c3d421bda3d2137f)): Build MSI with standalone ui manifest (working approach)
- **v0.2.37** ([`3247352`](https://github.com/Dicklesworthstone/ultrasearch/commit/3247352dbd3db39be84eeea5cf3a75ef07a2e184)): Add retry logic to GraalVM download in CI

All tags in this range: v0.2.23, v0.2.24, v0.2.25, v0.2.26, v0.2.27, v0.2.28, v0.2.29, v0.2.30, v0.2.31, v0.2.32, v0.2.33, v0.2.34, v0.2.35, v0.2.36, v0.2.37.

---

## [v0.2.22] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Build cargo wix with copied manifest preserving relative paths ([`3f48488`](https://github.com/Dicklesworthstone/ultrasearch/commit/3f484882f16c841136baefa2dda5c3554741ff76))

---

## [v0.2.21] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Run cargo wix from workspace root with ui package ([`aecad3e`](https://github.com/Dicklesworthstone/ultrasearch/commit/aecad3eaeea4a70c6dd5a7220c796ab924b880ee))

---

## [v0.2.20] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Call cargo wix with manifest path positional ([`006ff41`](https://github.com/Dicklesworthstone/ultrasearch/commit/006ff41aceb5152064101316e7eb8b9977e0b1f4))

---

## [v0.2.19] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Set `CARGO_MANIFEST_PATH` for cargo wix ([`8907b04`](https://github.com/Dicklesworthstone/ultrasearch/commit/8907b04d346b3dda90073c98322fbfedbe72b3c2))

---

## [v0.2.18] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Use default manifest for cargo wix invocation ([`b65b386`](https://github.com/Dicklesworthstone/ultrasearch/commit/b65b38683893d113aec879ef1eb4289d4a38b809))

---

## [v0.2.17] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Fix cargo wix invocation to use manifest path ([`284b85a`](https://github.com/Dicklesworthstone/ultrasearch/commit/284b85ab5b1638f2354ae7150284b01ac9a42844))

---

## [v0.2.16] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Fix MSI build bin-path for cargo wix ([`ecc66d2`](https://github.com/Dicklesworthstone/ultrasearch/commit/ecc66d2c91c40c0278e0a7f9e435fd559b3d77a3))

---

## [v0.2.15] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Pass `ui.exe` bin-path to cargo wix ([`d54dc62`](https://github.com/Dicklesworthstone/ultrasearch/commit/d54dc624654f7f23e30581178b3f8c1ee5737969))

---

## [v0.2.14] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Always upload MSI artifact and log path ([`54dad5d`](https://github.com/Dicklesworthstone/ultrasearch/commit/54dad5d777fda563ad4bd0e35dd966560ac8ed67))

---

## [v0.2.13] — 2025-11-24

GitHub Release (no release notes). CI iteration for MSI packaging.

- Log wix outputs and detect MSI recursively ([`6a6fa89`](https://github.com/Dicklesworthstone/ultrasearch/commit/6a6fa89ea43243c8da0df2c7cf113fc3b70b8c0b))

---

## [v0.2.12] / [v0.2.11] — 2025-11-24

GitHub Releases (both point to the same commit). First attempts at MSI packaging in CI.

- Determine `msi_built` via file existence ([`6c57dd1`](https://github.com/Dicklesworthstone/ultrasearch/commit/6c57dd15f4db12739de6fc3026654e0b0abe1b45))
- Generate temp manifest for cargo-wix without workspace deps ([`bcbcced`](https://github.com/Dicklesworthstone/ultrasearch/commit/bcbcced13e3986dd08fb5d4fb660e469378b36e9))
- Install cargo-wix from git to support workspace deps ([`820c440`](https://github.com/Dicklesworthstone/ultrasearch/commit/820c44021138c469d4371322e889c9600d08bd22))
- Grant contents:write for release publish and soften MSI requirement ([`f471903`](https://github.com/Dicklesworthstone/ultrasearch/commit/f47190302e3f01da90c893fc2beedf3e10faa828))
- Make MSI packaging optional when wix/cargo-wix fails ([`0194a1d`](https://github.com/Dicklesworthstone/ultrasearch/commit/0194a1d0a93a12b5226a22063ebda38e39dec379))

---

## v0.2.10 — 2025-11-23 (tag only)

CI stabilization for Windows builds.

- Increase Windows timeouts and stabilize `CARGO_TARGET_DIR` ([`807dce8`](https://github.com/Dicklesworthstone/ultrasearch/commit/807dce857c6a54c75c33cbbb148ccfb59fc33271))

---

## v0.2.9 — 2025-11-23 (tag only)

- Fetch GraalVM zip for Windows CI jobs ([`4939732`](https://github.com/Dicklesworthstone/ultrasearch/commit/4939732203092ce82012f225dac8c44d70ff179c))

---

## v0.2.8 — 2025-11-23 (tag only)

- Install GraalVM CE 23 for Windows CI jobs ([`d96685c`](https://github.com/Dicklesworthstone/ultrasearch/commit/d96685c4411c803a6a2a967e5bfd0baa334c0dab))

---

## v0.2.7 — 2025-11-23 (tag only)

- Drop linux/mac CI checks; keep Windows-only pipeline ([`31a0530`](https://github.com/Dicklesworthstone/ultrasearch/commit/31a0530b02c0bb77a9915f8e4ae05bf0469e265a))

---

## v0.2.6 — 2025-11-23 (tag only)

### Bug Fixes
- Fix disk sampler refresh and remove unreachable fallback in scheduler ([`6647675`](https://github.com/Dicklesworthstone/ultrasearch/commit/66476751b5ecd4177da7641d288759f23694dcf9))

---

## v0.2.5 — 2025-11-23 (tag only)

- Install GTK deps for GPUI builds in CI ([`1d2a0cd`](https://github.com/Dicklesworthstone/ultrasearch/commit/1d2a0cdd2bcca9c1ab40ba7fd2e577afeced172f))

---

## v0.2.4 — 2025-11-23 (tag only)

- Install glib deps and simplify CI cache keys ([`33085eb`](https://github.com/Dicklesworthstone/ultrasearch/commit/33085eb22eae917d6d419a2db2e70da14467a1e8))

---

## v0.2.3 — 2025-11-23 (tag only)

- Fix CI cache hash glob ([`be8a82d`](https://github.com/Dicklesworthstone/ultrasearch/commit/be8a82d7500fd2d978b24e0057480224214dacad))

---

## v0.2.2 — 2025-11-23 (tag only)

- Install mold linker for linux CI jobs ([`d9484bb`](https://github.com/Dicklesworthstone/ultrasearch/commit/d9484bb17ed67872ff0a917e5792c0b9d43e0a64))

---

## v0.2.1 — 2025-11-23 (tag only)

### Tests
- Isolate scheduler enqueue dropped counter test ([`d01a652`](https://github.com/Dicklesworthstone/ultrasearch/commit/d01a6521880f1af23e410452dcf0c0d92eb9c35d))

---

## v0.2.0 — 2025-11-23

**First tagged milestone.** Multi-OS CI pipeline, richer UI help, gated MSI packaging attempt.

This tag marks the point where the codebase was considered feature-complete enough for automated builds. It contains the full initial development history from project inception (2025-11-20) through stabilization.

### CI & Build
- Multi-OS CI workflow with fmt, clippy, test, and gated MSI artifact ([`b3b148b`](https://github.com/Dicklesworthstone/ultrasearch/commit/b3b148b423ad723b412f31b2814a4f8f67952420))
- Windows CI workflow for fmt/clippy/tests and MSI artifact ([`b714c03`](https://github.com/Dicklesworthstone/ultrasearch/commit/b714c03262f77d2cf461b3099eb23d57bd4ae627))
- Add Justfile for developer workflow automation ([`ad19f40`](https://github.com/Dicklesworthstone/ultrasearch/commit/ad19f40708a8123ae685ee7740b74185ca4a9a9a))
- Migrate to Rust 2024 edition on nightly with wildcard dependency tracking ([`5c42260`](https://github.com/Dicklesworthstone/ultrasearch/commit/5c4226094b22fad6cef0ce30cc0d57b462c100ee))

### UI (GPUI Desktop Application)
- Modern Desktop Experience: system tray, Spotlight-style quick search (`Alt+Space`), theming ([`d3929e4`](https://github.com/Dicklesworthstone/ultrasearch/commit/d3929e48e096075616efd984a950b7cafa7cb130))
- Help panel with keyboard shortcuts overlay (`F1`, `Ctrl+/`) and feature highlights ([`8908735`](https://github.com/Dicklesworthstone/ultrasearch/commit/89087359c9fab9334800feb2677763256396694e))
- GPUI action system integration with accessibility improvements ([`6578a43`](https://github.com/Dicklesworthstone/ultrasearch/commit/6578a43b511e3bb57c39c590cc12fca6973c907f))
- Results table with sortable columns, keyboard navigation, search debouncing ([`5252710`](https://github.com/Dicklesworthstone/ultrasearch/commit/5252710), [`3ab4e5c`](https://github.com/Dicklesworthstone/ultrasearch/commit/3ab4e5c))
- Search input bar and preview pane with file manager integration ([`f013ca8`](https://github.com/Dicklesworthstone/ultrasearch/commit/f013ca8))
- Update panel with opt-in check, download, restart flow ([`ee757eb`](https://github.com/Dicklesworthstone/ultrasearch/commit/ee757eb48cb697079398c1eba6ea19e64adfe0ae))
- Background jobs, quick search polish, and progress documentation ([`965f6f6`](https://github.com/Dicklesworthstone/ultrasearch/commit/965f6f6ba8a09cd2ee8747de44efc8a5f586b7c3))
- WiX packaging metadata for MSI installer ([`221e7be`](https://github.com/Dicklesworthstone/ultrasearch/commit/221e7be0448cce4cc009d7f15dad773a5ac6bd62))
- Cancelable background tasks and mouse hover affordances ([`c17ceaf`](https://github.com/Dicklesworthstone/ultrasearch/commit/c17ceaff3296df5469ff935d4b7453c1a9ec24b0))
- Preview scrolling and retry accessibility ([`ea1eab8`](https://github.com/Dicklesworthstone/ultrasearch/commit/ea1eab838047e2356cb5bdf9bd3e05bedfd1ea13))
- Onboarding surfaces and status panels ([`eac4c3a`](https://github.com/Dicklesworthstone/ultrasearch/commit/eac4c3a755dd1184b4df536761f1465a60da3e87))

### Service (Windows Background Daemon)
- Native Windows Service management via SCM ([`b6b310b`](https://github.com/Dicklesworthstone/ultrasearch/commit/b6b310bd2d8f47466c23e758865cc8e3c012952b))
- Named pipe IPC server with hardened ACL security ([`c6d2641`](https://github.com/Dicklesworthstone/ultrasearch/commit/c6d2641e11c17ffe6a359c66e3b0c98945c88c92), [`68d1220`](https://github.com/Dicklesworthstone/ultrasearch/commit/68d1220))
- SearchHandler and SchedulerRuntime abstractions ([`4f2d34a`](https://github.com/Dicklesworthstone/ultrasearch/commit/4f2d34a))
- IPC dispatch with search handler and status routing ([`9934c32`](https://github.com/Dicklesworthstone/ultrasearch/commit/9934c32))
- Metrics HTTP server for observability ([`89dc102`](https://github.com/Dicklesworthstone/ultrasearch/commit/89dc102))
- Content queue metrics plumbed into status/IPC responses ([`8e55927`](https://github.com/Dicklesworthstone/ultrasearch/commit/8e5592759b8ac8894554474f3026177e8ef79702))
- Config-aware volume filtering and improved polling watcher ([`a7c0776`](https://github.com/Dicklesworthstone/ultrasearch/commit/a7c0776daada30bf15eb6bb18728ed29f1255523))
- Meta-ingest module for NTFS metadata batch ingestion ([`ac8f25f`](https://github.com/Dicklesworthstone/ultrasearch/commit/ac8f25f))
- Config-driven JSON file logging with hourly/daily rotation ([`3224d71`](https://github.com/Dicklesworthstone/ultrasearch/commit/3224d71), [`829364f`](https://github.com/Dicklesworthstone/ultrasearch/commit/829364f))
- Reloadable configuration support ([`4446f43`](https://github.com/Dicklesworthstone/ultrasearch/commit/4446f43))
- Stub out unused polling watcher to avoid panic risk ([`5fcb73b`](https://github.com/Dicklesworthstone/ultrasearch/commit/5fcb73b90792a6c5ba6405e487a991ab6114df5c))

### Launcher
- One-click runner for service + UI with hidden console ([`3174df1`](https://github.com/Dicklesworthstone/ultrasearch/commit/3174df1682db218c35fe2235f5b9a2f425957a8d))
- Fix env loading and binary path resolution ([`6e0a38e`](https://github.com/Dicklesworthstone/ultrasearch/commit/6e0a38ef4d1c0aad664890231c4bd56e6f386cc4))
- Tolerate slow startup and surface service failures ([`cf58f02`](https://github.com/Dicklesworthstone/ultrasearch/commit/cf58f0257606095e2f0c1d152828762ded00f1a5))

### Installer
- WiX configuration and build script for MSI packaging ([`6e79005`](https://github.com/Dicklesworthstone/ultrasearch/commit/6e79005f711ddb18d278088943a0cc9355187209))

### Scheduler (Idle-Aware Background Indexing)
- Adaptive policy with stateful design integrated into runtime ([`228a5b4`](https://github.com/Dicklesworthstone/ultrasearch/commit/228a5b4), [`bb26d4c`](https://github.com/Dicklesworthstone/ultrasearch/commit/bb26d4c))
- Job queues and runtime loop ([`59909dd`](https://github.com/Dicklesworthstone/ultrasearch/commit/59909dd))
- Real disk I/O monitoring via Windows PDH API (replacing sysinfo) ([`341865d`](https://github.com/Dicklesworthstone/ultrasearch/commit/341865d))
- Idle detection state machine for Windows with platform timer ([`4b1f961`](https://github.com/Dicklesworthstone/ultrasearch/commit/4b1f961))
- Testable job eligibility policies ([`19ef4af`](https://github.com/Dicklesworthstone/ultrasearch/commit/19ef4af))
- Content queue counters surfaced to UI ([`c6bc90c`](https://github.com/Dicklesworthstone/ultrasearch/commit/c6bc90cb51be26c277bba4a6a675da78242096bf))
- Power save mode exposed in config ([`6f26e0e`](https://github.com/Dicklesworthstone/ultrasearch/commit/6f26e0e1982b7e5c5accd1829d90527d0605d015))

### NTFS Watcher & Metadata Index
- MFT enumeration via `usn-journal-rs` for Everything-style filename search ([`f8e0a1d`](https://github.com/Dicklesworthstone/ultrasearch/commit/f8e0a1d))
- Windows volume discovery and handle management ([`1c41503`](https://github.com/Dicklesworthstone/ultrasearch/commit/1c41503))
- USN watcher stub for filesystem change notifications ([`a695e93`](https://github.com/Dicklesworthstone/ultrasearch/commit/a695e93))
- FileMeta to MetaDoc conversion for watcher integration ([`2f8ff23`](https://github.com/Dicklesworthstone/ultrasearch/commit/2f8ff23))
- Metadata cache with path reconstruction ([`f5ae31a`](https://github.com/Dicklesworthstone/ultrasearch/commit/f5ae31a))
- FST (Finite State Transducer) index for fast prefix lookups ([`6e1f5e4`](https://github.com/Dicklesworthstone/ultrasearch/commit/6e1f5e4))

### Content Indexing (Tantivy)
- Tiered content index with Tantivy full-text search ([`e7851c4`](https://github.com/Dicklesworthstone/ultrasearch/commit/e7851c4))
- Batch job processing and content index integration in workers ([`ab7656b`](https://github.com/Dicklesworthstone/ultrasearch/commit/ab7656b))
- MVP CLI for single-file content extraction ([`925aa4f`](https://github.com/Dicklesworthstone/ultrasearch/commit/925aa4f))
- IndexWriter exposed publicly for worker integration ([`a7d9026`](https://github.com/Dicklesworthstone/ultrasearch/commit/a7d9026))

### Content Extraction
- Extractous backend for PDF/Office documents with optional feature flag ([`1306394`](https://github.com/Dicklesworthstone/ultrasearch/commit/1306394), [`1a66f1c`](https://github.com/Dicklesworthstone/ultrasearch/commit/1a66f1c))
- Windows IFilter COM bridge for legacy formats ([`8a6c5aa`](https://github.com/Dicklesworthstone/ultrasearch/commit/8a6c5aa))
- GraalVM CE 23.x build-time enforcement when extractous feature enabled ([`e305f08`](https://github.com/Dicklesworthstone/ultrasearch/commit/e305f0805b040bc72c71c07090e3b67d0ae57869))
- Stack builder methods for flexible backend selection ([`6570207`](https://github.com/Dicklesworthstone/ultrasearch/commit/6570207))

### Semantic Index
- HNSW vector index configured with hnsw_rs for insert/search ([`5fbb6ae`](https://github.com/Dicklesworthstone/ultrasearch/commit/5fbb6ae0138b47f4b48e475bbd18aa57b74e8605))

### IPC Protocol
- Named pipe client with retry/backoff and distributed tracing ([`cfedc91`](https://github.com/Dicklesworthstone/ultrasearch/commit/cfedc91))
- Complete message schema with Duration serialization ([`1e1f98e`](https://github.com/Dicklesworthstone/ultrasearch/commit/1e1f98e), [`f657625`](https://github.com/Dicklesworthstone/ultrasearch/commit/f657625))
- SearchRequest wire format with fluent builder methods ([`78fe1de`](https://github.com/Dicklesworthstone/ultrasearch/commit/78fe1de))
- Self-healing reconnect logic for service restarts and pipe-busy states ([`d1d3969`](https://github.com/Dicklesworthstone/ultrasearch/commit/d1d3969c258f2b83afebbba483b197f0864baa0f))

### CLI
- Full clap-based debug CLI with Search and Status commands ([`b3e7ef5`](https://github.com/Dicklesworthstone/ultrasearch/commit/b3e7ef5))
- JSON output support and formatted result display ([`0844c76`](https://github.com/Dicklesworthstone/ultrasearch/commit/0844c76))
- Content queue metrics in status output ([`db1fdbb`](https://github.com/Dicklesworthstone/ultrasearch/commit/db1fdbbf638f9d98e7e8f27349cf4f3d84abe470))
- Pagination and timeout in IPC request dispatcher ([`af4391d`](https://github.com/Dicklesworthstone/ultrasearch/commit/af4391d))

### Core Types & Serialization
- FileMeta helpers with Display/FromStr implementations ([`dbe40b9`](https://github.com/Dicklesworthstone/ultrasearch/commit/dbe40b9))
- Bincode deserialization with defensive size limits ([`f5b8e2e`](https://github.com/Dicklesworthstone/ultrasearch/commit/f5b8e2e))
- memmap2 support for memory-mapped file access ([`fa56eae`](https://github.com/Dicklesworthstone/ultrasearch/commit/fa56eae))
- rkyv zero-copy serialization with validator lifetime fix ([`cffc672`](https://github.com/Dicklesworthstone/ultrasearch/commit/cffc672))

### Unified Search
- Unified search pipeline combining filename and content results ([`682adca`](https://github.com/Dicklesworthstone/ultrasearch/commit/682adca))

### Testing
- Windows e2e smoke test with opt-in bootstrap hooks ([`960d1e5`](https://github.com/Dicklesworthstone/ultrasearch/commit/960d1e5), [`1629da6`](https://github.com/Dicklesworthstone/ultrasearch/commit/1629da6))
- Mockable scheduler disk sampling ([`1629da6`](https://github.com/Dicklesworthstone/ultrasearch/commit/1629da6))
- Meta-index regression test for document field mapping ([`ffbbd63`](https://github.com/Dicklesworthstone/ultrasearch/commit/ffbbd63))
- Scheduler `sample_duration` clamped to >=1ms ([`3f89131`](https://github.com/Dicklesworthstone/ultrasearch/commit/3f89131aab49233d96b31de2db498b726e090523))

---

## Pre-v0.2.0 — 2025-11-20

Project bootstrapped from scratch with multi-agent development.

### Initial Architecture (16 workspace crates)
| Crate | Purpose |
|-------|---------|
| `core-types` | Shared type system (`FileMeta`, `DocKey`, enums) |
| `core-serialization` | Bincode/rkyv/memmap2 serialization layer |
| `ipc` | Named pipe protocol (messages, client, server) |
| `ntfs-watcher` | MFT enumeration and USN journal tailing |
| `meta-index` | Tantivy metadata index with FST prefix search |
| `content-index` | Tantivy full-text content index |
| `content-extractor` | Extraction backends (Extractous, IFilter, OCR) |
| `semantic-index` | HNSW vector similarity index |
| `scheduler` | Idle-aware job scheduling with adaptive policy |
| `service` | Windows service daemon (SCM, IPC server, metrics) |
| `index-worker` | Short-lived content extraction worker process |
| `cli` | Clap-based command-line interface |
| `ui` | GPUI desktop application |
| `launcher` | One-click service + UI runner |

### First Commits
- Initial project structure and configuration ([`4c0c109`](https://github.com/Dicklesworthstone/ultrasearch/commit/4c0c109bc8137cd478cb40f1a48a713ca9a921f6)) — 2025-11-20
- Pin Rust nightly toolchain and configure optimized build profiles ([`13261ac`](https://github.com/Dicklesworthstone/ultrasearch/commit/13261ac0a65932892eb6149356a604979dd291c0))
- Initialize workspace structure and dependency graph ([`82953b3`](https://github.com/Dicklesworthstone/ultrasearch/commit/82953b364feae9fa08e5444bc0e94028a3ce18e3))
- Core type system and configuration infrastructure ([`d3bd4b2`](https://github.com/Dicklesworthstone/ultrasearch/commit/d3bd4b25744ec72a927888f6ba16461b9454507c))
- NTFS watcher and index subsystem stubs ([`59a0ef4`](https://github.com/Dicklesworthstone/ultrasearch/commit/59a0ef44f1b3c476601f95377ae019ec08a37983))
- Service, worker, and IPC infrastructure stubs ([`addaf5e`](https://github.com/Dicklesworthstone/ultrasearch/commit/addaf5e4d2ae084b2c8e1d0fe4bd99d6b9dfa142))
- UI and CLI application stubs ([`12b9136`](https://github.com/Dicklesworthstone/ultrasearch/commit/12b91361d68c1d8d3d1d08068f42dc1cbcae1c06))

---

## Link Reference

[Unreleased]: https://github.com/Dicklesworthstone/ultrasearch/compare/v1.4.5...HEAD
[v1.4.5]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.37...v1.4.5
[v0.2.22]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.21...v0.2.22
[v0.2.21]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.20...v0.2.21
[v0.2.20]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.19...v0.2.20
[v0.2.19]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.18...v0.2.19
[v0.2.18]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.17...v0.2.18
[v0.2.17]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.16...v0.2.17
[v0.2.16]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.15...v0.2.16
[v0.2.15]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.14...v0.2.15
[v0.2.14]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.13...v0.2.14
[v0.2.13]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.12...v0.2.13
[v0.2.12]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.11...v0.2.12
[v0.2.11]: https://github.com/Dicklesworthstone/ultrasearch/compare/v0.2.0...v0.2.11
