/**
 * Download request with fluent API.
 */

const { spawn } = require('child_process');

class ProgressEvent {
  constructor(data) {
    this._data = data;
  }

  get type() { return this._data.type || ''; }
  get percent() { return this._data.percent || 0; }
  get speed() { return this._data.speed || ''; }
  get eta() { return this._data.eta || ''; }
  get path() { return this._data.path || null; }
  get size() { return this._data.size || null; }
  get message() { return this._data.message || ''; }
}

class DownloadRequest {
  constructor(dw, url, mediaType = null) {
    this._dw = dw;
    this._url = url;
    this._mediaType = mediaType;
    this._overrides = {};
    this._onProgress = null;
    this._onComplete = null;
    this._onError = null;
  }

  format(fmt) {
    this._overrides.format = fmt;
    return this;
  }

  quality(q) {
    this._overrides.quality = q;
    return this;
  }

  output(path) {
    this._overrides.output = path;
    return this;
  }

  parallel(n) {
    this._overrides.parallel = n;
    return this;
  }

  embedMetadata(enabled) {
    this._overrides.embedMetadata = enabled;
    return this;
  }

  embedThumbnail(enabled) {
    this._overrides.embedThumbnail = enabled;
    return this;
  }

  embedSubtitles(enabled) {
    this._overrides.embedSubtitles = enabled;
    return this;
  }

  onProgress(callback) {
    this._onProgress = callback;
    return this;
  }

  onComplete(callback) {
    this._onComplete = callback;
    return this;
  }

  onError(callback) {
    this._onError = callback;
    return this;
  }

  run() {
    return new Promise((resolve, reject) => {
      const config = { ...this._dw.config, ...this._overrides };
      const args = ['download', this._url, '--json'];

      if (config.format) args.push('--format', config.format);
      if (config.quality) args.push('--quality', config.quality);
      if (config.output) args.push('--output', config.output);
      if (config.parallel) args.push('--parallel', String(config.parallel));
      if (config.embedMetadata) args.push('--embed-metadata');
      if (config.embedThumbnail) args.push('--embed-thumbnail');
      if (config.embedSubtitles) args.push('--embed-subtitles');
      if (this._mediaType === 'audio') args.push('--audio-only');
      if (this._mediaType === 'video') args.push('--video-only');

      const proc = spawn('souris-dw', args);
      let result = {};

      proc.stdout.on('data', (data) => {
        const lines = data.toString().split('\n').filter(l => l.trim());
        for (const line of lines) {
          try {
            const event = JSON.parse(line);
            const progress = new ProgressEvent(event);

            if (event.type === 'progress' && this._onProgress) {
              this._onProgress(progress);
            } else if (event.type === 'complete') {
              result = event;
              if (this._onComplete) this._onComplete(event);
            } else if (event.type === 'error') {
              const msg = event.message || 'Unknown error';
              if (this._onError) this._onError(msg);
              reject(new Error(msg));
              return;
            }
          } catch (e) {
            // Skip invalid JSON
          }
        }
      });

      proc.stderr.on('data', (data) => {
        // Collect stderr for error messages
      });

      proc.on('close', (code) => {
        if (code !== 0 && !result.type) {
          reject(new Error(`Download failed with code ${code}`));
        } else {
          resolve(result);
        }
      });
    });
  }
}

module.exports = { DownloadRequest, ProgressEvent };
