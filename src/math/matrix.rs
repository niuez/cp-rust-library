use crate::algebra::Field;

pub struct Matrix2D<F> {
    a: Box<[F]>,
    h: usize,
    w: usize,
}


impl<F: Field> Matrix2D<F> {
    pub fn zero(h: usize, w: usize) -> Self {
        Self {
            a: vec![F::zero(); w * h].into_boxed_slice(),
            h,
            w,
        }
    }
}

impl<F> std::ops::Index<usize> for Matrix2D<F> {
    type Output = [F];
    fn index(&self, i: usize) -> &Self::Output {
        let s = i * self.w;
        &self.a[s..s+self.w]
    }
}

impl<F> std::ops::IndexMut<usize> for Matrix2D<F> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let s = i * self.w;
        &mut self.a[s..s+self.w]
    }
}

impl<F: Field> std::ops::Add<Self> for Matrix2D<F> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        assert_eq!(self.h, rhs.h, "the heights of the two matrices is different.");
        assert_eq!(self.w, rhs.w, "the width of the two matrices is different.");
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] += rhs[i][j];
            }
        }
        self
    }
}

impl<F: Field> std::ops::Sub<Self> for Matrix2D<F> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        assert_eq!(self.h, rhs.h, "the heights of the two matrices is different.");
        assert_eq!(self.w, rhs.w, "the width of the two matrices is different.");
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] -= rhs[i][j];
            }
        }
        self
    }
}

impl<F: Field> std::ops::Mul<Self> for Matrix2D<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.w, rhs.h, "can't multiply because left.w and right.h are not equal.");
        let mut res = Matrix2D::zero(self.h, rhs.w);
        for i in 0..self.h {
            let c = &mut res[i];
            for k in 0..self.w {
                let a = self[i][k];
                let b = &rhs[k];
                for j in 0..rhs.w {
                    c[j] += a * b[j];
                }
            }
        }
        res
    }
}

impl<F: Field> std::ops::Mul<F> for Matrix2D<F> {
    type Output = Self;
    fn mul(mut self, rhs: F) -> Self {
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] *= rhs;
            }
        }
        self
    }
}
