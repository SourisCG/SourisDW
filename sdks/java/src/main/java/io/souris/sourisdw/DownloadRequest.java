package io.souris.sourisdw;

import java.io.*;
import java.util.function.Consumer;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

/**
 * Download request with fluent API.
 */
public class DownloadRequest {
    private final SourisDW dw;
    private final String url;
    private final String mediaType;
    private String format;
    private String quality;
    private String output;
    private Integer parallel;
    private Boolean embedMetadata;
    private Boolean embedThumbnail;
    private Boolean embedSubtitles;
    private Consumer<JsonObject> onProgress;
    private Consumer<JsonObject> onComplete;
    private Consumer<String> onError;

    DownloadRequest(SourisDW dw, String url, String mediaType) {
        this.dw = dw;
        this.url = url;
        this.mediaType = mediaType;
    }

    public DownloadRequest format(String fmt) {
        this.format = fmt;
        return this;
    }

    public DownloadRequest quality(String q) {
        this.quality = q;
        return this;
    }

    public DownloadRequest output(String path) {
        this.output = path;
        return this;
    }

    public DownloadRequest parallel(int n) {
        this.parallel = n;
        return this;
    }

    public DownloadRequest embedMetadata(boolean enabled) {
        this.embedMetadata = enabled;
        return this;
    }

    public DownloadRequest embedThumbnail(boolean enabled) {
        this.embedThumbnail = enabled;
        return this;
    }

    public DownloadRequest embedSubtitles(boolean enabled) {
        this.embedSubtitles = enabled;
        return this;
    }

    public DownloadRequest onProgress(Consumer<JsonObject> callback) {
        this.onProgress = callback;
        return this;
    }

    public DownloadRequest onComplete(Consumer<JsonObject> callback) {
        this.onComplete = callback;
        return this;
    }

    public DownloadRequest onError(Consumer<String> callback) {
        this.onError = callback;
        return this;
    }

    public JsonObject run() throws Exception {
        Config config = dw.getConfig();

        ProcessBuilder pb = new ProcessBuilder();
        pb.command().add("souris-dw");
        pb.command().add("download");
        pb.command().add(url);
        pb.command().add("--json");

        String fmt = format != null ? format : config.getFormat();
        if (fmt != null) {
            pb.command().add("--format");
            pb.command().add(fmt);
        }

        String q = quality != null ? quality : config.getQuality();
        if (q != null) {
            pb.command().add("--quality");
            pb.command().add(q);
        }

        String out = output != null ? output : config.getOutput();
        if (out != null) {
            pb.command().add("--output");
            pb.command().add(out);
        }

        int p = parallel != null ? parallel : config.getParallel();
        if (p > 0) {
            pb.command().add("--parallel");
            pb.command().add(String.valueOf(p));
        }

        boolean meta = embedMetadata != null ? embedMetadata : config.isEmbedMetadata();
        if (meta) pb.command().add("--embed-metadata");

        boolean thumb = embedThumbnail != null ? embedThumbnail : config.isEmbedThumbnail();
        if (thumb) pb.command().add("--embed-thumbnail");

        boolean subs = embedSubtitles != null ? embedSubtitles : config.isEmbedSubtitles();
        if (subs) pb.command().add("--embed-subtitles");

        if ("audio".equals(mediaType)) pb.command().add("--audio-only");
        if ("video".equals(mediaType)) pb.command().add("--video-only");

        Process proc = pb.start();
        JsonObject result = new JsonObject();

        BufferedReader reader = new BufferedReader(new InputStreamReader(proc.getInputStream()));
        String line;
        while ((line = reader.readLine()) != null) {
            line = line.trim();
            if (line.isEmpty()) continue;

            try {
                JsonObject event = JsonParser.parseString(line).getAsJsonObject();
                String type = event.has("type") ? event.get("type").getAsString() : "";

                switch (type) {
                    case "progress":
                        if (onProgress != null) onProgress.accept(event);
                        break;
                    case "complete":
                        result = event;
                        if (onComplete != null) onComplete.accept(event);
                        break;
                    case "error":
                        String msg = event.has("message") ? event.get("message").getAsString() : "Unknown error";
                        if (onError != null) onError.accept(msg);
                        throw new Exception("Download error: " + msg);
                }
            } catch (com.google.gson.JsonSyntaxException e) {
                // Skip invalid JSON
            }
        }

        int exitCode = proc.waitFor();
        if (exitCode != 0 && !result.has("type")) {
            String stderr = new String(proc.getErrorStream().readAllBytes());
            throw new Exception("Download failed: " + stderr);
        }

        return result;
    }
}
