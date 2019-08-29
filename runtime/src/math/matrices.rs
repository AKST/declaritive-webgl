use std::cmp::{Ord};
use std::ops::{Index, IndexMut, Mul, Add};

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<T> {
  height: usize,
  width: usize,
  data: Vec<T>,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
  type Output = T;

  fn index(&self, position: (usize, usize)) -> &Self::Output {
    let (x, y) = position;
    assert!(self.width > x, "invalid x position for matrix");
    assert!(self.height > y, "invalid x position for matrix");
    let index = (y * self.width) + x;
    &self.data[index]
  }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut<'a>(&'a mut self, position: (usize, usize)) -> &'a mut Self::Output {
    let (x, y) = position;
    assert!(self.width > x, "invalid x position for matrix");
    assert!(self.height > y, "invalid x position for matrix");
    let index = (y * self.width) + x;
    &mut self.data[index]
  }
}

impl<T> Index<usize> for Vector<T> {
  type Output = T;

  fn index(&self, position: usize) -> &Self::Output {
    &self.data[position]
  }
}

impl<T> Matrix<T> {
  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }
}

impl<T> Matrix<T> {
  pub fn new(width: usize, height: usize, data: Vec<T>) -> Matrix<T> {
    assert!(width * height == data.len(), "invalid dimensions for matrix data");
    Matrix { data, width, height }
  }
}

impl<T> Matrix<T> where T: Default + Clone {
  pub fn from_dimensions(width: usize, height: usize) -> Matrix<T> {
    let data: Vec<T> = vec![T::default(); width * height];
    Matrix::new(width, height, data)
  }
}

impl<T> Matrix<T> where T: Mul<Output=T> + Add<Output=T> + Default + Copy + Clone {
  pub fn mul(&self, other: &Matrix<T>) -> Matrix<T> {
    let width = self.width.min(other.width);
    let height = self.height.min(other.height);
    let mut data = vec![T::default(); width * height];

    let rows: Vec<Vector<T>> = (0..height).map(|y| self.row(y)).collect();
    let cols: Vec<Vector<T>> = (0..height).map(|y| other.column(y)).collect();

    (0..width).for_each(|x| {
      (0..height).for_each(|y| {
        let col = &cols[x];
        let row = &rows[y];
        let write = x + (y * width);
        data[write] = col.dot_product(row);
      });
    });

    Matrix { width, height, data }
  }
}

impl<T> Matrix<T> where T: Copy {
  pub fn column(&self, x: usize) -> Vector<T> {
    assert!(self.width > x, "x is greater than vector width");
    let data: Vec<T> = (0..self.height)
      .map(|y| self.data[x + (y * self.width)])
      .collect();
    Vector { length: self.height, data }
  }

  pub fn row(&self, y: usize) -> Vector<T> {
    assert!(self.height > y, "y is greater than vector height");
    let data: Vec<T> = (0..self.width)
      .map(|x| self.data[x + (y * self.width)])
      .collect();
    Vector { length: self.height, data }
  }
}

#[derive(Clone, Debug)]
pub struct Vector<T> {
  length: usize,
  data: Vec<T>,
}

impl<T> Vector<T> where T: Mul<Output=T> + Add<Output=T> + Default + Copy + Clone {
  fn dot_product(&self, other: &Vector<T>) -> T {
    let length = self.length.min(other.length);
    return (0..length)
      .map(|i| self.data[i] * other.data[i])
      .fold(T::default(), |a, b| a + b)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn matrix_from_dimensions() {
    let m = Matrix::<u32>::from_dimensions(3, 3);
    assert_eq!(m.width(), 3);
    assert_eq!(m.height(), 3);
    assert_eq!(m[(2, 2)], u32::default());
  }

  #[test]
  fn matrix_mul() {
    let d1 = vec![1, 2, 3, 4];
    let m1 = Matrix::new(2, 2, d1);

    let d2 = vec![1, 2, 5, 6];
    let m2 = Matrix::new(2, 2, d2);

    let m3 = m1.mul(&m2);

    let aa = 1*1 + 2*5;
    let ab = 1*2 + 2*6;
    let ba = 3*1 + 4*5;
    let bb = 3*2 + 4*6;
    let multipled_d = vec![aa, ab, ba, bb];
    let expected = Matrix::new(2, 2, multipled_d);
    assert_eq!(m3, expected);
  }
}
