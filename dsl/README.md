# SuperC DSL - Sintaxis Simplificada

Un lenguaje minimalista que se traduce a **Rust → C → CUDA/HIP/ASM**.

## Filosofía

```
Escribes poco → Ejecutas mucho
```

## Ejemplo Básico

```superc
// Archivo: ejemplo.sc

// Definir datos
data vec_a: f32[1000]
data vec_b: f32[1000]
data result: f32[1000]

// Operación paralela (auto-detecta GPU/CPU)
parallel {
    result = vec_a + vec_b
}

// Operación secuencial
seq {
    print(result[0])
}
```

## Sintaxis

### Tipos de Datos
```superc
data nombre: tipo[tamaño]

// Tipos soportados:
// i32, i64, f32, f64, bool
```

### Bloques de Ejecución
```superc
// Ejecutar en GPU si disponible, sino CPU
parallel { ... }

// Ejecutar en CPU secuencial
seq { ... }

// Forzar GPU (error si no hay)
gpu { ... }

// Forzar CPU con ASM optimizado
asm { ... }
```

### Operaciones
```superc
// Aritméticas (element-wise para arrays)
c = a + b
c = a - b
c = a * b
c = a / b

// Reducciones
sum = reduce(+, array)
max = reduce(max, array)
min = reduce(min, array)

// Funciones matemáticas
b = sqrt(a)
b = sin(a)
b = cos(a)
b = exp(a)
b = log(a)
```

### Funciones
```superc
fn nombre(param: tipo) -> tipo {
    ...
}
```

## Traducción

```
ejemplo.sc  →  Rust (AST)  →  C (código)  →  CUDA/HIP/ASM (ejecución)
```

## Uso

```bash
# Compilar DSL a ejecutable
superc build ejemplo.sc

# Ejecutar directamente
superc run ejemplo.sc

# Ver código generado
superc emit ejemplo.sc --target=rust
superc emit ejemplo.sc --target=c
superc emit ejemplo.sc --target=cuda
```
