/**
 * Builder for SourisDW configuration.
 */

class SourisDWBuilder {
  constructor() {
    this._config = {
      autoUpdate: true,
      format: 'mp4',
      quality: '1080p',
      output: './downloads',
      parallel: 4,
      embedMetadata: true,
      embedThumbnail: true,
      embedSubtitles: false,
      timeout: 300,
      maxRetries: 3,
      spotifyClientId: null,
      spotifyClientSecret: null,
    };
  }

  autoUpdate(enabled) {
    this._config.autoUpdate = enabled;
    return this;
  }

  format(fmt) {
    this._config.format = fmt;
    return this;
  }

  quality(q) {
    this._config.quality = q;
    return this;
  }

  output(path) {
    this._config.output = path;
    return this;
  }

  parallel(n) {
    this._config.parallel = n;
    return this;
  }

  embedMetadata(enabled) {
    this._config.embedMetadata = enabled;
    return this;
  }

  embedThumbnail(enabled) {
    this._config.embedThumbnail = enabled;
    return this;
  }

  embedSubtitles(enabled) {
    this._config.embedSubtitles = enabled;
    return this;
  }

  timeout(seconds) {
    this._config.timeout = seconds;
    return this;
  }

  maxRetries(n) {
    this._config.maxRetries = n;
    return this;
  }

  spotifyCredentials(clientId, clientSecret) {
    this._config.spotifyClientId = clientId;
    this._config.spotifyClientSecret = clientSecret;
    return this;
  }

  build() {
    const { SourisDW } = require('./index');
    return new SourisDW(this._config);
  }
}

module.exports = { SourisDWBuilder };
