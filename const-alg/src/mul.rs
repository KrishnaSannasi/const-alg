use super::*;

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b Matrix<T, { M }, { O }>>
    for &'a Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        unsafe {
            collect_mat(
                self.rows()
                    .map(|row| collect_array(other.cols().map(|col| dot(row.clone(), col)))),
            )
        }
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b mut Matrix<T, { M }, { O }>>
    for &'a Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b mut Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = self;
        let o = &*other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<Matrix<T, { M }, { O }>>
    for &'a Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = self;
        let o = &other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b Matrix<T, { M }, { O }>>
    for &'a mut Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &*self;
        let o = other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b mut Matrix<T, { M }, { O }>>
    for &'a mut Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b mut Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &*self;
        let o = &*other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<Matrix<T, { M }, { O }>>
    for &'a mut Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &*self;
        let o = &other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b Matrix<T, { M }, { O }>>
    for Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &self;
        let o = other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<&'b mut Matrix<T, { M }, { O }>>
    for Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: &'b mut Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &self;
        let o = &*other;

        s * o
    }
}

impl<'a, 'b, T, const N: usize, const M: usize, const O: usize> Mul<Matrix<T, { M }, { O }>>
    for Matrix<T, { N }, { M }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    type Output = Matrix<T, { N }, { O }>;

    fn mul(self, other: Matrix<T, { M }, { O }>) -> Matrix<T, { N }, { O }> {
        let s = &self;
        let o = &other;

        s * o
    }
}
