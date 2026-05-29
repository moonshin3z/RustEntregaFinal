dijkstra-rust
Implementación del algoritmo de Dijkstra en Rust para encontrar el camino más corto en un grafo ponderado dirigido, usando una red de ciudades de Guatemala como caso de uso.
Estructuras de datos utilizadas
EstructuraUsoVec<Vec<(usize, u32)>>Lista de adyacencia del grafoBinaryHeap<Reverse<(u32, usize)>>Cola de prioridad (min-heap)HashMap<usize, u32>Distancias mínimas por nodoHashMap<usize, usize>Predecesores para reconstruir el camino
Requisitos

Rust (cualquier versión estable)

Cómo ejecutar
bashgit clone https://github.com/moonshin3z/dijkstra-rust
cd dijkstra-rust
cargo run
Casos de prueba
El programa ejecuta automáticamente los siguientes casos al correr:
CasoOrigenDestinoDescripción1Guatemala CityFloresRuta larga con múltiples caminos posibles2Guatemala CitySan MarcosRuta con camino alternativo vía Antigua3AntiguaHuehuetenangoRuta desde nodo intermedioBorde 1Guatemala CityGuatemala CityOrigen igual a destinoBorde 2FloresGuatemala CityNodo inalcanzable (sin aristas salientes)Borde 3——Grafo sin aristas
Estructura del proyecto
dijkstra-rust/
├── src/
│   └── main.rs    # algoritmo y casos de prueba
├── Cargo.toml
└── README.md
