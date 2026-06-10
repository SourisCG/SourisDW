package io.souris.sourisdw;

/**
 * Main entry point for SourisDW.
 *
 * <pre>{@code
 * SourisDW dw = SourisDW.builder()
 *     .format("mp4")
 *     .quality("1080p")
 *     .output("./downloads")
 *     .build();
 *
 * dw.download("https://youtube.com/watch?v=xxx").run();
 * }</pre>
 */
public class SourisDW {
    private final Config config;

    SourisDW(Config config) {
        this.config = config;
    }

    public static SourisDWBuilder builder() {
        return new SourisDWBuilder();
    }

    public DownloadRequest download(String url) {
        return new DownloadRequest(this, url, null);
    }

    public DownloadRequest downloadAudio(String url) {
        return new DownloadRequest(this, url, "audio");
    }

    public DownloadRequest downloadVideo(String url) {
        return new DownloadRequest(this, url, "video");
    }

    public DownloadRequest downloadPlaylist(String url) {
        return new DownloadRequest(this, url, "playlist");
    }

    public String info(String url) throws Exception {
        ProcessBuilder pb = new ProcessBuilder("souris-dw", "info", url, "--json");
        Process proc = pb.start();
        String output = new String(proc.getInputStream().readAllBytes());
        proc.waitFor();
        return output;
    }

    public String search(String query, int limit) throws Exception {
        ProcessBuilder pb = new ProcessBuilder(
            "souris-dw", "search", query, "--json",
            "--limit", String.valueOf(limit)
        );
        Process proc = pb.start();
        String output = new String(proc.getInputStream().readAllBytes());
        proc.waitFor();
        return output;
    }

    public String update() throws Exception {
        ProcessBuilder pb = new ProcessBuilder("souris-dw", "update", "--json");
        Process proc = pb.start();
        String output = new String(proc.getInputStream().readAllBytes());
        proc.waitFor();
        return output;
    }

    public Config getConfig() {
        return config;
    }
}
