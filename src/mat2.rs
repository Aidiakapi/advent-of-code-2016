// Matrix 2D backed by a vector
#![allow(dead_code)]
use crate::vec2::Vec2us;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mat2<T: Clone> {
    pub data: Vec<T>,
    size: Vec2us,
}

impl<T: Clone> Mat2<T> {
    pub fn new(item: T, size: Vec2us) -> Self {
        Mat2 {
            data: vec![item; size.x * size.y],
            size,
        }
    }

    #[rustfmt::skip] #[inline(always)] pub fn size(&self) -> Vec2us { self.size }
    #[rustfmt::skip] #[inline(always)] pub fn width(&self) -> usize { self.size.x }
    #[rustfmt::skip] #[inline(always)] pub fn height(&self) -> usize { self.size.y }
}

impl<T: Clone> Index<usize> for Mat2<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &self.data[base..base + self.size.y]
    }
}

impl<T: Clone> IndexMut<usize> for Mat2<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        assert!(index < self.size.x);
        let base = index * self.size.y;
        &mut self.data[base..base + self.size.y]
    }
}

impl<T: Clone> Index<Vec2us> for Mat2<T> {
    type Output = T;

    fn index(&self, index: Vec2us) -> &T {
        assert!(index.x < self.size.x);
        &self.data[index.x * self.size.y + index.y]
    }
}

impl<T: Clone> IndexMut<Vec2us> for Mat2<T> {
    fn index_mut(&mut self, index: Vec2us) -> &mut T {
        assert!(index.x < self.size.x);
        &mut self.data[index.x * self.size.y + index.y]
    }
}
