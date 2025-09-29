use slipnaut::Graph;
use slipnaut::traversal::dfs_preorder;

fn main() {
    let n = 50;
    let k = 5;
    
    let verts = || 0..n;
    let adj = |v| verts().filter(move |&u| (u + v) & 1 == 1);

    let graph = (verts, adj);
    let coloring: Vec<_> = (0..n).map(|i| i % k).collect();

    // Filter out improperly colored edges
    let proper = graph.efilter(|&u, &v| coloring[u] != coloring[v]);

    for a in 1..k {
        for b in 0..a {
            // Filter out 2-colored vertices
            let chain = proper.vfilter(|&x| [a, b].contains(&coloring[x]));

            // Traverse the Kempe chains
            println!("{:?}", dfs_preorder(&chain, None).collect::<Vec<_>>());
        }
    }
}
