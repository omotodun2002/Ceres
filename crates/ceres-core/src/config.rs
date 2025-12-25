//! Configuration types for Ceres components.
//!
//! # Configuration Improvements
//!
//! TODO(config): Make all configuration values environment-configurable
//! Currently all defaults are hardcoded. Should support:
//! - `DB_MAX_CONNECTIONS` for database pool size
//! - `SYNC_CONCURRENCY` for parallel dataset processing
//! - `HTTP_TIMEOUT` for API request timeout
//! - `HTTP_MAX_RETRIES` for retry attempts
//!
//! Consider using the `config` crate for layered configuration:
//! defaults -> config file -> environment variables -> CLI args

use std::time::Duration;

/// Database connection pool configuration.
///
/// TODO(config): Support environment variable `DB_MAX_CONNECTIONS`
/// Default of 5 may be insufficient for high-concurrency scenarios.
pub struct DbConfig {
    pub max_connections: u32,
}

impl Default for DbConfig {
    fn default() -> Self {
        // TODO(config): Read from DB_MAX_CONNECTIONS env var
        Self { max_connections: 5 }
    }
}

/// HTTP client configuration for external API calls.
pub struct HttpConfig {
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_base_delay: Duration,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_base_delay: Duration::from_millis(500),
        }
    }
}

/// Portal synchronization configuration.
///
/// TODO(config): Support CLI arg `--concurrency` and env var `SYNC_CONCURRENCY`
/// Optimal value depends on portal rate limits and system resources.
/// Consider auto-tuning based on API response times.
pub struct SyncConfig {
    pub concurrency: usize,
}

impl Default for SyncConfig {
    fn default() -> Self {
        // TODO(config): Read from SYNC_CONCURRENCY env var
        Self { concurrency: 10 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_defaults() {
        let config = DbConfig::default();
        assert_eq!(config.max_connections, 5);
    }

    #[test]
    fn test_http_config_defaults() {
        let config = HttpConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_base_delay, Duration::from_millis(500));
    }

    #[test]
    fn test_sync_config_defaults() {
        let config = SyncConfig::default();
        assert_eq!(config.concurrency, 10);
    }
}
