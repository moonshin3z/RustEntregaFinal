fn main() {
    // Grafo: red de ciudades de Guatemala (distancias aproximadas en km)
    //
    //  0 = Guatemala City
    //  1 = Antigua
    //  2 = Escuintla
    //  3 = Quetzaltenango
    //  4 = Huehuetenango
    //  5 = Cobán
    //  6 = Puerto Barrios
    //  7 = Flores
    //  8 = San Marcos
    let nombres = [
        "Guatemala City", "Antigua", "Escuintla",
        "Quetzaltenango", "Huehuetenango", "Cobán",
        "Puerto Barrios", "Flores", "San Marcos",
    ];

    let mut grafo: Grafo = vec![vec![]; nombres.len()];

    // Guatemala City → vecinos
    grafo[0].push((1, 45));   // → Antigua
    grafo[0].push((2, 60));   // → Escuintla
    grafo[0].push((5, 215));  // → Cobán
    grafo[0].push((6, 295));  // → Puerto Barrios

    // Antigua → vecinos
    grafo[1].push((2, 55));   // → Escuintla
    grafo[1].push((3, 180));  // → Quetzaltenango

    // Escuintla → vecinos
    grafo[2].push((3, 170));  // → Quetzaltenango

    // Quetzaltenango → vecinos
    grafo[3].push((4, 95));   // → Huehuetenango
    grafo[3].push((8, 45));   // → San Marcos

    // Huehuetenango → vecinos
    grafo[4].push((8, 70));   // → San Marcos

    // Cobán → vecinos
    grafo[5].push((6, 380));  // → Puerto Barrios
    grafo[5].push((7, 360));  // → Flores

    // Puerto Barrios → vecinos
    grafo[6].push((7, 430));  // → Flores

    // San Marcos sin aristas salientes (terminal)
    // Flores sin aristas salientes (terminal)

    println!("=== Algoritmo de Dijkstra — Red de ciudades de Guatemala ===\n");

    // ── CASO 1: Guatemala City → Flores ──────────────────────
    println!("--- Caso 1: Guatemala City → Flores ---");
    match dijkstra(&grafo, 0, 7) {
        Some((d, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("  Distancia : {} km", d);
            println!("  Camino    : {}", ruta.join(" → "));
        }
        None => println!("  No existe camino."),
    }

    // ── CASO 2: Guatemala City → San Marcos ──────────────────
    println!("\n--- Caso 2: Guatemala City → San Marcos ---");
    match dijkstra(&grafo, 0, 8) {
        Some((d, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("  Distancia : {} km", d);
            println!("  Camino    : {}", ruta.join(" → "));
        }
        None => println!("  No existe camino."),
    }

    // ── CASO 3: Antigua → Huehuetenango ──────────────────────
    println!("\n--- Caso 3: Antigua → Huehuetenango ---");
    match dijkstra(&grafo, 1, 4) {
        Some((d, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("  Distancia : {} km", d);
            println!("  Camino    : {}", ruta.join(" → "));
        }
        None => println!("  No existe camino."),
    }

    // ── CASO BORDE 1: origen == destino ──────────────────────
    println!("\n--- Caso borde: origen == destino (Guatemala City → Guatemala City) ---");
    match dijkstra(&grafo, 0, 0) {
        Some((d, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("  Distancia : {} km", d);
            println!("  Camino    : {}", ruta.join(" → "));
        }
        None => println!("  No existe camino."),
    }

    // ── CASO BORDE 2: nodo inalcanzable ──────────────────────
    // Flores no tiene aristas salientes, Flores → Guatemala City es inalcanzable
    println!("\n--- Caso borde: nodo inalcanzable (Flores → Guatemala City) ---");
    match dijkstra(&grafo, 7, 0) {
        Some((d, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("  Distancia : {} km", d);
            println!("  Camino    : {}", ruta.join(" → "));
        }
        None => println!("  No existe camino desde {} hacia {}.", nombres[7], nombres[0]),
    }

    // ── CASO BORDE 3: grafo vacío ─────────────────────────────
    println!("\n--- Caso borde: grafo vacío ---");
    let grafo_vacio: Grafo = vec![vec![], vec![]];
    match dijkstra(&grafo_vacio, 0, 1) {
        Some((d, _)) => println!("  Distancia : {} km", d),
        None => println!("  No existe camino (grafo sin aristas)."),
    }

    // ── TABLA: todas las distancias desde Guatemala City ─────
    println!("\n--- Distancias desde Guatemala City a todos los nodos ---");
    for destino in 0..nombres.len() {
        match dijkstra(&grafo, 0, destino) {
            Some((d, camino)) => {
                let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
                println!("  {:<20} {:>5} km   ({})", nombres[destino], d, ruta.join(" → "));
            }
            None => println!("  {:<20}  inalcanzable", nombres[destino]),
        }
    }
}