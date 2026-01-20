const vscode = require('vscode');

function activate(context) {
    console.log('SuperC Language extension activated');

    // Completion Provider para IntelliSense inteligente
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        'superc',
        {
            provideCompletionItems(document, position) {
                const linePrefix = document.lineAt(position).text.substring(0, position.character);
                const completions = [];

                // Keywords principales
                const keywords = [
                    { label: 'data', detail: 'Declarar variable/array', insertText: new vscode.SnippetString('data ${1:name}: ${2|f32,f64,i32,i64,bool|}[${3:size}]'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'seq', detail: 'Bloque secuencial CPU', insertText: new vscode.SnippetString('seq {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'parallel', detail: 'Bloque paralelo (auto GPU/CPU)', insertText: new vscode.SnippetString('parallel {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'gpu', detail: 'Bloque GPU forzado', insertText: new vscode.SnippetString('gpu {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'asm', detail: 'Bloque ASM SIMD optimizado', insertText: new vscode.SnippetString('asm {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'for', detail: 'Loop for con rango', insertText: new vscode.SnippetString('for ${1:i} = ${2:0}:${3:100} {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'if', detail: 'Condicional if', insertText: new vscode.SnippetString('if ${1:condition} {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'else', detail: 'Bloque else', insertText: new vscode.SnippetString('else {\n\t$0\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'fn', detail: 'Definir función', insertText: new vscode.SnippetString('fn ${1:name}(${2:param}: ${3:f32}) -> ${4:f32} {\n\t$0\n\treturn ${5:result}\n}'), kind: vscode.CompletionItemKind.Keyword },
                    { label: 'return', detail: 'Retornar valor', insertText: new vscode.SnippetString('return ${1:value}'), kind: vscode.CompletionItemKind.Keyword },
                ];

                // Tipos
                const types = [
                    { label: 'f32', detail: 'Float 32-bit', kind: vscode.CompletionItemKind.TypeParameter },
                    { label: 'f64', detail: 'Float 64-bit', kind: vscode.CompletionItemKind.TypeParameter },
                    { label: 'i32', detail: 'Integer 32-bit', kind: vscode.CompletionItemKind.TypeParameter },
                    { label: 'i64', detail: 'Integer 64-bit', kind: vscode.CompletionItemKind.TypeParameter },
                    { label: 'bool', detail: 'Boolean', kind: vscode.CompletionItemKind.TypeParameter },
                ];

                // Funciones built-in
                const functions = [
                    { label: 'print', detail: 'Imprimir valor', insertText: new vscode.SnippetString('print(${1:value})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'sqrt', detail: 'Raíz cuadrada', insertText: new vscode.SnippetString('sqrt(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'sin', detail: 'Seno', insertText: new vscode.SnippetString('sin(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'cos', detail: 'Coseno', insertText: new vscode.SnippetString('cos(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'exp', detail: 'Exponencial', insertText: new vscode.SnippetString('exp(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'log', detail: 'Logaritmo natural', insertText: new vscode.SnippetString('log(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'abs', detail: 'Valor absoluto', insertText: new vscode.SnippetString('abs(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'floor', detail: 'Redondear abajo', insertText: new vscode.SnippetString('floor(${1:x})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'ceil', detail: 'Redondear arriba', insertText: new vscode.SnippetString('ceil(${1:x})'), kind: vscode.CompletionItemKind.Function },
                ];

                // Reducciones
                const reductions = [
                    { label: 'reduce(+, arr)', detail: 'Suma de array', insertText: new vscode.SnippetString('reduce(+, ${1:array})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'reduce(max, arr)', detail: 'Máximo de array', insertText: new vscode.SnippetString('reduce(max, ${1:array})'), kind: vscode.CompletionItemKind.Function },
                    { label: 'reduce(min, arr)', detail: 'Mínimo de array', insertText: new vscode.SnippetString('reduce(min, ${1:array})'), kind: vscode.CompletionItemKind.Function },
                ];

                // Templates completos
                const templates = [
                    { 
                        label: '📦 Template: Vector Add', 
                        detail: 'Suma de vectores completa',
                        insertText: new vscode.SnippetString(
                            '// Vector Addition\n' +
                            'data a: f32[${1:1000}]\n' +
                            'data b: f32[$1]\n' +
                            'data c: f32[$1]\n\n' +
                            'seq {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\ta[i] = i * 1.0\n' +
                            '\t\tb[i] = i * 2.0\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'parallel {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\tc[i] = a[i] + b[i]\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'seq {\n' +
                            '\tprint(c[0])\n' +
                            '}'
                        ),
                        kind: vscode.CompletionItemKind.Snippet 
                    },
                    { 
                        label: '📦 Template: Batch Processing', 
                        detail: 'Pipeline de procesamiento',
                        insertText: new vscode.SnippetString(
                            '// Batch Processing Pipeline\n' +
                            'data input: f32[${1:1000}]\n' +
                            'data output: f32[$1]\n' +
                            'data total: f32\n\n' +
                            'seq {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\tinput[i] = i * 0.1\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'parallel {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\toutput[i] = ${2:input[i] * 2.0}\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'seq {\n' +
                            '\ttotal = reduce(+, output)\n' +
                            '\tprint(total)\n' +
                            '}'
                        ),
                        kind: vscode.CompletionItemKind.Snippet 
                    },
                    { 
                        label: '📦 Template: GPU Compute', 
                        detail: 'Cómputo GPU',
                        insertText: new vscode.SnippetString(
                            '// GPU Compute\n' +
                            'data matrix: f32[${1:10000}]\n' +
                            'data result: f32[$1]\n\n' +
                            'seq {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\tmatrix[i] = i * 0.001\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'gpu {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\tresult[i] = ${2:matrix[i] * matrix[i]}\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'seq {\n' +
                            '\tprint(result[0])\n' +
                            '}'
                        ),
                        kind: vscode.CompletionItemKind.Snippet 
                    },
                    { 
                        label: '📦 Template: Statistics', 
                        detail: 'Estadísticas básicas',
                        insertText: new vscode.SnippetString(
                            '// Statistics\n' +
                            'data values: f32[${1:1000}]\n' +
                            'data sum_val: f32\n' +
                            'data min_val: f32\n' +
                            'data max_val: f32\n' +
                            'data mean_val: f32\n\n' +
                            'seq {\n' +
                            '\tfor i = 0:$1 {\n' +
                            '\t\tvalues[i] = ${2:i * 0.1}\n' +
                            '\t}\n' +
                            '}\n\n' +
                            'seq {\n' +
                            '\tsum_val = reduce(+, values)\n' +
                            '\tmin_val = reduce(min, values)\n' +
                            '\tmax_val = reduce(max, values)\n' +
                            '\tmean_val = sum_val / $1\n' +
                            '\tprint(mean_val)\n' +
                            '}'
                        ),
                        kind: vscode.CompletionItemKind.Snippet 
                    },
                ];

                // Crear CompletionItems
                const createItem = (item) => {
                    const completion = new vscode.CompletionItem(item.label, item.kind);
                    completion.detail = item.detail;
                    if (item.insertText) {
                        completion.insertText = item.insertText;
                    }
                    completion.sortText = '0' + item.label; // Priorizar nuestras sugerencias
                    return completion;
                };

                // Agregar todas las sugerencias
                keywords.forEach(k => completions.push(createItem(k)));
                types.forEach(t => completions.push(createItem(t)));
                functions.forEach(f => completions.push(createItem(f)));
                reductions.forEach(r => completions.push(createItem(r)));
                templates.forEach(t => completions.push(createItem(t)));

                // Buscar variables declaradas en el documento
                const text = document.getText();
                const dataRegex = /data\s+(\w+)\s*:/g;
                let match;
                while ((match = dataRegex.exec(text)) !== null) {
                    const varName = match[1];
                    const varItem = new vscode.CompletionItem(varName, vscode.CompletionItemKind.Variable);
                    varItem.detail = 'Variable declarada';
                    varItem.sortText = '1' + varName;
                    completions.push(varItem);
                }

                return completions;
            }
        },
        '' // Trigger en cualquier caracter
    );

    // Hover Provider para mostrar información
    const hoverProvider = vscode.languages.registerHoverProvider('superc', {
        provideHover(document, position) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);

            const docs = {
                'data': '**data** - Declara una variable o array\n\n```superc\ndata nombre: f32[100]\ndata escalar: i32\n```',
                'seq': '**seq** - Bloque de ejecución secuencial en CPU\n\n```superc\nseq {\n    // código secuencial\n}\n```',
                'parallel': '**parallel** - Bloque paralelo (GPU si disponible, sino CPU)\n\n```superc\nparallel {\n    for i = 0:1000 {\n        arr[i] = i * 2.0\n    }\n}\n```',
                'gpu': '**gpu** - Fuerza ejecución en GPU\n\n```superc\ngpu {\n    // código GPU\n}\n```',
                'asm': '**asm** - Ejecución con ASM SIMD optimizado (AVX)\n\n```superc\nasm {\n    // código optimizado\n}\n```',
                'for': '**for** - Loop con rango\n\n```superc\nfor i = 0:100 {\n    arr[i] = i\n}\n```',
                'reduce': '**reduce** - Operación de reducción\n\n```superc\nsum = reduce(+, array)   // suma\nmax = reduce(max, array) // máximo\nmin = reduce(min, array) // mínimo\n```',
                'f32': '**f32** - Tipo float de 32 bits',
                'f64': '**f64** - Tipo float de 64 bits',
                'i32': '**i32** - Tipo entero de 32 bits',
                'i64': '**i64** - Tipo entero de 64 bits',
                'bool': '**bool** - Tipo booleano',
                'print': '**print(value)** - Imprime un valor',
                'sqrt': '**sqrt(x)** - Raíz cuadrada',
                'sin': '**sin(x)** - Seno',
                'cos': '**cos(x)** - Coseno',
                'exp': '**exp(x)** - Exponencial',
                'log': '**log(x)** - Logaritmo natural',
            };

            if (docs[word]) {
                return new vscode.Hover(new vscode.MarkdownString(docs[word]));
            }
        }
    });

    // Signature Help para funciones
    const signatureProvider = vscode.languages.registerSignatureHelpProvider(
        'superc',
        {
            provideSignatureHelp(document, position) {
                const lineText = document.lineAt(position).text;
                const beforeCursor = lineText.substring(0, position.character);
                
                const signatures = {
                    'print': { label: 'print(value: any)', doc: 'Imprime un valor', params: [{ label: 'value', doc: 'Valor a imprimir' }] },
                    'sqrt': { label: 'sqrt(x: f32) -> f32', doc: 'Raíz cuadrada', params: [{ label: 'x', doc: 'Número positivo' }] },
                    'sin': { label: 'sin(x: f32) -> f32', doc: 'Seno (radianes)', params: [{ label: 'x', doc: 'Ángulo en radianes' }] },
                    'cos': { label: 'cos(x: f32) -> f32', doc: 'Coseno (radianes)', params: [{ label: 'x', doc: 'Ángulo en radianes' }] },
                    'exp': { label: 'exp(x: f32) -> f32', doc: 'Exponencial e^x', params: [{ label: 'x', doc: 'Exponente' }] },
                    'log': { label: 'log(x: f32) -> f32', doc: 'Logaritmo natural', params: [{ label: 'x', doc: 'Número positivo' }] },
                    'reduce': { label: 'reduce(op, array) -> f32', doc: 'Reducción de array', params: [{ label: 'op', doc: '+, max, o min' }, { label: 'array', doc: 'Array a reducir' }] },
                };

                for (const [name, sig] of Object.entries(signatures)) {
                    if (beforeCursor.includes(name + '(')) {
                        const signatureHelp = new vscode.SignatureHelp();
                        const signature = new vscode.SignatureInformation(sig.label, sig.doc);
                        signature.parameters = sig.params.map(p => new vscode.ParameterInformation(p.label, p.doc));
                        signatureHelp.signatures = [signature];
                        signatureHelp.activeSignature = 0;
                        signatureHelp.activeParameter = (beforeCursor.match(/,/g) || []).length;
                        return signatureHelp;
                    }
                }
            }
        },
        '(', ','
    );

    context.subscriptions.push(completionProvider, hoverProvider, signatureProvider);
}

function deactivate() {}

module.exports = { activate, deactivate };
