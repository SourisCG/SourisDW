package sourisdw

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os/exec"
)

// ProgressEvent represents a download progress event.
type ProgressEvent struct {
	Type    string  `json:"type"`
	Percent float64 `json:"percent"`
	Speed   string  `json:"speed"`
	Eta     string  `json:"eta"`
	Path    string  `json:"path,omitempty"`
	Size    int64   `json:"size,omitempty"`
	Message string  `json:"message,omitempty"`
}

// DownloadRequest is a fluent download request.
type DownloadRequest struct {
	dw          *SourisDW
	url         string
	mediaType   string
	overrides   map[string]interface{}
	onProgress  func(ProgressEvent)
	onComplete  func(map[string]interface{})
	onError     func(string)
}

// Format overrides the format.
func (r *DownloadRequest) Format(fmt string) *DownloadRequest {
	r.setOverride("format", fmt)
	return r
}

// Quality overrides the quality.
func (r *DownloadRequest) Quality(q string) *DownloadRequest {
	r.setOverride("quality", q)
	return r
}

// Output overrides the output directory.
func (r *DownloadRequest) Output(path string) *DownloadRequest {
	r.setOverride("output", path)
	return r
}

// Parallel overrides the parallel downloads.
func (r *DownloadRequest) Parallel(n int) *DownloadRequest {
	r.setOverride("parallel", n)
	return r
}

// EmbedMetadata overrides metadata embedding.
func (r *DownloadRequest) EmbedMetadata(enabled bool) *DownloadRequest {
	r.setOverride("embed_metadata", enabled)
	return r
}

// EmbedThumbnail overrides thumbnail embedding.
func (r *DownloadRequest) EmbedThumbnail(enabled bool) *DownloadRequest {
	r.setOverride("embed_thumbnail", enabled)
	return r
}

// EmbedSubtitles overrides subtitle embedding.
func (r *DownloadRequest) EmbedSubtitles(enabled bool) *DownloadRequest {
	r.setOverride("embed_subtitles", enabled)
	return r
}

// OnProgress sets the progress callback.
func (r *DownloadRequest) OnProgress(cb func(ProgressEvent)) *DownloadRequest {
	r.onProgress = cb
	return r
}

// OnComplete sets the completion callback.
func (r *DownloadRequest) OnComplete(cb func(map[string]interface{})) *DownloadRequest {
	r.onComplete = cb
	return r
}

// OnError sets the error callback.
func (r *DownloadRequest) OnError(cb func(string)) *DownloadRequest {
	r.onError = cb
	return r
}

// Run executes the download.
func (r *DownloadRequest) Run() (map[string]interface{}, error) {
	args := []string{"download", r.url, "--json"}

	config := r.dw.config
	if v, ok := r.overrides["format"]; ok {
		args = append(args, "--format", v.(string))
	} else if config.Format != "" {
		args = append(args, "--format", config.Format)
	}

	if v, ok := r.overrides["quality"]; ok {
		args = append(args, "--quality", v.(string))
	} else if config.Quality != "" {
		args = append(args, "--quality", config.Quality)
	}

	if v, ok := r.overrides["output"]; ok {
		args = append(args, "--output", v.(string))
	} else if config.Output != "" {
		args = append(args, "--output", config.Output)
	}

	if v, ok := r.overrides["parallel"]; ok {
		args = append(args, "--parallel", fmt.Sprintf("%d", v.(int)))
	} else if config.Parallel > 0 {
		args = append(args, "--parallel", fmt.Sprintf("%d", config.Parallel))
	}

	if config.EmbedMetadata {
		args = append(args, "--embed-metadata")
	}

	if config.EmbedThumbnail {
		args = append(args, "--embed-thumbnail")
	}

	if r.mediaType == "audio" {
		args = append(args, "--audio-only")
	} else if r.mediaType == "video" {
		args = append(args, "--video-only")
	}

	cmd := exec.Command("souris-dw", args...)
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return nil, fmt.Errorf("pipe failed: %w", err)
	}

	if err := cmd.Start(); err != nil {
		return nil, fmt.Errorf("start failed: %w", err)
	}

	result := make(map[string]interface{})
	scanner := bufio.NewScanner(stdout)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}

		var event map[string]interface{}
		if err := json.Unmarshal([]byte(line), &event); err != nil {
			continue
		}

		eventType, _ := event["type"].(string)

		switch eventType {
		case "progress":
			if r.onProgress != nil {
				pe := ProgressEvent{
					Type:    eventType,
					Percent: event["percent"].(float64),
					Speed:   event["speed"].(string),
					Eta:     event["eta"].(string),
				}
				r.onProgress(pe)
			}
		case "complete":
			result = event
			if r.onComplete != nil {
				r.onComplete(event)
			}
		case "error":
			msg, _ := event["message"].(string)
			if r.onError != nil {
				r.onError(msg)
			}
			return nil, fmt.Errorf("download error: %s", msg)
		}
	}

	if err := cmd.Wait(); err != nil {
		return nil, fmt.Errorf("download failed: %w", err)
	}

	return result, nil
}

func (r *DownloadRequest) setOverride(key string, value interface{}) {
	if r.overrides == nil {
		r.overrides = make(map[string]interface{})
	}
	r.overrides[key] = value
}
