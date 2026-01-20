# SuperC DSL - Backends de Ejecución

Guía de los backends disponibles y cómo seleccionarlos.

---

## Backends Disponibles

### 1. CUDA GPU

**Cuándo:** GPU NVIDIA disponible con drivers CUDA.

```powershell
superc run archivo.sc --gpu
```

**Características:**
- Máximo rendimiento para workloads grandes
- Requiere GPU NVIDIA
- Usa CUDA Toolkit

### 2. HIP GPU

**Cuándo:** GPU AMD disponible con ROCm.

```powershell
superc run archivo.sc --gpu
```

**Características:**
- Para GPUs AMD
- Compatible con código CUDA
- Requiere ROCm instalado

### 3. HIP-CPU (Fallback)

**Cuándo:** No hay GPU disponible pero se solicita `--gpu`.

```powershell
superc run archivo.sc --gpu
```

**Características:**
- Emula GPU en CPU
- Siempre disponible
- Portabilidad total
- Más lento que GPU real

### 4. ASM SIMD

**Cuándo:** Workloads medianos, optimización CPU.

```powershell
superc run archivo.sc --asm
```

**Características:**
- Usa instrucciones AVX/AVX2
- Muy rápido para operaciones vectoriales
- No requiere GPU
- Optimizado para Windows x64

### 5. Pure CPU

**Cuándo:** Workloads pequeños o fallback.

```powershell
superc run archivo.sc --cpu
```

**Características:**
- Siempre disponible
- Sin optimizaciones especiales
- Más portable

---

## Selección Automática

Sin especificar backend, el motor selecciona automáticamente:

```powershell
superc run archivo.sc
```

**Algoritmo de selección:**

| Tamaño Workload | Backend Seleccionado |
|-----------------|---------------------|
| > 100,000 elementos | GPU (si disponible) → ASM |
| 1,000 - 100,000 | ASM SIMD |
| < 1,000 | Pure CPU |

---

## Forzar Backend

### GPU

```powershell
superc run archivo.sc --gpu
```

Orden de preferencia:
1. CUDA GPU (NVIDIA)
2. HIP GPU (AMD)
3. HIP-CPU (fallback)

### ASM

```powershell
superc run archivo.sc --asm
```

Usa instrucciones SIMD optimizadas (AVX).

### CPU

```powershell
superc run archivo.sc --cpu
```

Ejecución secuencial pura.

---

## Verificar Backend Usado

La salida siempre muestra qué backend se usó:

```
🚀 SuperC Compute Engine
========================
📂 Archivo: ejemplo.sc
⚙️  Preferencia: GPU (CUDA/HIP/HIP-CPU)
------------------------
[output]
------------------------
✅ Ejecución completada
📊 Backend usado: HIP-CPU        <-- Aquí
⏱️  Tiempo: 1048 μs
========================
```

---

## Rendimiento por Backend

| Backend | Velocidad Relativa | Uso de Memoria |
|---------|-------------------|----------------|
| CUDA GPU | ⚡⚡⚡⚡⚡ | VRAM |
| HIP GPU | ⚡⚡⚡⚡⚡ | VRAM |
| ASM SIMD | ⚡⚡⚡⚡ | RAM |
| HIP-CPU | ⚡⚡⚡ | RAM |
| Pure CPU | ⚡⚡ | RAM |

---

## Requisitos por Backend

### CUDA GPU
- GPU NVIDIA (Compute Capability 3.5+)
- CUDA Toolkit 11.0+
- Drivers actualizados

### HIP GPU
- GPU AMD (GCN 3+)
- ROCm 4.0+

### ASM SIMD
- CPU con AVX/AVX2
- Windows x64

### HIP-CPU / Pure CPU
- Sin requisitos especiales
- Cualquier CPU x64
