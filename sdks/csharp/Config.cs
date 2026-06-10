namespace SourisDW;

/// <summary>
/// Configuration for SourisDW.
/// </summary>
public class Config
{
    public bool AutoUpdate { get; set; } = true;
    public string Format { get; set; } = "mp4";
    public string Quality { get; set; } = "1080p";
    public string Output { get; set; } = "./downloads";
    public int Parallel { get; set; } = 4;
    public bool EmbedMetadata { get; set; } = true;
    public bool EmbedThumbnail { get; set; } = true;
    public bool EmbedSubtitles { get; set; } = false;
    public int Timeout { get; set; } = 300;
    public int MaxRetries { get; set; } = 3;
    public string? SpotifyClientId { get; set; }
    public string? SpotifySecret { get; set; }
}
