using System.Diagnostics;
using System.Text.Json;

namespace SourisDW;

/// <summary>
/// Main entry point for SourisDW.
/// </summary>
/// <example>
/// <code>
/// var dw = SourisDW.Builder()
///     .Format("mp4")
///     .Quality("1080p")
///     .Output("./downloads")
///     .Build();
///
/// await dw.Download("https://youtube.com/watch?v=xxx").RunAsync();
/// </code>
/// </example>
public class SourisDownloader
{
    private readonly Config _config;

    internal SourisDownloader(Config config)
    {
        _config = config;
    }

    public static SourisDWBuilder Builder() => new();

    public DownloadRequest Download(string url) => new(this, url);
    public DownloadRequest DownloadAudio(string url) => new(this, url, "audio");
    public DownloadRequest DownloadVideo(string url) => new(this, url, "video");
    public DownloadRequest DownloadPlaylist(string url) => new(this, url, "playlist");

    public async Task<JsonElement> InfoAsync(string url)
    {
        var result = await RunProcessAsync("souris-dw", $"info \"{url}\" --json");
        return JsonSerializer.Deserialize<JsonElement>(result);
    }

    public async Task<JsonElement> SearchAsync(string query, int limit = 10)
    {
        var result = await RunProcessAsync("souris-dw", $"search \"{query}\" --json --limit {limit}");
        return JsonSerializer.Deserialize<JsonElement>(result);
    }

    public async Task<JsonElement> UpdateAsync()
    {
        var result = await RunProcessAsync("souris-dw", "update --json");
        return JsonSerializer.Deserialize<JsonElement>(result);
    }

    public async Task<JsonElement> UpdateCheckAsync()
    {
        var result = await RunProcessAsync("souris-dw", "update --json --check");
        return JsonSerializer.Deserialize<JsonElement>(result);
    }

    public Config Config => _config;

    private static async Task<string> RunProcessAsync(string command, string arguments)
    {
        var psi = new ProcessStartInfo
        {
            FileName = command,
            Arguments = arguments,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true,
        };

        using var process = Process.Start(psi);
        if (process == null) throw new Exception("Failed to start process");

        var output = await process.StandardOutput.ReadToEndAsync();
        await process.WaitForExitAsync();

        if (process.ExitCode != 0)
        {
            var error = await process.StandardError.ReadToEndAsync();
            throw new Exception($"Process failed: {error}");
        }

        return output;
    }
}
