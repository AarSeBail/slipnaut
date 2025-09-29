use rand::{Rng, rng};
use slipnaut::Graph;
use slipnaut::traversal::dfs;

fn main() {
    let n = 50usize;
    let verts = || 0..n;
    let adj = |k| verts().filter(move |&x| (x + k) % 4 == 1);

    let graph = (verts, adj);
    let mut coloring: Vec<_> = (0..n).map(|_| rng().random_range(0..3)).collect();
    coloring[0] = 0;

    let colors = [0, 1];

    // Filter out improperly colored edges
    let proper = graph.efilter(|&u, &v| coloring[u] != coloring[v]);
    // Filter out 2-colored vertices
    let chain = proper.vfilter(|&x| colors.contains(&coloring[x]));

    // Traverse 0/1 Kempe chain starting at 0
    for v in dfs(&chain, Some(0)) {
        println!("{v}");
    }
}
