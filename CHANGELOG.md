# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.4.0] - 2026-10-26
### Added
- New option `--no-persist` to extract into a temporary directory on every run and delete it when the application exits
- Packed binaries accept `--warp-clean` as their only argument to delete their cache directory instead of starting the application
- `--arch` defaults to `auto`, which packs for the platform the packer runs on

### Changed
- The payload is stored as zstd-compressed chunks (level 19) that the runner decompresses in parallel, replacing the gzip-compressed tar archive
- The runner locates the payload and its arguments through a footer at the end of the binary instead of scanning for markers
- Cached extractions are validated with a hash of the payload instead of modification times, and extraction happens atomically under a file lock
- The runner ignores Ctrl+C while the application runs and reports a termination by signal as exit code 128 + signal on Unix
- The runner reads chunks with positional reads instead of memory-mapping its binary, so peak memory during extraction no longer grows with the size of the payload
- `--exec` also accepts the executable as a path from the current directory or as an absolute path, as long as it is inside the input directory
- Running applications hold a shared lock on an `.inuse` file in their cache directory
- Replaced `bincode` (unmaintained, RUSTSEC-2025-0141) with `postcard`
- Updated Rust dependencies
- The CI workflow installs the musl C toolchain needed by `zstd-sys`

### Fixed
- Small files packed from an NTFS drive on Linux were stored as zeros
- Packing over an existing, larger output file left stale bytes at the end of the binary
- `--exec` paths pointing outside the input directory were accepted and produced binaries that could not find their executable
- Lock files were truncated when opened, which can fail on Windows while another process holds the lock

### Removed
- Unused dependency `tempdir`

## [v1.3.0] - 2026-03-20
### Added
- New Option `--hidden` to hide the console window on Windows (useful for GUI applications)

## [v1.2.0] - 2026-03-07
### Added
- Building files with Ziglang and adding a universal file for MacOS

## [v1.1.2] - 2026-03-07
### Changed
- Update dependencies

### Fixed
- Added code signature structure for MacOS
- Update build process and documentation to fix MacOS aarch support
- Update Apple runner in GitHub Workflow

## [v1.1.1] - 2025-08-23
### Changed
- Updated Rust dependencies to latest compatible versions

### Fixed
- Cache path structure corrected for Windows. In v1.1.0 the windows cache was `C:\Users\<user>\AppData\Local\warp\warp\data\packages` instead of `C:\Users\<user>\AppData\Local\warp\packages`

## [v1.1.0] - 2025-05-18
### Added
- Native build support for `aarch64-windows` (Windows ARM64)
- Native build support for `aarch64-macos` (Apple Silicon)
- GitHub Actions workflow to build `warp-runner` and `warp-packer` binaries for all major platforms (Linux, macOS, Windows - x64 and ARM64)
- Documentation in `BUILD.md` for triggering builds via GitHub Actions

### Changed
- Updated Rust dependencies to latest compatible versions
- Revised and tested `BUILD.md` instructions using newer SDKs and Rust toolchains
- Improved and documented usage of the `build.sh` automation script for local builds

## [v1.0.0] - 2024-05-18
### Added
- Support for native execution on aarch64-linux platforms, e.g., Raspberry Pi 4.
- New options for `warp-packer`.
- `BUILD.md` with instructions on how to compile warp-packer.
- `INSTALL.md` with instructions on how to run the application on Linux, macOS, and Windows systems.

### Changed
- Changed the default invocation to `warp-packer pack <argument1> <argument2> ...`.

For more details, see the [README](./README.md#changes-in-v100).

## [v0.1.0 - v0.3.0] - 2018-10-30
### Changed
- For changes see origin repository: https://github.com/dgiagio/warp/releases

[v1.4.0]: https://github.com/palexdev/warp/compare/v1.3.0...v1.4.0
[v1.3.0]: https://github.com/kirbylink/warp/compare/v1.2.0...v1.3.0
[v1.2.0]: https://github.com/kirbylink/warp/compare/v1.1.2...v1.2.0
[v1.1.2]: https://github.com/kirbylink/warp/compare/v1.1.1...v1.1.2
[v1.1.1]: https://github.com/kirbylink/warp/compare/v1.1.0...v1.1.1
[v1.1.0]: https://github.com/kirbylink/warp/compare/v1.0.0...v1.1.0
[v1.0.0]: https://github.com/kirbylink/warp/compare/v0.3.0...v1.0.0
[v0.1.0 - v0.3.0]: https://github.com/dgiagio/warp/releases