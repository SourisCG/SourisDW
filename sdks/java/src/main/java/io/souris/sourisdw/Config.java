package io.souris.sourisdw;

/**
 * Configuration for SourisDW.
 */
public class Config {
    private boolean autoUpdate = true;
    private String format = "mp4";
    private String quality = "1080p";
    private String output = "./downloads";
    private int parallel = 4;
    private boolean embedMetadata = true;
    private boolean embedThumbnail = true;
    private boolean embedSubtitles = false;
    private int timeout = 300;
    private int maxRetries = 3;
    private String spotifyClientId;
    private String spotifySecret;

    public boolean isAutoUpdate() { return autoUpdate; }
    public void setAutoUpdate(boolean autoUpdate) { this.autoUpdate = autoUpdate; }

    public String getFormat() { return format; }
    public void setFormat(String format) { this.format = format; }

    public String getQuality() { return quality; }
    public void setQuality(String quality) { this.quality = quality; }

    public String getOutput() { return output; }
    public void setOutput(String output) { this.output = output; }

    public int getParallel() { return parallel; }
    public void setParallel(int parallel) { this.parallel = parallel; }

    public boolean isEmbedMetadata() { return embedMetadata; }
    public void setEmbedMetadata(boolean embedMetadata) { this.embedMetadata = embedMetadata; }

    public boolean isEmbedThumbnail() { return embedThumbnail; }
    public void setEmbedThumbnail(boolean embedThumbnail) { this.embedThumbnail = embedThumbnail; }

    public boolean isEmbedSubtitles() { return embedSubtitles; }
    public void setEmbedSubtitles(boolean embedSubtitles) { this.embedSubtitles = embedSubtitles; }

    public int getTimeout() { return timeout; }
    public void setTimeout(int timeout) { this.timeout = timeout; }

    public int getMaxRetries() { return maxRetries; }
    public void setMaxRetries(int maxRetries) { this.maxRetries = maxRetries; }

    public String getSpotifyClientId() { return spotifyClientId; }
    public void setSpotifyClientId(String spotifyClientId) { this.spotifyClientId = spotifyClientId; }

    public String getSpotifySecret() { return spotifySecret; }
    public void setSpotifySecret(String spotifySecret) { this.spotifySecret = spotifySecret; }
}
