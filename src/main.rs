extern crate nalgebra as na;
extern crate rand;

use nalgebra::Matrix;
use rand::Rng;
use core::ops::{AddAssign, Neg};
use std::marker::PhantomData;

fn gen_affine_m_h_l(h: Vec<i32>, l: i32, n: usize, k: usize) -> (Matrix<i32, 1,0, >, Vec<Matrix<i32>>) {
    //from section 5.2 of Bartusek, “Affine determinant programs”)

    let mut rng = rand::thread_rng();

    let max = u32::MAX;
    let mut r: Matrix<i32> = Matrix::new(0,max, max, PhantomData);
    rng.fill(&mut r);

    let a: Matrix<i32> = -(&r * l);

    let b: Vec<Matrix<i32>> = h.into_iter()
        .map(|hi| &r*hi)
        .collect();
    (a,b)
}

fn encrypt(h: Vec<i32>, l: i32, b: Vec<i32>, n: usize, k: usize, message: i32) -> Matrix<i32> {
    let(a, bi) = gen_affine_m_h_l(h, l, n, k);
    let mut rng = rand::thread_rng();

    //sample s matrix from uniform distribution
    let mut s: Matrix<i32> = Matrix();
    rng.fill(&mut s);

    //encryption part
    let mut m_h_l_b: Matrix<i32> = a + (s*message);
    for(i, bi) in bi.iter().enumerate() {
        m_h_l_b[(i,0)].add_assign(bi);
    }
    m_h_l_b
}

fn decrypt(ciphertext: Matrix<i32>, witness: Vec<i32>) -> Option<i32> {
    // simply check whether determinant of M_h_l_b(witness) is non-zero
    if(ciphertext.determinant()==0) {
        Some(ciphertext[(0,0)])
    }
    else {
        None
    }
}

fn main() {
    //generate the parameters h and l and use encrypt() and decrypt() as required in the WE scheme
}