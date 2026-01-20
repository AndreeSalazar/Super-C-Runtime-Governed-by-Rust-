// SuperC DSL - Ejemplo básico
// Este archivo demuestra la sintaxis simplificada

// Declarar datos
data x: f32[100]
data y: f32[100]
data result: f32[100]

// Inicializar en CPU secuencial
seq {
    for i = 0:100 {
        x[i] = i * 1.0
        y[i] = i * 2.0
    }
}

// Operación paralela (GPU si disponible, sino CPU)
parallel {
    for i = 0:100 {
        result[i] = x[i] + y[i]
    }
}

// Mostrar resultado
seq {
    print(result[0])
    print(result[99])
}
