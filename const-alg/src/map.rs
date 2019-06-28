use crate::{collect_array, collect_mat, into_iter, ForEach, Matrix};

pub trait Map<F> {
    type Output;

    fn map(self, f: F) -> Self::Output;
}

impl<F, T, U, const N: usize, const M: usize> Map<F> for Matrix<T, { N }, { M }>
where
    F: FnMut(T) -> U,
{
    type Output = Matrix<U, { N }, { M }>;

    default fn map(self, mut f: F) -> Self::Output {
        unsafe {
            collect_mat(into_iter(self.0).map(|row| unsafe {
                let arr: [_; M] = collect_array(into_iter(row).map(&mut f));

                arr
            }))
        }
    }
}

impl<F, T, const N: usize, const M: usize> Map<F> for Matrix<T, { N }, { M }>
where
    F: FnMut(T),
{
    default fn map(self, f: F) -> Self::Output {
        self.for_each(f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, F, T, U, const N: usize, const M: usize> Map<F> for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T) -> U,
{
    type Output = Matrix<U, { N }, { M }>;

    default fn map(self, mut f: F) -> Self::Output {
        unsafe {
            collect_mat(self.0.iter_mut().map(|row| unsafe {
                let arr: [_; M] = collect_array(row.iter_mut().map(&mut f));

                arr
            }))
        }
    }
}

impl<'a, F, T, U, const N: usize, const M: usize> Map<F> for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T) -> U,
{
    type Output = Matrix<U, { N }, { M }>;

    default fn map(self, mut f: F) -> Self::Output {
        unsafe {
            collect_mat(self.0.iter().map(|row| unsafe {
                let arr: [_; M] = collect_array(row.iter().map(&mut f));

                arr
            }))
        }
    }
}

impl<'a, T, F, const N: usize, const M: usize> Map<F> for &'a mut Matrix<T, { N }, { M }>
where
    F: FnMut(&'a mut T),
{
    default fn map(self, f: F) -> Self::Output {
        self.for_each(f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}

impl<'a, 'b, T, F, const N: usize, const M: usize> Map<F> for &'a Matrix<T, { N }, { M }>
where
    F: FnMut(&'a T),
{
    default fn map(self, f: F) -> Self::Output {
        self.for_each(f);

        assert_eq!(std::mem::size_of::<Self::Output>(), 0);

        unsafe { std::mem::uninitialized() }
    }
}
