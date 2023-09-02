use core::mem::MaybeUninit;

use crate::constructs::*;

fn trans<const P: usize, const N: usize, T: Components<N>>(
    value: [T; P],
    clos: impl Fn([T::Comp; P]) -> T::Comp,
) -> T
where
    [(); N]: Sized,
{
    let compset = value.map(Components::components);

    let mut out = [MaybeUninit::uninit(); N];

    for i in 0..N {
        out[i].write(clos(compset.map(|comps| comps[i])));
    }

    // Unsafe: Initialized
    T::from_components(unsafe { core::mem::transmute_copy(&out) })
}

fn reduce<const P: usize, const N: usize, U, T: Components<N>>(
    value: [T; P],
    mut init: U,
    clos: impl Fn([T::Comp; P], &mut U),
) -> U
where
    [(); N]: Sized,
{
    let compset = value.map(Components::components);

    for i in 0..N {
        clos(compset.map(|comps| comps[i]), &mut init);
    }

    init
}

// binary

pub fn dot<const N: usize, T: Components<N, Comp = f64>>(lhs: T, rhs: T) -> f64
where
    [(); N]: Sized,
{
    reduce([lhs, rhs], 0.0, |[lhc, rhc], acc| *acc += lhc * rhc)
}

pub fn rem<const N: usize, T: Components<N, Comp = f64>>(lhs: T, rhs: T) -> T
where
    [(); N]: Sized,
{
    trans([lhs, rhs], |[lhc, rhc]| lhc.rem_euclid(rhc))
}

// unary

pub fn sqrt<const N: usize, T: Components<N, Comp = f64>>(value: T) -> T
where
    [(); N]: Sized,
{
    trans([value], |[comp]| comp.sqrt())
}
