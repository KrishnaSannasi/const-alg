use crate::{into_iter, Matrix};
use array_vec::{ArrayVec, IntoIter};

use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

use std::iter::FusedIterator;

#[derive(Clone)]
pub struct IntoLine<T, const N: usize>(IntoIter<T, { N }>);

#[derive(Clone)]
pub struct IntoRows<T, const N: usize, const M: usize> {
    inner: IntoIter<[T; M], { N }>,
}

#[derive(Clone)]
pub struct IntoCols<T, const N: usize, const M: usize> {
    arr: ArrayVec<IntoIter<T, { M }>, { N }>,
}

pub struct RowMut<'a, T, const N: usize, const M: usize>(IterMut<'a, T>);

pub struct RowsMut<'a, T, const N: usize, const M: usize> {
    inner: IterMut<'a, [T; M]>,
}

pub struct ColMut<'a, T, const N: usize, const M: usize> {
    inner: *mut [[T; M]; N],
    col: usize,
    row: usize,
    row_end: usize,
    lt: PhantomData<&'a mut [[T; M]; N]>,
}

pub struct ColsMut<'a, T, const N: usize, const M: usize> {
    inner: *mut [[T; M]; N],
    col: usize,
    col_end: usize,
    lt: PhantomData<&'a mut [[T; M]; N]>,
}

pub struct Row<'a, T, const N: usize, const M: usize>(Iter<'a, T>);

pub struct Rows<'a, T, const N: usize, const M: usize> {
    inner: Iter<'a, [T; M]>,
}

pub struct Col<'a, T, const N: usize, const M: usize> {
    inner: &'a [[T; M]; N],
    col: usize,
    row: usize,
    row_end: usize,
}

pub struct Cols<'a, T, const N: usize, const M: usize> {
    inner: &'a [[T; M]; N],
    col: usize,
    col_end: usize,
}

impl<T, const N: usize> Iterator for IntoLine<T, { N }> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoLine<T, { N }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoLine<T, { N }> {}
impl<T, const N: usize> FusedIterator for IntoLine<T, { N }> {}

impl<T, const N: usize, const M: usize> Iterator for IntoRows<T, { N }, { M }> {
    type Item = IntoLine<T, { M }>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(into_iter).map(IntoLine)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).map(into_iter).map(IntoLine)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T, const N: usize, const M: usize> DoubleEndedIterator for IntoRows<T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(into_iter).map(IntoLine)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).map(into_iter).map(IntoLine)
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for IntoRows<T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for IntoRows<T, { N }, { M }> {}

impl<T, const N: usize, const M: usize> Iterator for IntoCols<T, { N }, { M }> {
    type Item = IntoLine<T, { N }>;

    fn next(&mut self) -> Option<Self::Item> {
        self.arr
            .iter_mut()
            .map(|x| x.next())
            .collect::<Option<_>>()
            .map(ArrayVec::<T, { N }>::into_iter)
            .map(IntoLine)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.arr
            .iter_mut()
            .map(|x| x.nth(n))
            .collect::<Option<_>>()
            .map(ArrayVec::<T, { N }>::into_iter)
            .map(IntoLine)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if N == 0 {
            (0, Some(0))
        } else {
            self.arr[0].size_hint()
        }
    }
}

impl<T, const N: usize, const M: usize> DoubleEndedIterator for IntoCols<T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.arr
            .iter_mut()
            .map(|x| x.next_back())
            .collect::<Option<_>>()
            .map(ArrayVec::<T, { N }>::into_iter)
            .map(IntoLine)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.arr
            .iter_mut()
            .map(|x| x.nth_back(n))
            .collect::<Option<_>>()
            .map(ArrayVec::<T, { N }>::into_iter)
            .map(IntoLine)
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for IntoCols<T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for IntoCols<T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for RowMut<'a, T, { N }, { M }> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for RowMut<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for RowMut<'_, T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for RowMut<'_, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for RowsMut<'a, T, { N }, { M }> {
    type Item = RowMut<'a, T, { N }, { M }>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| RowMut(x.iter_mut()))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).map(|x| RowMut(x.iter_mut()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for RowsMut<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|x| RowMut(x.iter_mut()))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).map(|x| RowMut(x.iter_mut()))
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for RowsMut<'_, T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for RowsMut<'_, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for ColMut<'a, T, { N }, { M }> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_end {
            return None;
        }

        let val = unsafe {
            (*self.inner)
                .get_unchecked(self.row)
                .get_unchecked(self.col)
        };

        self.row += 1;

        Some(val)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.row.checked_add(n) {
            Some(row) if row < self.row_end => {
                self.row = row;
                self.next()
            }
            _ => {
                self.row = self.row_end;
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.row_end - self.row;

        (size, Some(size))
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for ColMut<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row == self.row_end {
            return None;
        }

        self.row_end -= 1;

        unsafe {
            Some(
                (*self.inner)
                    .get_unchecked(self.row_end)
                    .get_unchecked(self.col),
            )
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.row_end.checked_sub(n) {
            Some(row_end) if self.row < row_end => {
                self.row_end = row_end;
                self.next_back()
            }
            _ => {
                self.row = self.row_end;
                None
            }
        }
    }
}

impl<'a, T, const N: usize, const M: usize> ExactSizeIterator for ColMut<'a, T, { N }, { M }> {}
impl<'a, T, const N: usize, const M: usize> FusedIterator for ColMut<'a, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for ColsMut<'a, T, { N }, { M }> {
    type Item = ColMut<'a, T, { N }, { M }>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.col_end {
            return None;
        }

        let col = ColMut {
            inner: self.inner,
            col: self.col,
            row: 0,
            row_end: N,
            lt: PhantomData,
        };

        self.col += 1;

        Some(col)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.col.checked_add(n) {
            Some(col) if col < self.col_end => {
                self.col = col;
                self.next()
            }
            _ => {
                self.col = self.col_end;
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.col_end - self.col;

        (size, Some(size))
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for ColsMut<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.col == self.col_end {
            return None;
        }

        self.col_end -= 1;

        Some(ColMut {
            inner: self.inner,
            col: self.col_end,
            row: 0,
            row_end: N,
            lt: PhantomData,
        })
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.col_end.checked_sub(n) {
            Some(col_end) if self.col < col_end => {
                self.col_end = col_end;
                self.next_back()
            }
            _ => {
                self.col = self.col_end;
                None
            }
        }
    }
}

