pub fn garner(x: &Vec<u64>, mods: &Vec<u64>, m: u64) -> u64 {
    let mut mods = mods.clone();
    mods.push(m);
    let mut coeffs = vec![1u64; mods.len()];
    let mut constants = vec![0u64; mods.len()];
    let inv = |a, mo| {
        let mut exp = mo - 2;
        let mut now = a;
        let mut ans = 1;
        while exp > 0 {
            if (exp & 1) == 1 { ans = (ans * now) % mo; }
            now = (now * now) % mo;
            exp >>= 1;
        }
        ans
    };
    for i in 0..x.len() {
        let a = x[i] + mods[i] - constants[i];
        let a = if a > mods[i] { a - mods[i] } else { a };
        let v = a * inv(coeffs[i], mods[i]) % mods[i];
        for j in i + 1..mods.len() {
            constants[j] = (constants[j] + coeffs[j] * v) % mods[j];
            coeffs[j] = (coeffs[j] * mods[i]) % mods[j];
        }
    }
    constants[x.len()]
}
