# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/SecurityRonin/ad1-forensic/compare/ad1-core-v0.1.0...ad1-core-v0.1.1) - 2026-07-23

### Added

- *(vfs)* GREEN — impl FileSystem for Ad1Vfs

### Fixed

- *(deps)* widen stale caret requirements to published versions (safe-read 0.2, forensic-vfs 0.7)

### Other

- rename forensic-bytes dependency to safe-read
- *(ad1-core)* use forensic-bytes for bounded byte reads
