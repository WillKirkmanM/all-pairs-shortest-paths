use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::i32;

// Represents an edge in the graph
#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

// Helper struct for Priority Queue (Dijkstra)
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

// We need to implement Ord manually to make the BinaryHeap a Min-Heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice the flip: we compare other to self to get Min-Heap behavior
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Step 1 & 2: Bellman-Ford Algorithm
/// Computes the "potential" h(v) for the reweighting.
/// We simulate a dummy node connected to all other nodes with weight 0
/// by initialising all distances to 0.
fn bellman_ford(adj_list: &Vec<Vec<Edge>>, num_nodes: usize) -> Option<Vec<i32>> {
    // Initialise distances to 0 (simulating dummy node connection)
    let mut dist = vec![0; num_nodes];

    // Relax edges |V| - 1 times
    for _ in 0..num_nodes {
        let mut changed = false;
        for u in 0..num_nodes {
            for edge in &adj_list[u] {
                if dist[u] != i32::MAX && dist[u] + edge.weight < dist[edge.to] {
                    dist[edge.to] = dist[u] + edge.weight;
                    changed = true;
                }
            }
        }
        // If no edges relaxed, we can stop early
        if !changed {
            return Some(dist);
        }
    }

    // Check for negative cycles
    for u in 0..num_nodes {
        for edge in &adj_list[u] {
            if dist[u] != i32::MAX && dist[u] + edge.weight < dist[edge.to] {
                // Negative cycle detected
                return None;
            }
        }
    }

    Some(dist)
}

/// Step 4: Dijkstra's Algorithm
/// Standard implementation using a BinaryHeap
fn dijkstra(adj_list: &Vec<Vec<Edge>>, start_node: usize) -> Vec<Option<i32>> {
    let n = adj_list.len();
    let mut dist = vec![None; n]; // None represents Infinity
    let mut heap = BinaryHeap::new();

    dist[start_node] = Some(0);
    heap.push(State { cost: 0, position: start_node });

    while let Some(State { cost, position }) = heap.pop() {
        // If we found a shorter path already, ignore this one
        if let Some(d) = dist[position] {
            if cost > d { continue; }
        }

        for edge in &adj_list[position] {
            let next_cost = cost + edge.weight;
            
            // If we found a better path
            let is_shorter = match dist[edge.to] {
                Some(d) => next_cost < d,
                None => true,
            };

            if is_shorter {
                dist[edge.to] = Some(next_cost);
                heap.push(State { cost: next_cost, position: edge.to });
            }
        }
    }

    dist
}

/// Main Johnson's Algorithm
pub fn johnsons_algorithm(
    num_nodes: usize, 
    edges: Vec<(usize, usize, i32)>
) -> Result<Vec<Vec<Option<i32>>>, &'static str> {
    
    // 1. Build Adjacency List
    let mut adj_list = vec![Vec::new(); num_nodes];
    for (u, v, w) in &edges {
        adj_list[*u].push(Edge { to: *v, weight: *w });
    }

    // 2. Run Bellman-Ford to get potentials (h)
    // This handles the "dummy node" logic internally by initing dists to 0
    let h = match bellman_ford(&adj_list, num_nodes) {
        Some(h) => h,
        None => return Err("Negative Cycle Detected"),
    };

    // 3. Reweight the graph
    // New Weight = Old Weight + h[u] - h[v]
    let mut reweighted_adj = vec![Vec::new(); num_nodes];
    for u in 0..num_nodes {
        for edge in &adj_list[u] {
            let new_weight = edge.weight + h[u] - h[edge.to];
            reweighted_adj[u].push(Edge {
                to: edge.to,
                weight: new_weight
            });
        }
    }

    // 4. Run Dijkstra for every node
    let mut all_pairs_shortest_paths = Vec::new();

    for u in 0..num_nodes {
        let d_prime = dijkstra(&reweighted_adj, u);
        
        // 5. Un-reweight the distances
        // Real Dist = Dijkstra Dist - h[u] + h[v]
        let mut real_dists = Vec::new();
        for v in 0..num_nodes {
            let val = match d_prime[v] {
                Some(d) => Some(d - h[u] + h[v]),
                None => None,
            };
            real_dists.push(val);
        }
        all_pairs_shortest_paths.push(real_dists);
    }

    Ok(all_pairs_shortest_paths)
}

fn main() {
    // Example Graph
    // 0 -> 1 (weight -5)
    // 1 -> 2 (weight 2)
    // 2 -> 0 (weight 4)
    // 0 -> 2 (weight 3)
    // Negative edge exists, but no negative cycle.
    let edges = vec![
        (0, 1, -5),
        (1, 2, 2),
        (2, 0, 4),
        (0, 2, 3),
    ];

    let num_nodes = 3;

    match johnsons_algorithm(num_nodes, edges) {
        Ok(matrix) => {
            println!("All Pairs Shortest Paths:");
            for (u, row) in matrix.iter().enumerate() {
                for (v, dist) in row.iter().enumerate() {
                    match dist {
                        Some(d) => println!("{} -> {}: {}", u, v, d),
                        None => println!("{} -> {}: Inf", u, v),
                    }
                }
                println!("---");
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}