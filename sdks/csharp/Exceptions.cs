namespace SourisDW;

/// <summary>
/// Base exception for SourisDW.
/// </summary>
public class SourisException : Exception
{
    public SourisException(string message) : base(message) { }
    public SourisException(string message, Exception inner) : base(message, inner) { }
}

/// <summary>
/// Error with dependencies.
/// </summary>
public class DependencyException : SourisException
{
    public DependencyException(string message) : base(message) { }
}

/// <summary>
/// Error during download.
/// </summary>
public class DownloadException : SourisException
{
    public DownloadException(string message) : base(message) { }
}

/// <summary>
/// Configuration error.
/// </summary>
public class ConfigException : SourisException
{
    public ConfigException(string message) : base(message) { }
}
