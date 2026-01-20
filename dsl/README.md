# SuperC DSL

**Motor de Cómputo Unificado** - Escribe una vez, ejecuta en cualquier backend.

```
.sc → CPU | GPU | ASM (automático)
```

[![Version](https://img.shields.io/badge/version-0.3.0-blue.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2011-lightgrey.svg)]()
[![License](https://img.shields.io/badge/license-MIT-green.svg)]()

---

## 📋 Tabla de Contenidos

1. [Instalación](#-instalación)
2. [Inicio Rápido](#-inicio-rápido)
3. [Sintaxis del Lenguaje](#-sintaxis-del-lenguaje)
4. [Backends de Ejecución](#-backends-de-ejecución)
5. [Extensión VS Code](#-extensión-vs-code)
6. [Ejemplos](#-ejemplos)
7. [API Reference](#-api-reference)
8. [Comparación con Python](#-comparación-con-python)

---

## 🔧 Instalación

### Requisitos
- Windows 10/11 (x64)
- Rust (para compilar)
- VS Code (opcional, para extensión)

### Compilar

```powershell
cd dsl
cargo build --release
```

### Instalar Globalmente (Opcional)

```powershell
# Copiar a PATH (requiere admin)
copy target\release\superc.exe C:\Windows\System32\
```

### Verificar Instalación

```powershell
.\target\release\superc.exe
```

---

## 🚀 Inicio Rápido

### Paso 1: Crear archivo `.sc`

```superc
// mi_programa.sc
data numeros: f32[100]
data resultado: f32

seq {
    for i = 0:100 {
        numeros[i] = i * 2.5
    }
}

seq {
    resultado = reduce(+, numeros)
    print(resultado)
}
```

### Paso 2: Ejecutar

```powershell
.\target\release\superc.exe run mi_programa.sc
```

### Salida

```
🚀 SuperC Compute Engine
========================
📂 Archivo: mi_programa.sc
⚙️  Preferencia: Auto (selección automática)
------------------------
12375.000000
------------------------
✅ Ejecución completada
📊 Backend usado: Pure CPU
⏱️  Tiempo: 150 μs
========================
```

---

## 📝 Sintaxis del Lenguaje

### Declaración de Datos

```superc
// Arrays
data vector: f32[1000]      // 1000 floats
data matriz: i32[500]       // 500 enteros
data flags: bool[100]       // 100 booleanos

// Escalares
data suma: f32
data contador: i32
```

**Tipos disponibles:**

| Tipo | Descripción | Ejemplo |
|------|-------------|---------|
| `f32` | Float 32-bit | `3.14` |
| `f64` | Float 64-bit | `3.14159265` |
| `i32` | Entero 32-bit | `42` |
| `i64` | Entero 64-bit | `9999999999` |
| `bool` | Booleano | `true`, `false` |

### Bloques de Ejecución

```superc
// CPU secuencial
seq {
    // código
}

// Paralelo (auto GPU/CPU)
parallel {
    // código
}

// Forzar GPU
gpu {
    // código
}

// ASM SIMD optimizado
asm {
    // código
}
```

### Control de Flujo

```superc
// Loop for
for i = 0:100 {
    arr[i] = i * 2.0
}

// Condicional
if x > 0 {
    y = sqrt(x)
} else {
    y = 0.0
}
```

### Operaciones

```superc
// Aritméticas
c = a + b
c = a - b
c = a * b
c = a / b
c = a % b      // módulo

// Comparación
if a == b { }
if a != b { }
if a < b { }
if a > b { }
if a <= b { }
if a >= b { }

// Lógicas
if a && b { }  // AND
if a || b { }  // OR
```

### Funciones Matemáticas

```superc
y = sqrt(x)    // Raíz cuadrada
y = sin(x)     // Seno
y = cos(x)     // Coseno
y = exp(x)     // Exponencial
y = log(x)     // Logaritmo natural
y = abs(x)     // Valor absoluto
```

### Reducciones

```superc
suma = reduce(+, array)      // Suma todos los elementos
maximo = reduce(max, array)  // Encuentra el máximo
minimo = reduce(min, array)  // Encuentra el mínimo
```

---

## ⚡ Backends de Ejecución

### Selección Automática

El motor selecciona el mejor backend según el tamaño del workload:

| Tamaño | Backend |
|--------|---------|
| > 100,000 elementos | GPU (si disponible) |
| 1,000 - 100,000 | ASM SIMD |
| < 1,000 | Pure CPU |

### Forzar Backend

```powershell
# Auto-selección
superc run archivo.sc

# Forzar GPU (usa HIP-CPU si no hay GPU)
superc run archivo.sc --gpu

# Forzar ASM SIMD
superc run archivo.sc --asm

# Forzar CPU puro
superc run archivo.sc --cpu
```

### Backends Disponibles

| Backend | Descripción | Requisitos |
|---------|-------------|------------|
| **CUDA GPU** | NVIDIA GPU | CUDA Toolkit |
| **HIP GPU** | AMD GPU | ROCm |
| **HIP-CPU** | Fallback portable | Ninguno |
| **ASM SIMD** | AVX optimizado | CPU moderno |
| **Pure CPU** | CPU básico | Ninguno |

---

## 🎨 Extensión VS Code

### Instalación

1. Abre VS Code
2. Presiona `Ctrl+Shift+P`
3. Escribe: `Extensions: Install from VSIX...`
4. Navega a: `dsl/vscode-extension/superc-lang/`
5. Selecciona: `superc-lang-0.3.0.vsix`
6. Reinicia VS Code

### Características

| Característica | Descripción |
|----------------|-------------|
| **Coloreo de sintaxis** | Resalta keywords, tipos, funciones |
| **IntelliSense** | Autocompletado con `Ctrl+Space` |
| **Snippets** | Templates rápidos con TAB |
| **Hover docs** | Documentación al pasar el mouse |
| **Signature help** | Parámetros de funciones |

### Snippets Disponibles

| Trigger | Resultado |
|---------|-----------|
| `data` | Declarar array |
| `seq` | Bloque secuencial |
| `parallel` | Bloque paralelo |
| `for` | Loop for |
| `if` | Condicional |
| `fn` | Función |
| `batch` | Template pipeline completo |
| `vecadd` | Template suma de vectores |
| `gpucompute` | Template GPU |

---

## 📚 Ejemplos

### 1. Suma de Vectores

```superc
data a: f32[1000]
data b: f32[1000]
data c: f32[1000]

seq {
    for i = 0:1000 {
        a[i] = i * 1.0
        b[i] = i * 2.0
    }
}

parallel {
    for i = 0:1000 {
        c[i] = a[i] + b[i]
    }
}

seq {
    print(c[0])
    print(c[999])
}
```

### 2. Estadísticas

```superc
data valores: f32[500]
data suma: f32
data media: f32
data maximo: f32
data minimo: f32

seq {
    for i = 0:500 {
        valores[i] = sin(i * 0.1)
    }
}

seq {
    suma = reduce(+, valores)
    maximo = reduce(max, valores)
    minimo = reduce(min, valores)
    media = suma / 500.0
    
    print(media)
    print(maximo)
    print(minimo)
}
```

### 3. Procesamiento GPU

```superc
data matriz: f32[10000]
data resultado: f32[10000]

seq {
    for i = 0:10000 {
        matriz[i] = i * 0.001
    }
}

gpu {
    for i = 0:10000 {
        resultado[i] = sqrt(matriz[i]) * exp(matriz[i] * 0.01)
    }
}

seq {
    print(resultado[0])
    print(resultado[9999])
}
```

---

## 📖 API Reference

### Comandos CLI

```powershell
superc run <archivo.sc> [opciones]    # Ejecutar
superc emit <archivo.sc> --rust       # Generar Rust
superc emit <archivo.sc> --c          # Generar C
superc emit <archivo.sc> --asm        # Generar NASM
superc build <archivo.sc>             # Compilar a ejecutable
```

### Opciones de Ejecución

| Opción | Descripción |
|--------|-------------|
| `--gpu` | Preferir GPU (CUDA/HIP/HIP-CPU) |
| `--cpu` | Preferir CPU puro |
| `--asm` | Preferir ASM SIMD |

---

## 🐍 Comparación con Python

### Python (NumPy)

```python
import numpy as np

a = np.zeros(1000, dtype=np.float32)
b = np.zeros(1000, dtype=np.float32)

for i in range(1000):
    a[i] = i * 1.0
    b[i] = i * 2.0

c = a + b
print(c[0], c[999])
```

### SuperC DSL

```superc
data a: f32[1000]
data b: f32[1000]
data c: f32[1000]

seq {
    for i = 0:1000 {
        a[i] = i * 1.0
        b[i] = i * 2.0
    }
}

parallel {
    for i = 0:1000 {
        c[i] = a[i] + b[i]
    }
}

seq {
    print(c[0])
    print(c[999])
}
```

### Ventajas de SuperC

| Aspecto | Python | SuperC |
|---------|--------|--------|
| **GPU automático** | Requiere CuPy/PyTorch | ✅ Automático |
| **ASM optimizado** | No disponible | ✅ Automático |
| **Portabilidad** | Depende de librerías | ✅ Un solo binario |
| **Rendimiento** | Interpretado | ✅ Compilado |
| **Sintaxis** | Verbose | ✅ Minimalista |

---

## 📁 Estructura del Proyecto

```
dsl/
├── target/release/
│   └── superc.exe           # Ejecutable
├── examples/
│   ├── hello.sc             # Ejemplo básico
│   ├── vector_math.sc       # Operaciones vectoriales
│   └── batch_compute.sc     # Procesamiento batch
├── docs/
│   ├── SYNTAX.md            # Referencia de sintaxis
│   ├── BACKENDS.md          # Guía de backends
│   ├── EXAMPLES.md          # Más ejemplos
│   └── INSTALL.md           # Guía de instalación
├── vscode-extension/
│   └── superc-lang/
│       └── superc-lang-0.3.0.vsix
└── src/                     # Código fuente
    ├── lexer.rs
    ├── parser.rs
    ├── ast.rs
    ├── codegen.rs
    ├── codegen_asm.rs
    └── compute.rs
```

---

## 🔗 Enlaces

- [← Volver al proyecto principal](../README.md)
- [Documentación de sintaxis](./docs/SYNTAX.md)
- [Guía de backends](./docs/BACKENDS.md)
- [Más ejemplos](./docs/EXAMPLES.md)

---

## 📄 Licencia

MIT
