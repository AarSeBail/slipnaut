use std::iter::FusedIterator;
use std::marker::PhantomData;

use crate::Graph;

#[derive(Debug, Clone)]
pub struct EdgeFilter<'b, V, F, AI: Iterator<Item = V>>
where
    F: Fn(&V, &V) -> bool,
{
    v: V,
    inner: AI,
    predicate: &'b F,
}

impl<'b, V, F: Fn(&V, &V) -> bool, AI: Iterator<Item = V>> Iterator for EdgeFilter<'b, V, F, AI> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .by_ref()
            .filter(|u| (self.predicate)(&self.v, u))
            .next()
    }
}

impl<'b, V, F: Fn(&V, &V) -> bool, AI: DoubleEndedIterator<Item = V>> DoubleEndedIterator
    for EdgeFilter<'b, V, F, AI>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .by_ref()
            .filter(|u| (self.predicate)(&self.v, u))
            .next_back()
    }
}

impl<'b, V, F: Fn(&V, &V) -> bool, AI: FusedIterator<Item = V>> FusedIterator
    for EdgeFilter<'b, V, F, AI>
{
}

pub struct EFilter<'a, V, G, F>
where
    G: Graph<'a, V>,
    F: Fn(&V, &V) -> bool,
{
    pub(crate) predicate: F,
    pub(crate) graph: &'a G,
    pub(crate) _p: PhantomData<V>,
}

impl<'a, V, G, F> Graph<'a, V> for EFilter<'a, V, G, F>
where
    V: Clone,
    G: Graph<'a, V>,
    F: 'a + Fn(&V, &V) -> bool,
{
    type VI = G::VI;

    type AI = EdgeFilter<'a, V, F, G::AI>;

    fn verts<'b>(&'b self) -> Self::VI
    where
        'b: 'a,
    {
        self.graph.verts()
    }

    fn adj<'b>(&'b self, v: V) -> Option<Self::AI>
    where
        'b: 'a,
    {
        if let Some(pre) = self.graph.adj(v.clone()) {
            Some(EdgeFilter {
                v,
                inner: pre,
                predicate: &self.predicate,
            })
        } else {
            None
        }
    }
}
