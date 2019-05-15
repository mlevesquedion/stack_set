use stack_set::StackSet;

pub struct Graph {
    pub vertices: usize,
    pub edges: Vec<Vec<usize>>,
}

pub fn has_cycle(g: &Graph) -> bool {
    if g.vertices == 0 {
        return false;
    }
    let mut stack = StackSet::new();
    let mut seen = (0..g.vertices).map(|_| false).collect::<Vec<bool>>();
    stack.push(0);
    while !stack.is_empty() {
        let current_node = stack.top().unwrap();
        if seen[current_node] {
            continue;
        }
        seen[current_node] = true;
        let neighbors = &g.edges[current_node];
        for neighbor in neighbors.iter() {
            if stack.contains(*neighbor) {
                return true;
            }
            stack.push(*neighbor);
        }
    }
    false
}

fn main() {
    let graph_with_cycle = Graph {
        vertices: 3,
        edges: vec![vec![1], vec![2], vec![0]],
    };
    let graph_without_cycle = Graph {
        vertices: 0,
        edges: Vec::new(),
    };
    assert!(has_cycle(&graph_with_cycle));
    assert!(!has_cycle(&graph_without_cycle));
}
