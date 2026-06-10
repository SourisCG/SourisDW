/**
 * Exceptions for SourisDW.
 */

class SourisError extends Error {
  constructor(message) {
    super(message);
    this.name = 'SourisError';
  }
}

class DependencyError extends SourisError {
  constructor(message) {
    super(message);
    this.name = 'DependencyError';
  }
}

class DownloadError extends SourisError {
  constructor(message) {
    super(message);
    this.name = 'DownloadError';
  }
}

class ConfigError extends SourisError {
  constructor(message) {
    super(message);
    this.name = 'ConfigError';
  }
}

module.exports = { SourisError, DependencyError, DownloadError, ConfigError };
