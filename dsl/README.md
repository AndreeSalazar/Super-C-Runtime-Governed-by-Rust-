# SuperC DSL

**Lenguaje de cómputo unificado** - Escribe una vez, ejecuta en cualquier backend.

```
.sc → CPU | GPU | ASM (automático)
```

---

## Instalación

```powershell
# Windows
cd dsl
cargo build --release
copy target\release\superc.exe C:\Windows\System32\
```

Ahora puedes usar `superc` desde cualquier lugar.

---

## Inicio Rápido

### 1. Crear archivo `ejemplo.sc`

```superc
data a: f32[100]
data b: f32[100]
data c: f32[100]

seq {
    for i = 0:100 {
        a[i] = i * 1.0
        b[i] = i * 2.0
    }
}

parallel {
    for i = 0:100 {
        c[i] = a[i] + b[i]
    }
}

seq {
    print(c[0])
    print(c[99])
}
```

### 2. Ejecutar

```powershell
superc run ejemplo.sc
```

Salida:
```
🚀 SuperC Compute Engine
========================
📂 Archivo: ejemplo.sc
⚙️  Preferencia: Auto
------------------------
0.000000
297.000000
------------------------
✅ Backend: Pure CPU
⏱️  Tiempo: 250 μs
========================
```

---

## Comandos

| Comando | Descripción |
|---------|-------------|
| `superc run archivo.sc` | Ejecuta directamente |
| `superc run archivo.sc --gpu` | Forzar GPU/HIP-CPU |
| `superc run archivo.sc --asm` | Forzar ASM SIMD |
| `superc run archivo.sc --cpu` | Forzar CPU puro |
| `superc emit archivo.sc --rust` | Ver código Rust |
| `superc emit archivo.sc --c` | Ver código C |
| `superc emit archivo.sc --asm` | Ver código NASM |

---

## Sintaxis

### Datos

```superc
data nombre: tipo[tamaño]
data escalar: tipo
```

**Tipos:** `i32`, `i64`, `f32`, `f64`, `bool`

### Bloques de Ejecución

```superc
seq { }        // CPU secuencial
parallel { }   // Auto GPU/CPU
gpu { }        // Forzar GPU
asm { }        // ASM optimizado
```

### Operaciones

```superc
c = a + b      // Suma
c = a - b      // Resta
c = a * b      // Multiplicación
c = a / b      // División

sum = reduce(+, arr)    // Suma total
max = reduce(max, arr)  // Máximo
min = reduce(min, arr)  // Mínimo
```

### Control de Flujo

```superc
for i = 0:100 {
    // código
}

if x > 0 {
    // código
} else {
    // código
}
```

### Funciones Matemáticas

```superc
y = sqrt(x)
y = sin(x)
y = cos(x)
y = exp(x)
y = log(x)
```

---

## Backends

| Backend | Cuándo se usa |
|---------|---------------|
| **CUDA GPU** | NVIDIA disponible |
| **HIP GPU** | AMD disponible |
| **HIP-CPU** | Fallback GPU portable |
| **ASM SIMD** | Workloads medianos |
| **Pure CPU** | Workloads pequeños |

El motor selecciona automáticamente el mejor backend.

---

## Estructura del Proyecto

```
dsl/
├── superc.exe          # Ejecutable
├── examples/           # Ejemplos .sc
│   ├── hello.sc
│   ├── vector_math.sc
│   └── batch_compute.sc
├── docs/               # Documentación
│   ├── SYNTAX.md
│   ├── BACKENDS.md
│   └── EXAMPLES.md
└── vscode-extension/   # Extensión VS Code
    └── superc-lang/
```

---

## Extensión VS Code

Instala la extensión para colorear sintaxis:

```powershell
cd dsl/vscode-extension/superc-lang
code --install-extension superc-lang-0.1.0.vsix
```

---

## Licencia

MIT
