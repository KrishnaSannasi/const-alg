use crate::{into_iter, Matrix};

/// Calls a function eagerly on all elements of two matricies
/// 
/// This takes a secondary matrix, and a function that accepts two arguments,
/// elements into each matrix, and calls the function with all of the corrosponding elements of each matrix
pub trait ForBoth<RHS, F> {
    fn for_both(self, other: RHS, f: F);
}

impl<T, U, F, const N: usize, const M: usize> ForBoth<Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, U),
{
    fn for_both(self, other: Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = into_iter(o).map(into_iter);

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b mut Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b mut U),
{
    fn for_both(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = o.iter_mut().map(|row| row.iter_mut());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b U),
{
    fn for_both(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = o.iter().map(|row| row.iter());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, T, U, F, const N: usize, const M: usize> ForBoth<Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, U),
{
    fn for_both(self, other: Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let o = into_iter(o).map(into_iter);
        let s = s.iter_mut().map(|row| row.iter_mut());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b mut U),
{
    fn for_both(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter_mut().map(|row| row.iter_mut());
        let o = o.iter_mut().map(|row| row.iter_mut());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b U),
{
    fn for_both(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter_mut().map(|row| row.iter_mut());
        let o = o.iter().map(|row| row.iter());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, T, U, F, const N: usize, const M: usize> ForBoth<Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, U),
{
    fn for_both(self, other: Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = into_iter(o).map(into_iter);

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b mut U),
{
    fn for_both(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = o.iter_mut().map(|row| row.iter_mut());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ForBoth<&'b Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b U),
{
    fn for_both(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = o.iter().map(|row| row.iter());

        s.zip(o)
            .for_each(|(s, o)| unsafe { s.zip(o).for_each(|(s, o)| f(s, o)) });
    }
}
