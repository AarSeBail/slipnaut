# slipnaut - Ergonomic Graphs
`slipnaut` is a library providing *very* ergonomic graphs in rust. My core design principle for this library is laziness, not only for library users but also for graph instantiation. `slipnaut` provides concise and flexible instantiation of graphs.

One of the goals of `slipnaut` is to be nearly as concise as `NetworkX`. To achieve this in Rust, `slipnaut` encourages extensive use of closures.

## Example

Here is an example comparing `NetworkX` to `slipnaut`. Both examples traverse all Kempe chains in a graph.

<table>
<tr><th>slipnaut</th></tr>
<tr><td>

```rust
fn main() {
    let n = 50;
    let k = 5;
    
    let verts = || 0..n;
    let adj = |v| verts().filter(move |&u| (u + v) & 1 == 1);
    let graph = (verts, adj);

    let coloring: Vec<_> = (0..n).map(|i| i % k).collect();

    let proper = graph.efilter(|&u, &v| coloring[u] != coloring[v]);

    for a in 1..k {
        for b in 0..a {
            let chain = proper.vfilter(|&x| [a, b].contains(&coloring[x]));
            println!("{:?}", dfs_preorder(&chain, None).collect::<Vec<_>>());
        }
    }
}
```

</td></tr>
<tr><th>NetworkX</th></tr>
<tr><td>

```python
n = 50
k = 5

graph = nx.Graph()
graph.add_nodes_from(range(n))
graph.add_edges_from(filter(
    lambda e: (e[0] + e[1]) & 1 == 1, 
    product(range(n), range(n))
))

coloring = [i % k for i in range(n)]

proper = graph.edge_subgraph(filter(
    lambda e: coloring[e[0]] != coloring[e[1]], 
    graph.edges()
))

for a in range(k):
    for b in range(a):
        chain = proper.subgraph(filter(
            lambda v: coloring[v] in [a, b],
            proper.nodes()
        ))
        print(list(nx.dfs_preorder_nodes(chain)))


```

</td></tr>
</table>
