using System.Diagnostics;
using System.Text.Json;

namespace SourisDW;

/// <summary>
/// Download request with fluent API.
/// </summary>
public class DownloadRequest
{
    private readonly SourisDownloader _dw;
    private readonly string _url;
    private readonly string? _mediaType;
    private string? _format;
    private string? _quality;
    private string? _output;
    private int? _parallel;
    private bool? _embedMetadata;
    private bool? _embedThumbnail;
    private bool? _embedSubtitles;
    private Action<JsonElement>? _onProgress;
    private Action<JsonElement>? _onComplete;
    private Action<string>? _onError;

    internal DownloadRequest(SourisDownloader dw, string url, string? mediaType = null)
    {
        _dw = dw;
        _url = url;
        _mediaType = mediaType;
    }

    public DownloadRequest Format(string fmt) { _format = fmt; return this; }
    public DownloadRequest Quality(string q) { _quality = q; return this; }
    public DownloadRequest Output(string path) { _output = path; return this; }
    public DownloadRequest Parallel(int n) { _parallel = n; return this; }
    public DownloadRequest EmbedMetadata(bool enabled) { _embedMetadata = enabled; return this; }
    public DownloadRequest EmbedThumbnail(bool enabled) { _embedThumbnail = enabled; return this; }
    public DownloadRequest EmbedSubtitles(bool enabled) { _embedSubtitles = enabled; return this; }
    public DownloadRequest OnProgress(Action<JsonElement> callback) { _onProgress = callback; return this; }
    public DownloadRequest OnComplete(Action<JsonElement> callback) { _onComplete = callback; return this; }
    public DownloadRequest OnError(Action<string> callback) { _onError = callback; return this; }

    public async Task<JsonElement> RunAsync()
    {
        var config = _dw.Config;
        var args = new List<string> { "download", _url, "--json" };

        var fmt = _format ?? config.Format;
        if (fmt != null) { args.Add("--format"); args.Add(fmt); }

        var q = _quality ?? config.Quality;
        if (q != null) { args.Add("--quality"); args.Add(q); }

        var outDir = _output ?? config.Output;
        if (outDir != null) { args.Add("--output"); args.Add(outDir); }

        var p = _parallel ?? config.Parallel;
        if (p > 0) { args.Add("--parallel"); args.Add(p.ToString()); }

        var meta = _embedMetadata ?? config.EmbedMetadata;
        if (meta) args.Add("--embed-metadata");

        var thumb = _embedThumbnail ?? config.EmbedThumbnail;
        if (thumb) args.Add("--embed-thumbnail");

        var subs = _embedSubtitles ?? config.EmbedSubtitles;
        if (subs) args.Add("--embed-subtitles");

        if (_mediaType == "audio") args.Add("--audio-only");
        if (_mediaType == "video") args.Add("--video-only");

        var psi = new ProcessStartInfo
        {
            FileName = "souris-dw",
            Arguments = string.Join(" ", args),
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true,
        };

        using var process = Process.Start(psi);
        if (process == null) throw new Exception("Failed to start process");

        var result = new JsonElement();

        while (await process.StandardOutput.ReadLineAsync() is { } line)
        {
            line = line.Trim();
            if (string.IsNullOrEmpty(line)) continue;

            try
            {
                var doc = JsonDocument.Parse(line);
                var root = doc.RootElement;
                var type = root.GetProperty("type").GetString();

                switch (type)
                {
                    case "progress":
                        _onProgress?.Invoke(root);
                        break;
                    case "complete":
                        result = root;
                        _onComplete?.Invoke(root);
                        break;
                    case "error":
                        var msg = root.GetProperty("message").GetString() ?? "Unknown error";
                        _onError?.Invoke(msg);
                        throw new Exception($"Download error: {msg}");
                }
            }
            catch (JsonException)
            {
                // Skip invalid JSON
            }
        }

        await process.WaitForExitAsync();

        if (process.ExitCode != 0 && result.ValueKind == JsonValueKind.Undefined)
        {
            var error = await process.StandardError.ReadToEndAsync();
            throw new Exception($"Download failed: {error}");
        }

        return result;
    }
}
