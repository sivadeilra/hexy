#![allow(non_camel_case_types, incomplete_features, unused)]
#![feature(tuple_trait, generic_const_exprs)]

mod constructs;
mod functions;

use crate::constructs::*;
use crate::functions::*;

fn hex_id(uv: vec2) -> vec2 {
    let r = vec2((sqrt(3.0), 1.0));
    let h = r / 2.0;

    let a = rem(uv, r) - h;
    let b = rem(uv - h, r) - h;

    let gv = if dot(a, a) < dot(b, b) { a } else { b };

    uv - gv
}

fn from_euclidean(pos: vec2) -> vec2 {
    let compx = sqrt(3.0) * pos.x / 2.0;
    let compy = pos.y / 2.0;

    //return vec2(compy + compx, compy + compx);
    mat2((1.0 / 3.0, -1.0 / 3.0, sqrt(1.0 / 3.0), sqrt(1.0 / 3.0))) * pos
}

fn to_euclidean(axial: vec2) -> vec2 {
    mat2((3.0 / 2.0, sqrt(3.0) / 2.0, -3.0 / 2.0, sqrt(3.0) / 2.0)) * axial
}

fn calc(mut uv: vec2) -> vec2 {
    let scale = 10.0;

    uv *= scale;
    //uv.y *= -1.0;

    from_euclidean(hex_id(uv)) / 4.0
}

fn main() {
    dbg!(from_euclidean(to_euclidean(vec2((1.0, 0.0)))));
    //dbg!(to_euclidean(vec2((1.0, 0.0))));
}
