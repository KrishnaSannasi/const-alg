#![feature(const_generics, specialization)]

use std::convert::TryInto;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod test;

pub struct ArrayVec<T, const N: usize> {
    arr: MaybeUninit<[T; N]>,
    len: usize,
}

impl<T, const N: usize> Default for ArrayVec<T, { N }> {
    fn default() -> Self {
        ArrayVec {
            arr: MaybeUninit::uninit(),
            len: 0,
        }
    }
}

impl<T, const N: usize> ArrayVec<T, { N }> {
    pub fn as_ptr(&self) -> *const T {
        self.arr.as_ptr() as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.arr.as_mut_ptr() as *mut T
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            unsafe {
                self.as_mut_ptr().add(self.len).write(value);
            }
            self.len += 1;
            Ok(())
        } else {
            Err(value)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if N == 0 {
            return None;
        }
        self.len = self.len.checked_sub(1)?;

        unsafe { Some(self.as_ptr().add(self.len).read()) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn clear(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.as_mut_slice());
        }
        self.len = 0;
    }

    pub unsafe fn into_array_unchecked(mut self) -> [T; N] {
        self.len = 0;
        std::mem::replace(&mut self.arr, MaybeUninit::uninit()).assume_init()
    }
}

impl<T: Clone, const N: usize> Clone for ArrayVec<T, { N }> {
    default fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T: Copy, const N: usize> Clone for ArrayVec<T, { N }> {
    fn clone(&self) -> Self {
        let mut arr = ArrayVec::<T, { N }>::default();

        unsafe {
            std::ptr::copy_nonoverlapping(self.as_ptr(), arr.as_mut_ptr(), self.len);

            arr.len = self.len;
        }

        arr
    }
}

impl<T, const N: usize> Deref for ArrayVec<T, { N }> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T, const N: usize> DerefMut for ArrayVec<T, { N }> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}

impl<T, const N: usize> Drop for ArrayVec<T, { N }> {
    fn drop(&mut self) {
        self.clear()
    }
}

impl<T, const N: usize> From<[T; N]> for ArrayVec<T, { N }> {
    fn from(arr: [T; N]) -> Self {
        Self {
            arr: MaybeUninit::new(arr),
            len: N,
        }
    }
}

impl<T, const N: usize> TryInto<[T; N]> for ArrayVec<T, { N }> {
    type Error = Self;

    fn try_into(self) -> Result<[T; N], Self> {
        if self.len == N {
            unsafe { Ok(self.into_array_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl<T, const N: usize> IntoIterator for ArrayVec<T, { N }> {
    type Item = T;
    type IntoIter = IntoIter<T, { N }>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            arr: ManuallyDrop::new(self),
            idx: 0,
        }
    }
}

pub struct IntoIter<T, const N: usize> {
    arr: ManuallyDrop<ArrayVec<T, { N }>>,
    idx: usize,
}

impl<T: Clone, const N: usize> Clone for IntoIter<T, { N }> {
    default fn clone(&self) -> Self {
        IntoIter {
            arr: ManuallyDrop::new(self.arr[self.idx..].iter().cloned().collect()),
            idx: 0,
        }
    }
}

impl<T: Copy, const N: usize> Clone for IntoIter<T, { N }> {
    fn clone(&self) -> Self {
        let mut arr = ArrayVec::<T, { N }>::default();

        unsafe {
            std::ptr::copy_nonoverlapping(
                self.arr.as_ptr().add(self.idx),
                arr.as_mut_ptr(),
                self.arr.len - self.idx,
            );

            arr.len = self.arr.len - self.idx;
        }

        IntoIter {
            arr: ManuallyDrop::new(arr),
            idx: 0,
        }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, { N }> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if N == 0 || self.arr.len == self.idx {
            None
        } else {
            let output = unsafe { self.arr.as_ptr().add(self.idx).read() };

            self.idx += 1;

            Some(output)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.arr.len - self.idx;

        (size, Some(size))
    }

    fn nth(&mut self, n: usize) -> Option<T> {
        match self.idx.checked_add(n) {
            Some(idx) if idx < self.arr.len => {
                unsafe {
                    std::ptr::drop_in_place(&mut self.arr[self.idx..idx]);
                }
                self.idx = idx;
                self.next()
            }
            _ => {
                unsafe {
                    std::ptr::drop_in_place(&mut self.arr[self.idx..]);
                }
                self.idx = self.arr.len;
                None
            }
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, { N }> {}
impl<T, const N: usize> std::iter::FusedIterator for IntoIter<T, { N }> {}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, { N }> {
    fn next_back(&mut self) -> Option<T> {
        if self.idx == self.arr.len {
            None
        } else {
            self.arr.pop()
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<T> {
        match self.arr.len.checked_sub(n) {
            Some(len) if self.idx < len => {
                unsafe {
                    std::ptr::drop_in_place(&mut self.arr[len..]);
                }
                self.arr.len = len;
                self.next_back()
            }
            _ => {
                unsafe {
                    std::ptr::drop_in_place(&mut self.arr[self.idx..]);
                }
                self.idx = self.arr.len;
                None
            }
        }
    }
}

impl<T, const N: usize> Extend<T> for ArrayVec<T, { N }> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let _ = iter.into_iter().take(N).try_fold((), |_, x| self.push(x));
    }
}

impl<T, const N: usize> std::iter::FromIterator<T> for ArrayVec<T, { N }> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = ArrayVec::<T, { N }>::default();

        arr.extend(iter);

        arr
    }
}

impl<T, const N: usize> Drop for IntoIter<T, { N }> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(
                self.arr.as_mut_ptr().add(self.idx),
                self.arr.len - self.idx,
            ))
        }
    }
}
