use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

// Vec<Vec<(usize, u32)>>: lista de adyacencia compacta en memoria y O(1) de acceso
// por índice de nodo. Cada entrada almacena (destino, peso).
type Grafo = Vec<Vec<(usize, u32)>>;

// Retorna (distancia_minima, camino) o None si el destino no es alcanzable.
// Se devuelve el camino completo reconstruido mediante el mapa de predecesores.
fn dijkstra(grafo: &Grafo, origen: usize, destino: usize) -> Option<(u32, Vec<usize>)> {
    // HashMap<usize, u32>: sólo almacenamos nodos ya relajados (lazy initialization).
    // Alternativa Vec<u32> sería más rápida con nodos densos, pero HashMap es más
    // expresiva y correcta cuando el grafo puede ser disperso.
    let mut dist: HashMap<usize, u32> = HashMap::new();

    // HashMap de predecesores para reconstruir el camino al final.
    // Clave = nodo, Valor = nodo desde el cual llegamos con distancia mínima.
    let mut pred: HashMap<usize, usize> = HashMap::new();

    // BinaryHeap<Reverse<...>>: Rust provee max-heap por defecto.
    // Reverse invierte el orden, convirtiendo el heap en min-heap sin crates externos.
    // Almacenamos (costo_acumulado, nodo) — Reverse compara el primer campo primero.
    let mut cola: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

    dist.insert(origen, 0);
    cola.push(Reverse((0, origen)));

    while let Some(Reverse((costo, u))) = cola.pop() {
        // Si sacamos un costo mayor al registrado, esta entrada es obsoleta (lazy deletion).
        // En Rust no hay decrease-key nativo en BinaryHeap, así que permitimos duplicados
        // y los descartamos aquí.
        if costo > *dist.get(&u).unwrap_or(&u32::MAX) {
            continue;
        }

        // Llegamos al destino: salida temprana.
        if u == destino {
            break;
        }

        for &(v, peso) in &grafo[u] {
            let nueva_dist = costo + peso;
            let dist_actual = *dist.get(&v).unwrap_or(&u32::MAX);

            if nueva_dist < dist_actual {
                dist.insert(v, nueva_dist);
                pred.insert(v, u);
                cola.push(Reverse((nueva_dist, v)));
            }
        }
    }

    let distancia = *dist.get(&destino)?;

    // Reconstrucción del camino: seguimos predecesores desde destino hasta origen.
    let mut camino = Vec::new();
    let mut nodo = destino;
    loop {
        camino.push(nodo);
        if nodo == origen {
            break;
        }
        match pred.get(&nodo) {
            Some(&anterior) => nodo = anterior,
            None => return None, // destino no alcanzable (grafo desconectado)
        }
    }
    camino.reverse();

    Some((distancia, camino))
}

fn main() {
    // Grafo: red de ciudades de México (distancias aproximadas en km)
    //
    //  0 = CDMX
    //  1 = Puebla
    //  2 = Veracruz
    //  3 = Oaxaca
    //  4 = Guadalajara
    //  5 = Monterrey
    //  6 = Mérida
    //
    // Aristas dirigidas: (destino, km)
    let nombres = [
        "CDMX", "Puebla", "Veracruz", "Oaxaca",
        "Guadalajara", "Monterrey", "Mérida",
    ];

    let mut grafo: Grafo = vec![vec![]; nombres.len()];

    // CDMX → vecinos
    grafo[0].push((1, 120));  // CDMX → Puebla
    grafo[0].push((4, 540));  // CDMX → Guadalajara
    grafo[0].push((5, 900));  // CDMX → Monterrey

    // Puebla → vecinos
    grafo[1].push((2, 295));  // Puebla → Veracruz
    grafo[1].push((3, 360));  // Puebla → Oaxaca

    // Veracruz → vecinos
    grafo[2].push((3, 340));  // Veracruz → Oaxaca
    grafo[2].push((6, 680));  // Veracruz → Mérida

    // Oaxaca → vecinos
    grafo[3].push((6, 900));  // Oaxaca → Mérida

    // Guadalajara → vecinos
    grafo[4].push((5, 530));  // Guadalajara → Monterrey

    // Monterrey → vecinos
    grafo[5].push((6, 1200)); // Monterrey → Mérida

    // Sin aristas salientes de Mérida (nodo terminal en este ejemplo)

    let origen = 0;   // CDMX
    let destino = 6;  // Mérida

    println!("=== Algoritmo de Dijkstra — Red de ciudades de México ===\n");
    println!("Origen : {}", nombres[origen]);
    println!("Destino: {}\n", nombres[destino]);

    match dijkstra(&grafo, origen, destino) {
        Some((distancia, camino)) => {
            let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
            println!("Distancia mínima : {} km", distancia);
            println!("Camino reconstruido: {}", ruta.join(" → "));
        }
        None => println!("No existe camino entre {} y {}.", nombres[origen], nombres[destino]),
    }

    // Mostrar todas las distancias desde el origen (corremos Dijkstra completo sin destino fijo)
    println!("\n--- Distancias desde {} a todos los nodos ---", nombres[origen]);
    let mut dist_total: HashMap<usize, u32> = HashMap::new();
    let mut pred_total: HashMap<usize, usize> = HashMap::new();
    let mut cola: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new();

    dist_total.insert(origen, 0);
    cola.push(Reverse((0, origen)));

    while let Some(Reverse((costo, u))) = cola.pop() {
        if costo > *dist_total.get(&u).unwrap_or(&u32::MAX) {
            continue;
        }
        for &(v, peso) in &grafo[u] {
            let nueva = costo + peso;
            if nueva < *dist_total.get(&v).unwrap_or(&u32::MAX) {
                dist_total.insert(v, nueva);
                pred_total.insert(v, u);
                cola.push(Reverse((nueva, v)));
            }
        }
    }

    let mut nodos: Vec<usize> = dist_total.keys().cloned().collect();
    nodos.sort();

    for nodo in nodos {
        let d = dist_total[&nodo];
        // Reconstruir camino parcial
        let mut camino = vec![nodo];
        let mut cur = nodo;
        while cur != origen {
            match pred_total.get(&cur) {
                Some(&p) => { cur = p; camino.push(cur); }
                None => break,
            }
        }
        camino.reverse();
        let ruta: Vec<&str> = camino.iter().map(|&i| nombres[i]).collect();
        println!("  {:<14} {:>5} km   ({})", nombres[nodo], d, ruta.join(" → "));
    }
}
