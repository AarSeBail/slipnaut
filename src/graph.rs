use std::marker::PhantomData;

use crate::combinators::{EFilter, VFilter};

pub trait Graph<'a, V> {
    type VI: Iterator<Item = V>;
    type AI: Iterator<Item = V>;

    fn verts<'b>(&'b self) -> Self::VI
    where
        'b: 'a;
    fn adj<'b>(&'b self, v: V) -> Option<Self::AI>
    where
        'b: 'a;

    fn vfilter<F: Fn(&V) -> bool>(&'a self, f: F) -> VFilter<'a, V, Self, F>
    where
        Self: Sized,
    {
        VFilter {
            predicate: f,
            graph: &self,
            _p: PhantomData,
        }
    }

    fn efilter<F>(&'a self, f: F) -> EFilter<'a, V, Self, F>
    where
        Self: Sized,
        F: Fn(&V, &V) -> bool,
    {
        EFilter {
            predicate: f,
            graph: &self,
            _p: PhantomData,
        }
    }
}

impl<'a, V, VI, AI, VS, AS> Graph<'a, V> for (VS, AS)
where
    VI: Iterator<Item = V>,
    AI: Iterator<Item = V>,
    VS: Fn() -> VI,
    AS: Fn(V) -> AI,
{
    type VI = VI;

    type AI = AI;

    fn verts<'b>(&'b self) -> VI
    where
        'b: 'a,
    {
        self.0()
    }

    fn adj<'b>(&'b self, v: V) -> Option<AI>
    where
        'b: 'a,
    {
        Some(self.1(v))
    }
}
