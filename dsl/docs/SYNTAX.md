# SuperC DSL - Referencia de Sintaxis

Guía completa de la sintaxis del lenguaje SuperC.

---

## Declaración de Datos

### Arrays

```superc
data nombre: tipo[tamaño]
```

Ejemplos:
```superc
data vector: f32[1000]      // Array de 1000 floats
data matriz: f64[100]       // Array de 100 doubles
data flags: bool[50]        // Array de 50 booleanos
data indices: i32[256]      // Array de 256 enteros
```

### Escalares

```superc
data nombre: tipo
```

Ejemplos:
```superc
data suma: f32              // Float escalar
data contador: i32          // Entero escalar
data activo: bool           // Booleano escalar
```

### Tipos Disponibles

| Tipo | Descripción | Tamaño |
|------|-------------|--------|
| `i32` | Entero con signo | 32 bits |
| `i64` | Entero con signo | 64 bits |
| `f32` | Punto flotante | 32 bits |
| `f64` | Punto flotante | 64 bits |
| `bool` | Booleano | 1 byte |

---

## Bloques de Ejecución

### `seq` - Secuencial

Ejecuta código en CPU de forma secuencial.

```superc
seq {
    // Código secuencial
}
```

### `parallel` - Paralelo

Ejecuta en GPU si está disponible, sino en CPU.

```superc
parallel {
    // Código paralelo
}
```

### `gpu` - Forzar GPU

Fuerza ejecución en GPU (error si no hay GPU).

```superc
gpu {
    // Código GPU obligatorio
}
```

### `asm` - ASM Optimizado

Ejecuta con instrucciones SIMD optimizadas (AVX).

```superc
asm {
    // Código ASM optimizado
}
```

---

## Operadores

### Aritméticos

| Operador | Descripción |
|----------|-------------|
| `+` | Suma |
| `-` | Resta |
| `*` | Multiplicación |
| `/` | División |
| `%` | Módulo |

```superc
c = a + b
c = a - b
c = a * b
c = a / b
c = a % b
```

### Comparación

| Operador | Descripción |
|----------|-------------|
| `==` | Igual |
| `!=` | Diferente |
| `<` | Menor que |
| `>` | Mayor que |
| `<=` | Menor o igual |
| `>=` | Mayor o igual |

### Lógicos

| Operador | Descripción |
|----------|-------------|
| `&&` | AND lógico |
| `\|\|` | OR lógico |
| `!` | NOT lógico |

---

## Control de Flujo

### For Loop

```superc
for variable = inicio:fin {
    // código
}
```

Ejemplo:
```superc
for i = 0:100 {
    arr[i] = i * 2.0
}
```

### If/Else

```superc
if condición {
    // si verdadero
} else {
    // si falso
}
```

Ejemplo:
```superc
if x > 0 {
    y = sqrt(x)
} else {
    y = 0.0
}
```

---

## Funciones Matemáticas

| Función | Descripción |
|---------|-------------|
| `sqrt(x)` | Raíz cuadrada |
| `sin(x)` | Seno |
| `cos(x)` | Coseno |
| `exp(x)` | Exponencial |
| `log(x)` | Logaritmo natural |

```superc
y = sqrt(x)
y = sin(x)
y = cos(x)
y = exp(x)
y = log(x)
```

---

## Reducciones

Operaciones que reducen un array a un escalar.

```superc
suma = reduce(+, array)      // Suma todos los elementos
maximo = reduce(max, array)  // Encuentra el máximo
minimo = reduce(min, array)  // Encuentra el mínimo
```

---

## Funciones de Usuario

```superc
fn nombre(param1: tipo, param2: tipo) -> tipo_retorno {
    // código
    return valor
}
```

Ejemplo:
```superc
fn distancia(x: f32, y: f32) -> f32 {
    return sqrt(x * x + y * y)
}
```

---

## Comentarios

```superc
// Esto es un comentario de línea
```

---

## Ejemplo Completo

```superc
// Procesamiento de batch
data input: f32[1000]
data output: f32[1000]
data total: f32

// Inicializar
seq {
    for i = 0:1000 {
        input[i] = i * 0.1
    }
}

// Procesar en paralelo
parallel {
    for i = 0:1000 {
        output[i] = sqrt(input[i]) * 2.0
    }
}

// Reducir
seq {
    total = reduce(+, output)
    print(total)
}
```
