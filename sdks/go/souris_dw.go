// Package sourisdw provides a Go SDK for SourisDW.
//
// Usage:
//
//	dw, err := sourisdw.NewBuilder().
//	    Format("mp4").
//	    Quality("1080p").
//	    Output("./downloads").
//	    Build()
//
//	result, err := dw.Download("https://youtube.com/watch?v=xxx").Run()
package sourisdw

import (
	"encoding/json"
	"fmt"
	"os/exec"
)

// SourisDW is the main entry point.
type SourisDW struct {
	config Config
}

// Config holds the configuration.
type Config struct {
	AutoUpdate      bool   `json:"auto_update"`
	Format          string `json:"format"`
	Quality         string `json:"quality"`
	Output          string `json:"output"`
	Parallel        int    `json:"parallel"`
	EmbedMetadata   bool   `json:"embed_metadata"`
	EmbedThumbnail  bool   `json:"embed_thumbnail"`
	EmbedSubtitles  bool   `json:"embed_subtitles"`
	Timeout         int    `json:"timeout"`
	MaxRetries      int    `json:"max_retries"`
	SpotifyClientID string `json:"spotify_client_id,omitempty"`
	SpotifySecret   string `json:"spotify_client_secret,omitempty"`
}

// NewBuilder creates a new builder.
func NewBuilder() *Builder {
	return &Builder{
		config: Config{
			AutoUpdate:     true,
			Format:         "mp4",
			Quality:        "1080p",
			Output:         "./downloads",
			Parallel:       4,
			EmbedMetadata:  true,
			EmbedThumbnail: true,
			Timeout:        300,
			MaxRetries:     3,
		},
	}
}

// Download creates a download request.
func (s *SourisDW) Download(url string) *DownloadRequest {
	return &DownloadRequest{
		dw:  s,
		url: url,
	}
}

// DownloadAudio creates an audio download request.
func (s *SourisDW) DownloadAudio(url string) *DownloadRequest {
	return &DownloadRequest{
		dw:        s,
		url:       url,
		mediaType: "audio",
	}
}

// DownloadVideo creates a video download request.
func (s *SourisDW) DownloadVideo(url string) *DownloadRequest {
	return &DownloadRequest{
		dw:        s,
		url:       url,
		mediaType: "video",
	}
}

// DownloadPlaylist creates a playlist download request.
func (s *SourisDW) DownloadPlaylist(url string) *DownloadRequest {
	return &DownloadRequest{
		dw:        s,
		url:       url,
		mediaType: "playlist",
	}
}

// Info gets media info without downloading.
func (s *SourisDW) Info(url string) (map[string]interface{}, error) {
	cmd := exec.Command("souris-dw", "info", url, "--json")
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("info failed: %w", err)
	}

	var result map[string]interface{}
	if err := json.Unmarshal(output, &result); err != nil {
		return nil, fmt.Errorf("parse failed: %w", err)
	}

	return result, nil
}

// Search searches for media.
func (s *SourisDW) Search(query string, limit int) ([]map[string]interface{}, error) {
	cmd := exec.Command("souris-dw", "search", query, "--json", "--limit", fmt.Sprintf("%d", limit))
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("search failed: %w", err)
	}

	var result []map[string]interface{}
	if err := json.Unmarshal(output, &result); err != nil {
		return nil, fmt.Errorf("parse failed: %w", err)
	}

	return result, nil
}

// Config returns the configuration.
func (s *SourisDW) Config() Config {
	return s.config
}
