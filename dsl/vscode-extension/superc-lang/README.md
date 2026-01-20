# SuperC Language Extension

Extensión inteligente de VS Code para archivos `.sc` (SuperC DSL).

## Características

- **Coloreo de sintaxis** completo
- **Autocompletado inteligente** (TAB / Ctrl+Space)
- **Snippets** para estructuras comunes
- **Auto-indentación** y cierre de brackets
- **Folding** de bloques

## Instalación

### Windows (PowerShell como Admin)

```powershell
xcopy /E /I "C:\ruta\al\proyecto\dsl\vscode-extension\superc-lang" "%USERPROFILE%\.vscode\extensions\superc-lang"
```

Reinicia VS Code.

## Snippets Disponibles (TAB / Ctrl+Space)

### Declaraciones
| Trigger | Descripción |
|---------|-------------|
| `data` | Declarar array |
| `scalar` | Declarar variable escalar |

### Bloques de Ejecución
| Trigger | Descripción |
|---------|-------------|
| `seq` | Bloque secuencial |
| `parallel` | Bloque paralelo |
| `gpu` | Bloque GPU |
| `asm` | Bloque ASM SIMD |

### Control de Flujo
| Trigger | Descripción |
|---------|-------------|
| `for` | Loop for |
| `if` | Condicional if |
| `ife` | Condicional if-else |
| `fn` | Definir función |

### Funciones
| Trigger | Descripción |
|---------|-------------|
| `print` | Imprimir valor |
| `sqrt` | Raíz cuadrada |
| `sin` | Seno |
| `cos` | Coseno |
| `exp` | Exponencial |
| `log` | Logaritmo |

### Reducciones
| Trigger | Descripción |
|---------|-------------|
| `sum` | Suma de array |
| `max` | Máximo de array |
| `min` | Mínimo de array |

### Templates Completos
| Trigger | Descripción |
|---------|-------------|
| `vecadd` | Template suma de vectores |
| `batch` | Template procesamiento batch |
| `gpucompute` | Template cómputo GPU |
| `header` | Comentario de cabecera |

## Uso

1. Abre un archivo `.sc`
2. Escribe un trigger (ej: `parallel`)
3. Presiona **TAB** o **Ctrl+Space**
4. Usa **TAB** para navegar entre campos

## Ejemplo

```superc
// Escribe "batch" + TAB para generar:

// Batch Processing Pipeline
data input: f32[1000]
data output: f32[1000]
data total: f32

seq {
    for i = 0:1000 {
        input[i] = i * 0.1
    }
}

parallel {
    for i = 0:1000 {
        output[i] = input[i] * 2.0
    }
}

seq {
    total = reduce(+, output)
    print(total)
}
```

## Coloreo de Sintaxis

- **Palabras clave:** `data`, `fn`, `if`, `else`, `for`, `return`
- **Bloques:** `parallel`, `seq`, `gpu`, `asm`
- **Tipos:** `i32`, `i64`, `f32`, `f64`, `bool`
- **Funciones:** `print`, `sqrt`, `sin`, `cos`, `reduce`
- **Operadores:** `+`, `-`, `*`, `/`, `=`, `==`, `!=`
- **Números y strings**
- **Comentarios:** `// comentario`

## Licencia

MIT
