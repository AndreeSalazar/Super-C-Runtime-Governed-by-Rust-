# SuperC DSL - Ejemplos

Colección de ejemplos prácticos para aprender SuperC.

---

## 1. Hola Mundo

El ejemplo más básico.

```superc
// hello.sc
data x: f32

seq {
    x = 42.0
    print(x)
}
```

```powershell
superc run hello.sc
```

---

## 2. Suma de Vectores

Operación paralela básica.

```superc
// vector_add.sc
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

---

## 3. Producto Escalar

Reducción de arrays.

```superc
// dot_product.sc
data a: f32[1000]
data b: f32[1000]
data temp: f32[1000]
data result: f32

seq {
    for i = 0:1000 {
        a[i] = i * 0.1
        b[i] = i * 0.2
    }
}

parallel {
    for i = 0:1000 {
        temp[i] = a[i] * b[i]
    }
}

seq {
    result = reduce(+, temp)
    print(result)
}
```

---

## 4. Normalización

Escalar un vector.

```superc
// normalize.sc
data vec: f32[100]
data sum: f32
data mean: f32

seq {
    for i = 0:100 {
        vec[i] = i * 1.0
    }
}

seq {
    sum = reduce(+, vec)
    mean = sum / 100.0
}

parallel {
    for i = 0:100 {
        vec[i] = vec[i] - mean
    }
}

seq {
    print(vec[0])
    print(vec[50])
}
```

---

## 5. Estadísticas Básicas

Min, max, suma.

```superc
// stats.sc
data data: f32[500]
data min_val: f32
data max_val: f32
data sum_val: f32
data mean_val: f32

seq {
    for i = 0:500 {
        data[i] = sin(i * 0.1)
    }
}

seq {
    min_val = reduce(min, data)
    max_val = reduce(max, data)
    sum_val = reduce(+, data)
    mean_val = sum_val / 500.0
    
    print(min_val)
    print(max_val)
    print(mean_val)
}
```

---

## 6. Transformación Matemática

Aplicar función a cada elemento.

```superc
// transform.sc
data input: f32[100]
data output: f32[100]

seq {
    for i = 0:100 {
        input[i] = i * 0.1
    }
}

parallel {
    for i = 0:100 {
        output[i] = sqrt(input[i]) * exp(input[i] * 0.01)
    }
}

seq {
    print(output[0])
    print(output[99])
}
```

---

## 7. Procesamiento en Batch

Pipeline de datos.

```superc
// batch.sc
data raw: f32[1000]
data processed: f32[1000]
data scaled: f32[1000]
data final: f32[1000]
data total: f32

// Paso 1: Generar datos
seq {
    for i = 0:1000 {
        raw[i] = i * 0.01
    }
}

// Paso 2: Procesar (paralelo)
parallel {
    for i = 0:1000 {
        processed[i] = raw[i] * raw[i]
    }
}

// Paso 3: Escalar (paralelo)
parallel {
    for i = 0:1000 {
        scaled[i] = processed[i] * 2.5
    }
}

// Paso 4: Normalizar (paralelo)
parallel {
    for i = 0:1000 {
        final[i] = scaled[i] / 1000.0
    }
}

// Paso 5: Reducir
seq {
    total = reduce(+, final)
    print(total)
}
```

---

## 8. Condicionales

Uso de if/else.

```superc
// conditional.sc
data values: f32[100]
data result: f32[100]

seq {
    for i = 0:100 {
        values[i] = i - 50.0
    }
}

seq {
    for i = 0:100 {
        if values[i] > 0 {
            result[i] = sqrt(values[i])
        } else {
            result[i] = 0.0
        }
    }
}

seq {
    print(result[0])
    print(result[50])
    print(result[99])
}
```

---

## 9. ASM Optimizado

Forzar uso de SIMD.

```superc
// simd.sc
data a: f32[10000]
data b: f32[10000]
data c: f32[10000]

seq {
    for i = 0:10000 {
        a[i] = i * 0.001
        b[i] = i * 0.002
    }
}

asm {
    for i = 0:10000 {
        c[i] = a[i] + b[i]
    }
}

seq {
    print(c[0])
    print(c[9999])
}
```

---

## 10. GPU Forzado

Usar HIP-CPU como fallback.

```superc
// gpu_compute.sc
data matrix: f32[10000]
data result: f32[10000]
data sum: f32

seq {
    for i = 0:10000 {
        matrix[i] = i * 0.0001
    }
}

gpu {
    for i = 0:10000 {
        result[i] = matrix[i] * matrix[i]
    }
}

seq {
    sum = reduce(+, result)
    print(sum)
}
```

---

## Ejecutar Ejemplos

```powershell
# Ejecutar con auto-selección
superc run examples/hello.sc

# Ejecutar con GPU
superc run examples/batch.sc --gpu

# Ejecutar con ASM
superc run examples/simd.sc --asm

# Ver código generado
superc emit examples/hello.sc --rust
superc emit examples/hello.sc --c
superc emit examples/hello.sc --asm
```
