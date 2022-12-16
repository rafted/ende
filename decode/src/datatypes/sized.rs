use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub struct SizedVec<T, const S: usize> {
    pub inner: Vec<T>,
}

impl<T, const S: usize> SizedVec<T, S>
where
    T: Clone,
{
    pub fn new() -> Self {
        let mut vec = Vec::<T>::new();
        vec.reserve_exact(S);

        Self { inner: vec }
    }

    pub fn from_vec(inner: Vec<T>) -> Self {
        let mut inner = inner.clone();
        inner.reserve_exact(S);

        Self {
            inner: inner.to_vec(),
        }
    }

    pub fn get_size(&self) -> usize {
        S
    }

    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }
}

impl<T, const S: usize> From<Vec<T>> for SizedVec<T, S>
where
    T: Clone,
{
    fn from(data: Vec<T>) -> Self {
        SizedVec::<T, S>::from_vec(data)
    }
}
