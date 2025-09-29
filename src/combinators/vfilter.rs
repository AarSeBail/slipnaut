use std::iter::Filter;
use std::marker::PhantomData;

use crate::Graph;

pub struct VFilter<'a, V, G, F>
where
    G: Graph<'a, V>,
    F: Fn(&V) -> bool,
{
    pub(crate) predicate: F,
    pub(crate) graph: &'a G,
    pub(crate) _p: PhantomData<V>,
}

impl<'a, V, G, F> Graph<'a, V> for VFilter<'a, V, G, F>
where
    G: Graph<'a, V>,
    F: 'a + Fn(&V) -> bool,
{
    type VI = Filter<G::VI, &'a F>;

    type AI = Filter<G::AI, &'a F>;

    fn verts<'b>(&'b self) -> Self::VI
    where
        'b: 'a,
    {
        self.graph.verts().filter(&self.predicate)
    }

    fn adj<'b>(&'b self, v: V) -> Option<Self::AI>
    where
        'b: 'a,
    {
        if !(self.predicate)(&v) {
            None
        } else if let Some(pre) = self.graph.adj(v) {
            Some(pre.filter(&self.predicate))
        } else {
            None
        }
    }
}
