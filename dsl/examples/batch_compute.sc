// SuperC DSL - Batch Compute Example
// Motor de cómputo unificado para procesamiento masivo
//
// Este ejemplo demuestra:
// - Procesamiento de datos en lote
// - Auto-selección de backend (CPU/GPU/ASM)
// - Operaciones vectoriales optimizadas

// Datos de entrada (simulando batch de 1000 elementos)
data input_a: f32[1000]
data input_b: f32[1000]
data weights: f32[1000]

// Resultados intermedios
data transformed: f32[1000]
data scaled: f32[1000]

// Resultado final
data output: f32[1000]

// Escalares
data total_sum: f32
data batch_mean: f32

// Paso 1: Inicializar datos de entrada
seq {
    for i = 0:1000 {
        input_a[i] = i * 0.1
        input_b[i] = i * 0.05
        weights[i] = 1.0
    }
}

// Paso 2: Transformación paralela (GPU si disponible)
parallel {
    for i = 0:1000 {
        transformed[i] = input_a[i] + input_b[i]
    }
}

// Paso 3: Aplicar pesos (paralelo)
parallel {
    for i = 0:1000 {
        scaled[i] = transformed[i] * weights[i]
    }
}

// Paso 4: Normalización
parallel {
    for i = 0:1000 {
        output[i] = scaled[i] / 1000.0
    }
}

// Paso 5: Reducción - calcular suma total
seq {
    total_sum = reduce(+, output)
    batch_mean = total_sum / 1000.0
    print(total_sum)
    print(batch_mean)
}
