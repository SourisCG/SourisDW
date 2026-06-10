package sourisdw

// Builder configures SourisDW defaults.
type Builder struct {
	config Config
}

// AutoUpdate enables/disables auto-update.
func (b *Builder) AutoUpdate(enabled bool) *Builder {
	b.config.AutoUpdate = enabled
	return b
}

// Format sets the default format.
func (b *Builder) Format(fmt string) *Builder {
	b.config.Format = fmt
	return b
}

// Quality sets the default quality.
func (b *Builder) Quality(q string) *Builder {
	b.config.Quality = q
	return b
}

// Output sets the default output directory.
func (b *Builder) Output(path string) *Builder {
	b.config.Output = path
	return b
}

// Parallel sets the number of parallel downloads.
func (b *Builder) Parallel(n int) *Builder {
	b.config.Parallel = n
	return b
}

// EmbedMetadata enables/disables metadata embedding.
func (b *Builder) EmbedMetadata(enabled bool) *Builder {
	b.config.EmbedMetadata = enabled
	return b
}

// EmbedThumbnail enables/disables thumbnail embedding.
func (b *Builder) EmbedThumbnail(enabled bool) *Builder {
	b.config.EmbedThumbnail = enabled
	return b
}

// EmbedSubtitles enables/disables subtitle embedding.
func (b *Builder) EmbedSubtitles(enabled bool) *Builder {
	b.config.EmbedSubtitles = enabled
	return b
}

// Timeout sets the download timeout.
func (b *Builder) Timeout(seconds int) *Builder {
	b.config.Timeout = seconds
	return b
}

// MaxRetries sets the max retries on failure.
func (b *Builder) MaxRetries(n int) *Builder {
	b.config.MaxRetries = n
	return b
}

// SpotifyCredentials sets the Spotify API credentials.
func (b *Builder) SpotifyCredentials(clientID, secret string) *Builder {
	b.config.SpotifyClientID = clientID
	b.config.SpotifySecret = secret
	return b
}

// Build creates the SourisDW instance.
func (b *Builder) Build() *SourisDW {
	return &SourisDW{config: b.config}
}
