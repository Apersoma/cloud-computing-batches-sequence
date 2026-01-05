

mod phi_x_omicron;
mod benches;

#[cfg(test)]
use std::iter::Step;

#[cfg(test)]
#[allow(unused_imports)]
pub use phi_x_omicron::*;

#[cfg(test)]
pub fn stepped_iter<K: Step+Copy, const L: usize>(bounds: &[K; L]) -> impl Iterator<Item = (usize, K)> {
    bounds
        .array_windows::<2>()
        .map(|p| (p[0]..p[1]).rev())
        .rev()
        .enumerate()
        .flat_map(|v|v.1.map(move |e|(v.0, e)))
}