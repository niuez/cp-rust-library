use crate::math::complex::Complex;

pub fn fast_fourier_transform(arr: &[Complex]) -> Vec<Complex> {
    let n = arr.len();
    assert!(n.count_ones() == 1, "the length of array is not square");
    let mut a: Vec<_> = arr.to_vec();
    let bit = n.trailing_zeros();
 
    for si in (0..bit).rev() {
        let s = (1 << si) as usize;
        let zeta = Complex::polar(1.0, std::f64::consts::PI * 2.0 / (s << 1) as f64);
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = Complex::new(1.0, 0.0);
            for j in 0..s {
                let t = a[i + j] - a[s + i + j];
                a[i + j] = a[i + j] + a[s + i + j];
                a[s + i + j] = t * z_i;
                z_i *= zeta;
            }
        }
    }
    a
}

pub fn inverse_fast_fourier_transform(arr: &[Complex]) -> Vec<Complex> {
    let n = arr.len();
    assert!(n.count_ones() == 1, "the length of array is not square");

    let mut a: Vec<_> = arr.to_vec();
    let bit = n.trailing_zeros();
    for si in 0..bit {
        let s = (1 << si) as usize;
        let zeta = Complex::polar(1.0, std::f64::consts::PI * -2.0 / (s << 1) as f64);
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = Complex::new(1.0, 0.0);
            for j in 0..s {
                let t = a[s + i + j] * z_i;
                a[s + i + j] = a[i + j] - t;
                a[i + j] = a[i + j] + t;
                z_i *= zeta;
            }
        }
    }
    let inv_n = Complex::new(1f64 / n as f64, 0f64);
    a.iter().map(|&x| x * inv_n).collect()
}

#[test]
fn fft_test() {
    let n = 4;
    let m = 8;
    let x = [1, 2, 3, 4];
    let y = [1, 2, 4, 8];
    let mut a = vec![Complex::new(0.0, 0.0); m];
    let mut b = vec![Complex::new(0.0, 0.0); m];
    for i in 0..n {
        a[i] = Complex::new(x[i] as f64, 0.0);
        b[i] = Complex::new(y[i] as f64, 0.0);
    }
    let af = fast_fourier_transform(&a);
    let bf = fast_fourier_transform(&b);
    let cf: Vec<_> = (0..m).map(|i| af[i] * bf[i]).collect();
    let cf = inverse_fast_fourier_transform(&cf);
    assert!(cf.split_at(7).0.iter().map(|x| (x.x + 0.5) as u64).collect::<Vec<_>>() == vec![1, 4, 11, 26, 36, 40, 32]);
}
