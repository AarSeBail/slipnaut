use std::collections::HashMap;
use std::hash::Hash;

pub trait DataStore<V, D> {
    // Makes a new instance
    // Allows stores to initialize based on knowledge of the domain.
    fn make<VI: Iterator<Item = V>, F: Fn(&V) -> D>(verts: VI, default: Option<F>) -> Self;
    fn insert_data(&mut self, v: V, data: D);
    fn get_data(&self, v: V) -> Option<D>;
}

impl<V: Eq + Hash, D: Clone> DataStore<V, D> for HashMap<V, D> {
    fn make<VI: Iterator<Item = V>, F: Fn(&V) -> D>(verts: VI, default: Option<F>) -> Self {
        let mut result = Self::default();
        if let Some(f) = default {
            for v in verts {
                let d = f(&v);
                result.insert(v, d);
            }
        }
        result
    }

    fn insert_data(&mut self, v: V, data: D) {
        self.insert(v, data);
    }

    fn get_data(&self, v: V) -> Option<D> {
        self.get(&v).cloned()
    }
}

impl<D: Clone + Default> DataStore<usize, D> for Vec<D> {
    fn make<VI: Iterator<Item = usize>, F: Fn(&usize) -> D>(verts: VI, default: Option<F>) -> Self {
        let mut result = Vec::with_capacity(verts.size_hint().1.unwrap());

        if let Some(f) = default {
            for v in verts {
                if v > result.len() {
                    result.resize(v + 1, D::default());
                }
                let d = f(&v);
                result[v] = d;
            }
        }

        result
    }

    fn insert_data(&mut self, v: usize, data: D) {
        self[v] = data;
    }

    fn get_data(&self, v: usize) -> Option<D> {
        self.get(v).cloned()
    }
}
