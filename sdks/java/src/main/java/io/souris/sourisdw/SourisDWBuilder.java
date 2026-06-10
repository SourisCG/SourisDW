package io.souris.sourisdw;

/**
 * Builder for configuring SourisDW defaults.
 */
public class SourisDWBuilder {
    private final Config config = new Config();

    public SourisDWBuilder autoUpdate(boolean enabled) {
        config.setAutoUpdate(enabled);
        return this;
    }

    public SourisDWBuilder format(String fmt) {
        config.setFormat(fmt);
        return this;
    }

    public SourisDWBuilder quality(String q) {
        config.setQuality(q);
        return this;
    }

    public SourisDWBuilder output(String path) {
        config.setOutput(path);
        return this;
    }

    public SourisDWBuilder parallel(int n) {
        config.setParallel(n);
        return this;
    }

    public SourisDWBuilder embedMetadata(boolean enabled) {
        config.setEmbedMetadata(enabled);
        return this;
    }

    public SourisDWBuilder embedThumbnail(boolean enabled) {
        config.setEmbedThumbnail(enabled);
        return this;
    }

    public SourisDWBuilder embedSubtitles(boolean enabled) {
        config.setEmbedSubtitles(enabled);
        return this;
    }

    public SourisDWBuilder timeout(int seconds) {
        config.setTimeout(seconds);
        return this;
    }

    public SourisDWBuilder maxRetries(int n) {
        config.setMaxRetries(n);
        return this;
    }

    public SourisDWBuilder spotifyCredentials(String clientId, String secret) {
        config.setSpotifyClientId(clientId);
        config.setSpotifySecret(secret);
        return this;
    }

    public SourisDW build() {
        return new SourisDW(config);
    }
}
