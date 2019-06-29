#![feature(const_generics, specialization, existential_type)]
#![allow(unused_unsafe)]
// #![forbid(missing_docs)]

/*!
 * const-alg
 * 
 * This is an experimental linear algebra library that is backed by arrays!
 * 
 * The syntax for an matrix is just,
 * 
 * ```rust
 * let m = Matrix(
 *      [[0, 1, 2],
 *       [3, 4, 5]]
 * );
 * ```
 * 
 * For a 2x3 matrix! This will work with any number of rows and columns!
 */

use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

use array_vec::ArrayVec;
use num::{One, Zero};

pub mod iter;
mod mul;

mod zip_with;
pub use zip_with::ZipWith;

mod map;
pub use map::Map;

mod for_each;
pub use for_each::ForEach;

mod for_both;
pub use for_both::ForBoth;

#[cfg(test)]
mod test;

fn into_iter<T, const N: usize>(arr: [T; N]) -> array_vec::IntoIter<T, { N }> {
    ArrayVec::<T, { N }>::from(arr).into_iter()
}

cfg_if::cfg_if! {
    if #[cfg(any(debug_assertions))] {
        use std::convert::TryInto;

        // This branch panics if something goes wrong, and is perfectly safe
        // With this, the only unsafty is in `Matrix::get_all_mut`
        // Which is safe as described in it's docs

        fn collect_array<I, T, const N: usize>(iter: I) -> [T; N] where I: IntoIterator<Item = T> {
            to_array(
                iter.into_iter().collect::<ArrayVec<T, { N }>>()
            )
        }
        fn to_array<T, const N: usize>(arr: ArrayVec<T, {N}>) -> [T; N] {
            arr.try_into()
                .ok()
                .expect("iterator wasn't long enough")
        }


        fn collect_mat<I, T, const N: usize, const M: usize>(iter: I) -> Matrix<T, {N}, {M}> where I: IntoIterator<Item = [T; M]> {
            Matrix(collect_array(iter))
        }

    } else {

        // This branch causes UB if something goes wrong, and is very unsafe
        // But because the lengths of all of the arrays are known ahead of time
        // it is easy to check for soundness

        unsafe fn collect_array<I, T, const N: usize>(iter: I) -> [T; N] where I: IntoIterator<Item = T> {
            to_array(
                iter.into_iter().collect::<ArrayVec<T, { N }>>()
            )
        }

        unsafe fn to_array<T, const N: usize>(arr: ArrayVec<T, {N}>) -> [T; N] {
            arr.into_array_unchecked()
        }

        unsafe fn collect_mat<I, T, const N: usize, const M: usize>(iter: I) -> Matrix<T, {N}, {M}> where I: IntoIterator<Item = [T; M]> {
            Matrix(collect_array(iter))
        }

    }
}

/// A Square Matrix represents an `N x N` matrix stored in row-major order
/// 
/// So when you go to make it, you can just do
/// 
/// ```
/// Matrix(
///     [[0, 1],
///      [3, 4]]
/// )
/// ```
/// 
/// And Rust will take care of the rest!
pub type SquareMatrix<T, const N: usize> = Matrix<T, { N }, { N }>;

/// Matrix represents an `N x M` matrix stored in row-major order
/// 
/// So when you go to make it, you can just do
/// 
/// ```
/// Matrix(
///     [[0, 1, 2],
///      [3, 4, 5]]
/// )
/// ```
/// 
/// And Rust will take care of the rest!
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Matrix<T, const N: usize, const M: usize>(pub [[T; M]; N]);

impl<T, const N: usize, const M: usize> From<[[T; M]; N]> for Matrix<T, { N }, { M }> {
    fn from(arr: [[T; M]; N]) -> Self {
        Self(arr)
    }
}

impl<T, const N: usize, const M: usize> Into<[[T; M]; N]> for Matrix<T, { N }, { M }> {
    fn into(self) -> [[T; M]; N] {
        self.0
    }
}

impl<T: Zero, const N: usize, const M: usize> Zero for Matrix<T, { N }, { M }> {
    fn zero() -> Self {
        let zeros = std::iter::repeat_with(T::zero);
        let rows = std::iter::repeat_with(|| -> [T; M] { unsafe { collect_array(zeros.clone()) } });

        unsafe { collect_mat(rows) }
    }

