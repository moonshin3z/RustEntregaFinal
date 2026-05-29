dijkstra-rust
Implementación del algoritmo de Dijkstra en Rust para encontrar el camino más corto en un grafo ponderado dirigido, usando una red de ciudades de Guatemala como caso de uso.
Estructuras de datos utilizadas
Estructura
Uso
Vec<Vec<(usize, u32)>>
Lista de adyacencia del grafo
BinaryHeap<Reverse<(u32, usize)>>
Cola de prioridad (min-heap)
HashMap<usize, u32>
Distancias mínimas por nodo
HashMap<usize, usize>
Predecesores para reconstruir el camino

Requisitos
Rust (cualquier versión estable)
Cómo ejecutar
git clone https://github.com/moonshin3z/dijkstra-rust

cd dijkstra-rust

cargo run
Casos de prueba
El programa ejecuta automáticamente los siguientes casos al correr:

Caso
Origen
Destino
Descripción
1
Guatemala City
Flores
Ruta larga con múltiples caminos posibles
2
Guatemala City
San Marcos
Ruta con camino alternativo vía Antigua
3
Antigua
Huehuetenango
Ruta desde nodo intermedio
Borde 1
Guatemala City
Guatemala City
Origen igual a destino
Borde 2
Flores
Guatemala City
Nodo inalcanzable (sin aristas salientes)
Borde 3
—
—
Grafo sin aristas

Estructura del proyecto
dijkstra-rust/

├── src/

│   └── main.rs    # algoritmo y casos de prueba

├── Cargo.toml

└── README.md

