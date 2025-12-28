# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-12-28

### Added
- Multi-portal batch harvesting via `portals.toml` configuration file
- Delta harvesting with content hash tracking for incremental updates
- Migration tracking to prevent re-execution of applied migrations
- Structured error handling for Gemini API with detailed error information
- `GeminiErrorKind` enum for type-safe error classification (Authentication, RateLimit, QuotaExceeded, ServerError, NetworkError, Unknown)
- `GeminiErrorDetails` struct with error kind, message, and HTTP status code
- `classify_gemini_error()` helper function for centralized error classification
- Sync service layer for cleaner code organization
- Improved test coverage for sync and config modules

### Changed
- **BREAKING**: Refactored Gemini error handling to use structured types instead of strings
  - Replaced `AppError::GeminiError(String)` with `AppError::GeminiError(GeminiErrorDetails)`
- Extracted sync logic into dedicated service layer
- Updated Gemini API request to include API key in headers
- Replaced string parsing in `user_message()` with pattern matching
- Updated `is_retryable()` to intelligently handle different Gemini error types

### Fixed
- Gemini API key now passed in headers per Google API documentation

## [0.0.1] - Initial Release

### Added
- Semantic search engine for CKAN open data portals
- Integration with Google Gemini API for text embeddings
- PostgreSQL database with pgvector extension for vector similarity search
- CLI commands: harvest, search, export, stats
- Support for multiple CKAN portals
- Concurrent dataset processing
- CSV and JSONL export formats