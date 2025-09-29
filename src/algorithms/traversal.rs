use std::collections::HashSet;
use std::hash::Hash;

use rustc_hash::FxHashSet;

use crate::Graph;

pub fn dfs_preorder<'a, V: Hash + Copy + Eq, G: Graph<'a, V, AI: DoubleEndedIterator>>(
    g: &'a G,
    root: Option<V>,
) -> impl Iterator<Item = V> {
    let mut visited: HashSet<V, _> = FxHashSet::default();
    let mut stack: Vec<V> = vec![];

    if let Some(r) = root {
        stack.push(r);
    } else {
        stack.extend(g.verts());
        stack.reverse();
    }

    (0..)
        .map(move |_| {
            // p will be yielded if it has not been visited
            // additionally p's children will be added to the stack
            while let Some(p) = stack.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                stack.extend(g.adj(p).unwrap().rev().filter(|x| !visited.contains(x)));
                return Some(p);
            }
            None
        })
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap())
}
