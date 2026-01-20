// SuperC DSL - Operaciones vectoriales
// Demuestra operaciones matemáticas paralelas

// Vectores de entrada
data a: f32[1000]
data b: f32[1000]
data c: f32[1000]

// Escalar
data scale: f32

// Resultados
data sum_result: f32
data dot_result: f32

// Inicializar datos
seq {
    scale = 2.5
    for i = 0:1000 {
        a[i] = i * 0.1
        b[i] = i * 0.2
    }
}

// Suma de vectores (paralelo)
parallel {
    for i = 0:1000 {
        c[i] = a[i] + b[i]
    }
}

// Escalar vector
parallel {
    for i = 0:1000 {
        c[i] = c[i] * scale
    }
}

// Reducción (suma total)
seq {
    sum_result = reduce(+, c)
    print(sum_result)
}
