use crate::{collect_array, collect_mat, into_iter, ForBoth, Matrix};

/// Zips the matrix eagerly with the given function
/// 
/// Similar to Haskell's [zipWith](https://hoogle.haskell.org/?hoogle=zipWith)
/// 
/// This takes a secondary matrix, and a function that accepts two arguments,
/// elements into each matrix, and calls the function with all of the corrosponding elements of each matrix
/// and produces a matrix of the result 
pub trait ZipWith<RHS, F> {
    type Output;

    fn zip_with(self, other: RHS, f: F) -> Self::Output;
}

impl<T, U, V, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = into_iter(o).map(into_iter);

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b mut U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = o.iter_mut().map(|row| row.iter_mut());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = into_iter(s).map(into_iter);
        let o = o.iter().map(|row| row.iter());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, T, U, V, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let o = into_iter(o).map(into_iter);
        let s = s.iter_mut().map(|row| row.iter_mut());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, 'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b mut U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter_mut().map(|row| row.iter_mut());
        let o = o.iter_mut().map(|row| row.iter_mut());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, 'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter_mut().map(|row| row.iter_mut());
        let o = o.iter().map(|row| row.iter());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, T, U, V, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = into_iter(o).map(into_iter);

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, 'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b mut U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = o.iter_mut().map(|row| row.iter_mut());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<'a, 'b, T, U, V, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b U) -> V,
{
    type Output = Matrix<V, { N }, { M }>;

    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, mut f: F) -> Self::Output {
        let Matrix(s) = self;
        let Matrix(o) = other;

        let s = s.iter().map(|row| row.iter());
        let o = o.iter().map(|row| row.iter());

        let iter = s.zip(o).map(|(s, o)| unsafe {
            let arr: [_; M] = collect_array(s.zip(o).map(|(s, o)| f(s, o)));

            arr
        });

        unsafe { collect_mat(iter) }
    }
}

impl<T, U, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, U),
{
    default fn zip_with(self, other: Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b mut U),
{
    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for Matrix<T, { N }, { M }>
where
    F: FnMut(T, &'b U),
{
    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, T, U, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, U),
{
    default fn zip_with(self, other: Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b mut U),
{
    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T, &'b U),
{
    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, T, U, F, const N: usize, const M: usize> ZipWith<Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, U),
{
    default fn zip_with(self, other: Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b mut Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b mut U),
{
    default fn zip_with(self, other: &'b mut Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, 'b, T, U, F, const N: usize, const M: usize> ZipWith<&'b Matrix<U, { N }, { M }>, F>
    for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T, &'b U),
{
    default fn zip_with(self, other: &'b Matrix<U, { N }, { M }>, f: F) -> Self::Output {
        self.for_both(other, f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}
