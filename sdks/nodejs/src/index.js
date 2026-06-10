/**
 * SourisDW - Cross-platform music & video downloader for YouTube and Spotify.
 *
 * @example
 * const { SourisDW } = require('souris-dw');
 *
 * const dw = SourisDW.builder()
 *   .format('mp4')
 *   .quality('1080p')
 *   .output('./downloads')
 *   .build();
 *
 * await dw.download('https://youtube.com/watch?v=xxx').run();
 */

const { SourisDWBuilder } = require('./builder');
const { DownloadRequest } = require('./request');
const { SourisError, DependencyError, DownloadError } = require('./exceptions');

class SourisDW {
  constructor(config) {
    this._config = config;
  }

  static builder() {
    return new SourisDWBuilder();
  }

  download(url) {
    return new DownloadRequest(this, url);
  }

  downloadAudio(url) {
    return new DownloadRequest(this, url, 'audio');
  }

  downloadVideo(url) {
    return new DownloadRequest(this, url, 'video');
  }

  downloadPlaylist(url) {
    return new DownloadRequest(this, url, 'playlist');
  }

  async info(url) {
    const { execSync } = require('child_process');
    const result = execSync(`souris-dw info "${url}" --json`, { encoding: 'utf-8' });
    return JSON.parse(result);
  }

  async search(query, limit = 10) {
    const { execSync } = require('child_process');
    const result = execSync(`souris-dw search "${query}" --json --limit ${limit}`, { encoding: 'utf-8' });
    return JSON.parse(result);
  }

  async update() {
    const { execSync } = require('child_process');
    const result = execSync('souris-dw update --json', { encoding: 'utf-8' });
    return JSON.parse(result);
  }

  async updateCheck() {
    const { execSync } = require('child_process');
    const result = execSync('souris-dw update --json --check', { encoding: 'utf-8' });
    return JSON.parse(result);
  }

  get config() {
    return { ...this._config };
  }
}

module.exports = { SourisDW };
