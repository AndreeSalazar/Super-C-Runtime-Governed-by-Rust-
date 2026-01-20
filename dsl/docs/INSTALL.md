# SuperC DSL - Guía de Instalación

Instrucciones para instalar y configurar SuperC en Windows.

---

## Requisitos

- **Windows 10/11** (x64)
- **Rust** (para compilar)
- **NASM** (opcional, para ASM)

---

## Instalación Rápida

### 1. Compilar

```powershell
cd dsl
cargo build --release
```

### 2. Instalar globalmente

```powershell
# Opción A: Copiar a System32 (requiere admin)
copy target\release\superc.exe C:\Windows\System32\

# Opción B: Agregar a PATH
$env:Path += ";C:\ruta\a\dsl\target\release"
```

### 3. Verificar

```powershell
superc
```

Debería mostrar:
```
SuperC Compute Engine - Motor de Cómputo Unificado
==================================================
...
```

---

## Instalar Extensión VS Code

### Opción 1: Copiar carpeta

```powershell
xcopy /E /I dsl\vscode-extension\superc-lang %USERPROFILE%\.vscode\extensions\superc-lang
```

Reinicia VS Code.

### Opción 2: Desde VSIX

```powershell
cd dsl\vscode-extension\superc-lang
npm install -g vsce
vsce package
code --install-extension superc-lang-0.1.0.vsix
```

---

## Verificar Instalación

### Test básico

Crea `test.sc`:
```superc
data x: f32
seq {
    x = 42.0
    print(x)
}
```

Ejecuta:
```powershell
superc run test.sc
```

Resultado esperado:
```
🚀 SuperC Compute Engine
========================
📂 Archivo: test.sc
⚙️  Preferencia: Auto
------------------------
42.000000
------------------------
✅ Backend: Pure CPU
⏱️  Tiempo: XX μs
========================
```

---

## Estructura Final

```
dsl/
├── target/release/
│   └── superc.exe          # Ejecutable
├── examples/
│   ├── hello.sc
│   ├── vector_math.sc
│   └── batch_compute.sc
├── docs/
│   ├── SYNTAX.md
│   ├── BACKENDS.md
│   ├── EXAMPLES.md
│   └── INSTALL.md
├── vscode-extension/
│   └── superc-lang/
├── src/                     # Código fuente
└── README.md
```

---

## Desinstalar

```powershell
# Eliminar ejecutable
del C:\Windows\System32\superc.exe

# Eliminar extensión VS Code
rmdir /S %USERPROFILE%\.vscode\extensions\superc-lang
```

---

## Solución de Problemas

### "superc no se reconoce"

El ejecutable no está en PATH. Usa la ruta completa:
```powershell
.\target\release\superc.exe run archivo.sc
```

### "Error de compilación"

Asegúrate de tener Rust instalado:
```powershell
rustup --version
cargo --version
```

### Extensión no funciona

1. Verifica que la carpeta esté en `%USERPROFILE%\.vscode\extensions\`
2. Reinicia VS Code completamente
3. Abre un archivo `.sc`
