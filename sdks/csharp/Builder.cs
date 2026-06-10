namespace SourisDW;

/// <summary>
/// Builder for configuring SourisDW defaults.
/// </summary>
public class SourisDWBuilder
{
    private readonly Config _config = new();

    public SourisDWBuilder AutoUpdate(bool enabled)
    {
        _config.AutoUpdate = enabled;
        return this;
    }

    public SourisDWBuilder Format(string fmt)
    {
        _config.Format = fmt;
        return this;
    }

    public SourisDWBuilder Quality(string q)
    {
        _config.Quality = q;
        return this;
    }

    public SourisDWBuilder Output(string path)
    {
        _config.Output = path;
        return this;
    }

    public SourisDWBuilder Parallel(int n)
    {
        _config.Parallel = n;
        return this;
    }

    public SourisDWBuilder EmbedMetadata(bool enabled)
    {
        _config.EmbedMetadata = enabled;
        return this;
    }

    public SourisDWBuilder EmbedThumbnail(bool enabled)
    {
        _config.EmbedThumbnail = enabled;
        return this;
    }

    public SourisDWBuilder EmbedSubtitles(bool enabled)
    {
        _config.EmbedSubtitles = enabled;
        return this;
    }

    public SourisDWBuilder Timeout(int seconds)
    {
        _config.Timeout = seconds;
        return this;
    }

    public SourisDWBuilder MaxRetries(int n)
    {
        _config.MaxRetries = n;
        return this;
    }

    public SourisDWBuilder SpotifyCredentials(string clientId, string secret)
    {
        _config.SpotifyClientId = clientId;
        _config.SpotifySecret = secret;
        return this;
    }

    public SourisDownloader Build() => new(_config);
}