    fn set_zero(&mut self) {
        self.0
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(T::set_zero)
    }

    fn is_zero(&self) -> bool {
        for i in self.0.iter().flat_map(|row| row.iter()) {
            if !i.is_zero() {
                return false;
            }
        }

        true
    }
}

impl<T: PartialEq + Zero + One, const N: usize> One for Matrix<T, { N }, { N }>
where
    Self: Mul<Output = Self>,
{
    fn one() -> Self {
        let mut mat = Matrix::<T, { N }, { N }>::zero();

        for i in 0..N {
            mat[(i, i)].set_one();
        }

        mat
    }

    fn set_one(&mut self) {
        self.set_zero();

        for i in 0..N {
            unsafe {
                self.get_unchecked_mut(i, i).set_one();
            }
        }
    }

    fn is_one(&self) -> bool {
        for i in 0..N {
            for j in 0..N {
                let val = &self[(i, j)];
                if i == j {
                    if !val.is_one() {
                        return false;
                    }
                } else {
                    if !val.is_zero() {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl<T, const N: usize, const M: usize> Add for Matrix<T, { N }, { M }>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.zip_with(other, T::add)
    }
}

impl<T, const N: usize, const M: usize> AddAssign for Matrix<T, { N }, { M }>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.zip_with(other, T::add_assign);
    }
}

impl<T, const N: usize, const M: usize> Sub for Matrix<T, { N }, { M }>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.zip_with(other, T::sub)
    }
}

impl<T, const N: usize, const M: usize> SubAssign for Matrix<T, { N }, { M }>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.zip_with(other, T::sub_assign);
    }
}

fn dot<'a, T: 'a, I: IntoIterator<Item = &'a T>, J: IntoIterator<Item = &'a T>>(a: I, b: J) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    a.into_iter()
        .cloned()
        .zip(b.into_iter().cloned())
        .map(|(a, b)| a * b)
        .fold(T::zero(), |acc, x| acc + x)
}

impl<T, const N: usize> MulAssign for Matrix<T, { N }, { N }>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + Clone,
{
    fn mul_assign(&mut self, other: Self) {
        let Matrix(s) = self.clone();
        let Matrix(o) = other.transpose();

        for (j, o_row) in o.iter().enumerate() {
            for (i, s_row) in s.iter().enumerate() {
                self[(i, j)] = dot(s_row as &[_], o_row as &[_]);
            }
        }
    }
}

impl<T, const N: usize, const M: usize> Mul<T> for Matrix<T, { N }, { M }>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Matrix<T, { N }, { M }>;

    fn mul(self, other: T) -> Matrix<T, { N }, { M }> {
        self.map(|x| -> T { x * other.clone() })
    }
}

impl<T, const N: usize, const M: usize> Div<T> for Matrix<T, { N }, { M }>
where
    T: Div<Output = T> + Clone,
{
    type Output = Matrix<T, { N }, { M }>;

    fn div(self, other: T) -> Matrix<T, { N }, { M }> {
        self.map(|x| -> T { x / other.clone() })
    }
}

impl<T, const N: usize, const M: usize> MulAssign<T> for Matrix<T, { N }, { M }>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, other: T) {
        self.for_each(|x| *x *= other.clone());
    }
}

impl<T, const N: usize, const M: usize> DivAssign<T> for Matrix<T, { N }, { M }>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, other: T) {
        self.for_each(|x| *x /= other.clone());
    }
}

impl<T, const N: usize, const M: usize> Neg for Matrix<T, { N }, { M }>
where
    T: Neg,
{
    type Output = Matrix<T::Output, { N }, { M }>;

    fn neg(self) -> Self::Output {
        self.map(T::neg)
    }
}

cfg_if::cfg_if! {
    if #[cfg(any(debug_assertions))] {

        impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
            fn check_bounds(&self, row: usize, col: usize) {
                assert!(row < N && col < M, "You have a bug in your code that would be UB in release mode, this check will not trip in release mode (unless you are testing in release mode)");
            }
        }

    } else {

        impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
            fn check_bounds(&self, _row: usize, _col: usize) {

            }
        }

    }
}

impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    /// Transposes the matrix,
    /// 
    /// This is some sugar for transpose
    #[allow(non_snake_case)]
    pub fn T(self) -> Matrix<T, { M }, { N }> {
        self.transpose()
    }

    /// Does the matrix transpose of the given matrix
    /// 
    /// Every element `matrix[(row, col)]` will be taken to `output[(col, row)]`
    pub fn transpose(self) -> Matrix<T, { M }, { N }> {
        unsafe { collect_mat(self.into_cols().map(|col| collect_array(col))) }
    }

    /// Gets an element from the matrix
    /// 
    /// If the row or col is out of bounds, None is returned
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < N && col < M {
            Some(&self.0[row][col])
        } else {
            None
        }
    }

    /// Gets an element from the matrix
    /// 
    /// If the row or col is out of bounds, None is returned
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < N && col < M {
            Some(&mut self.0[row][col])
        } else {
            None
        }
    }

    /// Gets an element from the matrix
    /// 
    /// If the row or col is out of bounds, this causes UB on release mode, and panics on debug mode
    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.check_bounds(row, col);
        
        self.0.get_unchecked(row).get_unchecked(col)
    }

    /// Gets an element from the matrix
    /// 
    /// If the row or col is out of bounds, this causes UB on release mode, and panics on debug mode
    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.check_bounds(row, col);

        self.0.get_unchecked_mut(row).get_unchecked_mut(col)
    }

    /// Get's all of the elements specified by the array,
    /// 
    /// If there are any duplicates, the first one will return a value and the rest will be None,
    /// 
    /// If any pos is out of bounds, it will result in None
    /// 
    /// Example,
    /// 
    /// ```
    /// let m = Matrix(
    ///     [[0, 1, 2],
    ///      [3, 4, 5]]
    /// );
    /// 
    /// assert_eq!(m.get_all_mut([(0, 0), (0, 2), (2, 0), (0, 0)]), [Some(&mut 0), Some(&mut 2), None, None]);
    /// ```
    pub fn get_all_mut<const P: usize>(&mut self, pos: [(usize, usize); P]) -> [Option<&mut T>; P] {
        let pos = ArrayVec::<(usize, usize), { P }>::from(pos);

        let mut pos = pos
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<ArrayVec<_, { P }>>();

        pos.sort_unstable();

        let mut output = std::iter::repeat_with(|| None).collect::<ArrayVec<_, { P }>>();

        let mut pos = pos.into_iter();

        if let Some((mut last, i)) = pos.next() {
            let (row, col) = last;

            let this: *mut Self = self;
            output[i] = unsafe { (*this).get_mut(row, col) };

            for (pos, i) in pos {
                if last != pos {
                    last = pos;

                    let (row, col) = pos;

                    output[i] = unsafe { (*this).get_mut(row, col) };
                }
            }
        }

        unsafe { to_array(output) }
    }
}

use std::fmt;

impl<T: fmt::Debug, const N: usize, const M: usize> fmt::Debug for Matrix<T, { N }, { M }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();

        for i in self.0.iter() {
            list.entry(&(i as &[_]));
        }

        list.finish()
    }
}

impl<T: Eq, const N: usize, const M: usize> Eq for Matrix<T, { N }, { M }> {}
impl<T: PartialEq, const N: usize, const M: usize> PartialEq for Matrix<T, { N }, { M }> {
    fn eq(&self, other: &Self) -> bool {
        for (s, o) in self.0.iter().zip(other.0.iter()) {
            if s as &[_] != o as &[_] {
                return false;
            }
        }

        true
    }
}

use std::hash::{Hash, Hasher};

impl<T: Hash, const N: usize, const M: usize> Hash for Matrix<T, { N }, { M }> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let arr = self
            .0
            .iter()
            .map(|x| x as &[_])
            .collect::<ArrayVec<&[T], { N }>>();

        let arr: &[&[T]] = &arr;

        arr.hash(state);
    }
}

impl<T, const N: usize, const M: usize> Index<(usize, usize)> for Matrix<T, { N }, { M }> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        self.get(row, col).expect("position out of bounds!")
    }
}

impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for Matrix<T, { N }, { M }> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.get_mut(row, col).expect("position out of bounds!")
    }
}

impl<T, const N: usize, const M: usize> Deref for Matrix<T, { N }, { M }> {
    type Target = [[T; M]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize, const M: usize> DerefMut for Matrix<T, { N }, { M }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
