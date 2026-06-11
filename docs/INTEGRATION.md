# Integration Guide

Use SourisDW as a download engine from **any programming language** via subprocess with JSON output.

## How It Works

SourisDW exposes all functionality via its CLI with `--json` output. Any language that can execute a subprocess and parse JSON can use SourisDW.

## Protocol

### Commands

| Command | Description | JSON Output |
|---------|-------------|-------------|
| `souris-dw download <URL> --json` | Download with streaming progress | Progress events + result |
| `souris-dw download <URL> --json -f mp4 -q 1080p` | With format/quality options | Progress events + result |
| `souris-dw info <URL> --json` | Get media info | Media info object |
| `souris-dw search <query> --json` | Search | Array of results |
| `souris-dw update --json` | Update all deps | Status array |
| `souris-dw update --check --json` | Check updates only | Status array |
| `souris-dw deps status --json` | Dep status | Status array |
| `souris-dw deps install --json` | Install/refresh deps | Status array |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success or cancelled by user |
| 1 | General error |
| 2 | Dependency error (missing binary, download failure) |
| 3 | Network error or timeout |

### Progress Events (streamed, one per line)

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"postprocess","item":1,"total":1,"stage":"converting","format":"mp4"}
{"type":"metadata","item":1,"total":1,"stage":"embedding_tags"}
{"type":"complete","item":1,"total":1,"path":"/path/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

### Info Object

```json
{
  "id": "dQw4w9WgXcQ",
  "title": "Video Title",
  "platform": "YouTube",
  "media_type": "video",
  "duration": 213,
  "uploader": "Channel Name",
  "thumbnail": "https://...",
  "formats": [
    {"format_id": "140", "ext": "m4a", "media_type": "audio", "acodec": "aac", "abr": 128},
    {"format_id": "137", "ext": "mp4", "media_type": "video", "vcodec": "h264", "resolution": "1920x1080"}
  ]
}
```

### DepStatus Object

```json
{
  "name": "yt-dlp",
  "installed": true,
  "version": "2024.12.06",
  "path": "/home/user/.local/share/souris-dw/bin/yt-dlp"
}
```

## Python

```python
import subprocess, json

def download(url, format="mp4", quality="1080p"):
    proc = subprocess.Popen(
        ["souris-dw", "download", url, "--json", "--format", format, "--quality", quality],
        stdout=subprocess.PIPE, text=True
    )
    for line in proc.stdout:
        event = json.loads(line.strip())
        if event["type"] == "progress":
            print(f"{event['percent']}%")
        elif event["type"] == "complete":
            return event

result = download("https://youtube.com/watch?v=xxx")
print(f"Downloaded: {result['path']}")
```

## Node.js

```javascript
const { spawn } = require("child_process");

function download(url, format = "mp4") {
  return new Promise((resolve, reject) => {
    const proc = spawn("souris-dw", ["download", url, "--json", "--format", format]);
    proc.stdout.on("data", (data) => {
      const event = JSON.parse(data.toString().trim());
      if (event.type === "progress") console.log(`${event.percent}%`);
      if (event.type === "complete") resolve(event);
    });
    proc.on("close", (code) => { if (code !== 0) reject(new Error("Failed")); });
  });
}

download("https://youtube.com/watch?v=xxx").then(r => console.log("Downloaded:", r.path));
```

## Go

```go
func download(url string) (map[string]interface{}, error) {
    cmd := exec.Command("souris-dw", "download", url, "--json")
    stdout, _ := cmd.StdoutPipe()
    cmd.Start()

    result := make(map[string]interface{})
    scanner := bufio.NewScanner(stdout)
    for scanner.Scan() {
        var event map[string]interface{}
        json.Unmarshal(scanner.Bytes(), &event)
        if event["type"] == "complete" { result = event }
    }
    cmd.Wait()
    return result, nil
}
```

## Java

```java
ProcessBuilder pb = new ProcessBuilder("souris-dw", "download", url, "--json");
Process proc = pb.start();
BufferedReader reader = new BufferedReader(new InputStreamReader(proc.getInputStream()));

String line;
while ((line = reader.readLine()) != null) {
    JsonObject event = JsonParser.parseString(line).getAsJsonObject();
    if ("progress".equals(event.get("type").getAsString())) {
        System.out.println(event.get("percent").getAsDouble() + "%");
    }
}
proc.waitFor();
```

## C#

```csharp
var proc = Process.Start(new ProcessStartInfo {
    FileName = "souris-dw",
    Arguments = $"download {url} --json",
    RedirectStandardOutput = true
});

while (!proc.StandardOutput.EndOfStream) {
    var evt = JsonSerializer.Deserialize<JsonElement>(proc.StandardOutput.ReadLine()!);
    if (evt.GetProperty("type").GetString() == "progress") {
        Console.WriteLine($"{evt.GetProperty("percent").GetDouble()}%");
    }
}
```

## Rust

```rust
use souris_dw::SourisDW;

let dw = SourisDW::builder().build().await?;
dw.download("URL").format("mp3").await?;
```

## Creating Your Own SDK

To create a wrapper in any language:

1. **Builder class** - stores default config in memory
2. **DownloadRequest class** - chainable methods that store overrides
3. **Run method** - builds CLI command and executes subprocess
4. **Progress handler** - parses JSON lines from stdout and calls callbacks

### Template

```python
class SourisDWBuilder:
    def __init__(self):
        self._config = {"format": "mp4", "quality": "1080p", "output": "./downloads"}

    def format(self, f): self._config["format"] = f; return self
    def quality(self, q): self._config["quality"] = q; return self
    def build(self): return SourisDW(self._config)

class DownloadRequest:
    def __init__(self, dw, url):
        self._dw = dw
        self._url = url
        self._overrides = {}

    def format(self, f): self._overrides["format"] = f; return self
    def quality(self, q): self._overrides["quality"] = q; return self

    def run(self):
        config = {**self._dw.config, **self._overrides}
        cmd = ["souris-dw", "download", self._url, "--json",
               "--format", config["format"],
               "--quality", config["quality"]]
        # Execute and parse JSON output...
```