impl<'a, T, const N: usize, const M: usize> ExactSizeIterator for ColsMut<'a, T, { N }, { M }> {}
impl<'a, T, const N: usize, const M: usize> FusedIterator for ColsMut<'a, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for Row<'a, T, { N }, { M }> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for Row<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for Row<'_, T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for Row<'_, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for Rows<'a, T, { N }, { M }> {
    type Item = Row<'a, T, { N }, { M }>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| Row(x.iter()))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).map(|x| Row(x.iter()))
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for Rows<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|x| Row(x.iter()))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).map(|x| Row(x.iter()))
    }
}

impl<T, const N: usize, const M: usize> ExactSizeIterator for Rows<'_, T, { N }, { M }> {}
impl<T, const N: usize, const M: usize> FusedIterator for Rows<'_, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for Col<'a, T, { N }, { M }> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_end {
            return None;
        }

        let val = unsafe { self.inner.get_unchecked(self.row).get_unchecked(self.col) };

        self.row += 1;

        Some(val)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.row.checked_add(n) {
            Some(row) if row < self.row_end => {
                self.row = row;
                self.next()
            }
            _ => {
                self.row = self.row_end;
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.row_end - self.row;

        (size, Some(size))
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for Col<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row == self.row_end {
            return None;
        }

        self.row_end -= 1;

        unsafe {
            Some(
                self.inner
                    .get_unchecked(self.row_end)
                    .get_unchecked(self.col),
            )
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.row_end.checked_sub(n) {
            Some(row_end) if self.row < row_end => {
                self.row_end = row_end;
                self.next_back()
            }
            _ => {
                self.row = self.row_end;
                None
            }
        }
    }
}

impl<'a, T, const N: usize, const M: usize> ExactSizeIterator for Col<'a, T, { N }, { M }> {}
impl<'a, T, const N: usize, const M: usize> FusedIterator for Col<'a, T, { N }, { M }> {}

impl<'a, T, const N: usize, const M: usize> Iterator for Cols<'a, T, { N }, { M }> {
    type Item = Col<'a, T, { N }, { M }>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.col_end {
            return None;
        }

        let col = Col {
            inner: self.inner,
            col: self.col,
            row: 0,
            row_end: N,
        };

        self.col += 1;

        Some(col)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.col.checked_add(n) {
            Some(col) if col < self.col_end => {
                self.col = col;
                self.next()
            }
            _ => {
                self.col = self.col_end;
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.col_end - self.col;

        (size, Some(size))
    }
}

impl<'a, T, const N: usize, const M: usize> DoubleEndedIterator for Cols<'a, T, { N }, { M }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.col == self.col_end {
            return None;
        }

        self.col_end -= 1;

        let col = Col {
            inner: self.inner,
            col: self.col_end,
            row: 0,
            row_end: N,
        };

        Some(col)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self.col_end.checked_sub(n) {
            Some(col_end) if self.col < col_end => {
                self.col_end = col_end;
                self.next_back()
            }
            _ => {
                self.col = self.col_end;
                None
            }
        }
    }
}

impl<'a, T, const N: usize, const M: usize> ExactSizeIterator for Cols<'a, T, { N }, { M }> {}
impl<'a, T, const N: usize, const M: usize> FusedIterator for Cols<'a, T, { N }, { M }> {}

impl<T, const N: usize, const M: usize> Matrix<T, { N }, { M }> {
    pub fn into_rows(self) -> IntoRows<T, { N }, { M }> {
        IntoRows {
            inner: into_iter(self.0),
        }
    }

    pub fn into_cols(self) -> IntoCols<T, { N }, { M }> {
        IntoCols {
            arr: into_iter(self.0).map(into_iter).collect(),
        }
    }

    pub fn rows_mut(&mut self) -> RowsMut<T, { N }, { M }> {
        RowsMut {
            inner: self.0.iter_mut(),
        }
    }

    pub fn cols_mut(&mut self) -> ColsMut<T, { N }, { M }> {
        ColsMut {
            inner: &mut self.0,
            col: 0,
            col_end: N,
            lt: PhantomData,
        }
    }

    pub fn rows(&self) -> Rows<T, { N }, { M }> {
        Rows {
            inner: self.0.iter(),
        }
    }

    pub fn cols(&self) -> Cols<T, { N }, { M }> {
        Cols {
            inner: &self.0,
            col: 0,
            col_end: N,
        }
    }
}

impl<'a, T, const N: usize, const M: usize> Clone for Row<'a, T, { N }, { M }> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, T, const N: usize, const M: usize> Clone for Col<'a, T, { N }, { M }> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T, const N: usize, const M: usize> Clone for Rows<'a, T, { N }, { M }> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T, const N: usize, const M: usize> Clone for Cols<'a, T, { N }, { M }> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}
