use crate::{into_iter, Matrix};

/// Calls a function eagerly on all elements of a matrix
/// 
/// This takes a function that accepts items from the matrix, and calls it on all elements of the matrix
pub trait ForEach<F> {
    fn for_each(self, f: F);
}

impl<T, F, const N: usize, const M: usize> ForEach<F> for Matrix<T, { N }, { M }>
where
    F: FnMut(T),
{
    fn for_each(self, mut f: F) {
        let Matrix(s) = self;

        let s = into_iter(s).map(into_iter);

        s.for_each(|s| unsafe { s.for_each(&mut f) });
    }
}

impl<'a, T, F, const N: usize, const M: usize> ForEach<F> for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T),
{
    fn for_each(self, mut f: F) {
        let Matrix(s) = self;

        let s = s.iter_mut().map(|row| row.iter_mut());

        s.for_each(|s| unsafe { s.for_each(&mut f) });
    }
}

impl<'a, T, F, const N: usize, const M: usize> ForEach<F> for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T),
{
    fn for_each(self, mut f: F) {
        let Matrix(s) = self;

        let s = s.iter().map(|row| row.iter());

        s.for_each(|s| unsafe { s.for_each(&mut f) });
    }
}
