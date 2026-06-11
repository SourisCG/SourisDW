# Guia de Integracion

Usa SourisDW como motor de descarga desde **cualquier lenguaje de programacion** via subprocess con salida JSON.

## Como Funciona

SourisDW expone toda su funcionalidad via su CLI con salida `--json`. Cualquier lenguaje que pueda ejecutar un subprocess y parsear JSON puede usar SourisDW.

## Comandos

| Comando | Descripcion | Salida JSON |
|---------|-------------|-------------|
| `souris-dw download <URL> --json` | Descargar con progreso | Eventos de progreso + resultado |
| `souris-dw info <URL> --json` | Obtener info del medio | Objeto MediaInfo |
| `souris-dw search <query> --json` | Buscar | Array de resultados |
| `souris-dw update --json` | Actualizar dependencias | Array de estado |
| `souris-dw deps status --json` | Estado de dependencias | Array de estado |
| `souris-dw deps install --json` | Instalar/refrescar | Array de estado |

## Codigos de Salida

| Codigo | Significado |
|--------|-------------|
| 0 | Exito o cancelado por usuario |
| 1 | Error general |
| 2 | Error de dependencia |
| 3 | Error de red o timeout |

## Eventos de Progreso

```json
{"type":"init","url":"...","platform":"youtube","title":"...","media_type":"video","total_items":1}
{"type":"progress","item":1,"total":1,"percent":45.2,"speed":"2.3MB/s","eta":"00:12"}
{"type":"complete","item":1,"total":1,"path":"/path/file.mp4","size":125000000}
{"type":"error","item":1,"total":1,"code":"DOWNLOAD_FAILED","message":"..."}
{"type":"summary","total":10,"success":9,"failed":1,"elapsed":"02:34"}
```

## Python

```python
import subprocess, json

proc = subprocess.Popen(
    ["souris-dw", "download", url, "--json", "--format", "mp4"],
    stdout=subprocess.PIPE, text=True
)
for line in proc.stdout:
    event = json.loads(line.strip())
    if event["type"] == "progress":
        print(f"{event['percent']}%")
    elif event["type"] == "complete":
        print(f"Descargado: {event['path']}")
```

## Node.js

```javascript
const { spawn } = require("child_process");
const proc = spawn("souris-dw", ["download", url, "--json", "--format", "mp4"]);
proc.stdout.on("data", (data) => {
  const event = JSON.parse(data.toString());
  if (event.type === "progress") console.log(`${event.percent}%`);
});
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
```

## Crear tu Propio SDK

Para crear un wrapper en cualquier lenguaje:

1. **Clase Builder** - almacena configuracion por defecto
2. **Clase DownloadRequest** - metodos encadenables que sobrescriben defaults
3. **Metodo run** - construye el comando CLI y ejecuta subprocess
4. **Manejador de progreso** - parsea lineas JSON de stdout
