use crate::math::fps_multiply::*;
use crate::math::modint::*;
use crate::math::convolution::number_theoretic_transform::*;
use crate::math::garner::garner;

pub struct NttArbMultiply<M>(std::marker::PhantomData<M>);

impl<M: Mod> FpsMultiply for NttArbMultiply<M> {
    type Target = ModInt<M>;
    type Output = (Vec<ModInt<NttMod998244353>>, Vec<ModInt<NttMod935329793>>, Vec<ModInt<NttMod950009857>>);
    fn dft(arr: Vec<Self::Target>) -> Self::Output {
        let f = number_theoretic_transform(arr.iter().map(|x| ModInt::<NttMod998244353>::new(x.value())).collect());
        let g = number_theoretic_transform(arr.iter().map(|x| ModInt::<NttMod935329793>::new(x.value())).collect());
        let h = number_theoretic_transform(arr.iter().map(|x| ModInt::<NttMod950009857>::new(x.value())).collect());
        (f, g, h)
    }
    fn idft(arr: Self::Output) -> Vec<Self::Target> {
        let (f, g, h) = arr;
        let f = inverse_number_theoretic_transform(f);
        let g = inverse_number_theoretic_transform(g);
        let h = inverse_number_theoretic_transform(h);
        let mut ans = vec![ModInt::<M>::new(0); f.len()];
        for i in 0..f.len() {
            ans[i] = ModInt::<M>::new(garner([
                                             (f[i].value(), NttMod998244353::m()),
                                             (g[i].value(), NttMod935329793::m()),
                                             (h[i].value(), NttMod950009857::m()),
                                            ].to_vec(), M::m()));
        }
        ans
    }
    fn multiply(mut a: Self::Output, b: Self::Output) -> Self::Output {
        for i in 0..a.0.len() { a.0[i] *= b.0[i]; }
        for i in 0..a.1.len() { a.1[i] *= b.1[i]; }
        for i in 0..a.2.len() { a.2[i] *= b.2[i]; }
        a
    }
    fn multiply_self(mut a: Self::Output) -> Self::Output {
        for i in 0..a.0.len() { a.0[i] = a.0[i] * a.0[i]; }
        for i in 0..a.1.len() { a.1[i] = a.1[i] * a.1[i]; }
        for i in 0..a.2.len() { a.2[i] = a.2[i] * a.2[i]; }
        a
    }
}
