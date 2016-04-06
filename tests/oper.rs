#[macro_use(s)] extern crate ndarray;
extern crate num as libnum;

use ndarray::RcArray;
use ndarray::{arr0, rcarr1, rcarr2};
use ndarray::{
    OwnedArray,
    Ix, Ixs,
    AsArray,
    NdFloat,
    aview0,
};

use std::fmt;
use libnum::Float;

fn test_oper(op: &str, a: &[f32], b: &[f32], c: &[f32])
{
    let aa = rcarr1(a);
    let bb = rcarr1(b);
    let cc = rcarr1(c);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
    let dim = (2, 2);
    let aa = aa.reshape(dim);
    let bb = bb.reshape(dim);
    let cc = cc.reshape(dim);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
    let dim = (1, 2, 1, 2);
    let aa = aa.reshape(dim);
    let bb = bb.reshape(dim);
    let cc = cc.reshape(dim);
    test_oper_arr(op, aa.clone(), bb.clone(), cc.clone());
}

fn test_oper_arr<A: Float + fmt::Debug, D: ndarray::Dimension>
    (op: &str, mut aa: RcArray<A,D>, bb: RcArray<A, D>, cc: RcArray<A, D>)
{
    match op {
        "+" => {
            assert_eq!(&aa + &bb, cc);
            aa.iadd(&bb);
            assert_eq!(aa, cc);
        },
        "-" => {
            assert_eq!(&aa - &bb, cc);
            aa.isub(&bb);
            assert_eq!(aa, cc);
        },
        "*" => {
            assert_eq!(&aa * &bb, cc);
            aa.imul(&bb);
            assert_eq!(aa, cc);
        },
        "/" => {
            assert_eq!(&aa / &bb, cc);
            aa.idiv(&bb);
            assert_eq!(aa, cc);
        },
        "%" => {
            assert_eq!(&aa % &bb, cc);
            aa.irem(&bb);
            assert_eq!(aa, cc);
        },
        "neg" => {
            assert_eq!(-aa.clone(), cc);
            aa.ineg();
            assert_eq!(aa, cc);
        },
        _ => panic!()
    }
}

#[test]
fn operations()
{
    test_oper("+", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[1.0,3.0,5.0,7.0]);
    test_oper("-", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[1.0,1.0,1.0,1.0]);
    test_oper("*", &[1.0,2.0,3.0,4.0], &[0.0, 1.0, 2.0, 3.0], &[0.0,2.0,6.0,12.0]);
    test_oper("/", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[1.0,2.0,3.0/2.0,4.0/3.0]);
    test_oper("%", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[0.0,0.0,1.0,1.0]);
    test_oper("neg", &[1.0,2.0,3.0,4.0], &[1.0, 1.0, 2.0, 3.0], &[-1.0,-2.0,-3.0,-4.0]);
}

#[test]
fn scalar_operations()
{
    let a = arr0::<f32>(1.);
    let b = rcarr1::<f32>(&[1., 1.]);
    let c = rcarr2(&[[1., 1.], [1., 1.]]);

    {
        let mut x = a.clone();
        let mut y = arr0(0.);
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, a + arr0(1.));
        assert_eq!(x, y);
    }

    {
        let mut x = b.clone();
        let mut y = rcarr1(&[0., 0.]);
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, b + arr0(1.));
        assert_eq!(x, y);
    }

    {
        let mut x = c.clone();
        let mut y = RcArray::zeros((2, 2));
        x.iadd_scalar(&1.);
        y.assign_scalar(&2.);
        assert_eq!(x, c + arr0(1.));
        assert_eq!(x, y);
    }
}

fn assert_approx_eq<F: fmt::Debug + Float>(f: F, g: F, tol: F) -> bool {
    assert!((f - g).abs() <= tol, "{:?} approx== {:?} (tol={:?})",
            f, g, tol);
    true
}

fn reference_dot<'a,A, V1, V2>(a: V1, b: V2) -> A
    where A: NdFloat,
          V1: AsArray<'a, A>,
          V2: AsArray<'a, A>,
{
    let a = a.into();
    let b = b.into();
    a.iter().zip(b.iter()).fold(A::zero(), |acc, (&x, &y)| {
        acc + x * y
    })
}

#[test]
fn dot_product() {
    let a = OwnedArray::range(0., 69., 1.);
    let b = &a * 2. - 7.;
    let dot = 197846.;
    assert_approx_eq(a.dot(&b), reference_dot(&a, &b), 1e-5);

    // test different alignments
    let max = 8 as Ixs;
    for i in 1..max {
        let a1 = a.slice(s![i..]);
        let b1 = b.slice(s![i..]);
        assert_approx_eq(a1.dot(&b1), reference_dot(&a1, &b1), 1e-5);
        let a2 = a.slice(s![..-i]);
        let b2 = b.slice(s![i..]);
        assert_approx_eq(a2.dot(&b2), reference_dot(&a2, &b2), 1e-5);
    }


    let a = a.map(|f| *f as f32);
    let b = b.map(|f| *f as f32);
    assert_approx_eq(a.dot(&b), dot as f32, 1e-5);

    let max = 8 as Ixs;
    for i in 1..max {
        let a1 = a.slice(s![i..]);
        let b1 = b.slice(s![i..]);
        assert_approx_eq(a1.dot(&b1), reference_dot(&a1, &b1), 1e-5);
        let a2 = a.slice(s![..-i]);
        let b2 = b.slice(s![i..]);
        assert_approx_eq(a2.dot(&b2), reference_dot(&a2, &b2), 1e-5);
    }

    let a = a.map(|f| *f as i32);
    let b = b.map(|f| *f as i32);
    assert_eq!(a.dot(&b), dot as i32);
}

// test that we can dot product with a broadcast array
#[test]
fn dot_product_0() {
    let a = OwnedArray::range(0., 69., 1.);
    let x = 1.5;
    let b = aview0(&x);
    let b = b.broadcast(a.dim()).unwrap();
    assert_approx_eq(a.dot(&b), reference_dot(&a, &b), 1e-5);

    // test different alignments
    let max = 8 as Ixs;
    for i in 1..max {
        let a1 = a.slice(s![i..]);
        let b1 = b.slice(s![i..]);
        assert_approx_eq(a1.dot(&b1), reference_dot(&a1, &b1), 1e-5);
        let a2 = a.slice(s![..-i]);
        let b2 = b.slice(s![i..]);
        assert_approx_eq(a2.dot(&b2), reference_dot(&a2, &b2), 1e-5);
    }
}

#[test]
fn dot_product_neg_stride() {
    // test that we can dot with negative stride
    let a = OwnedArray::range(0., 69., 1.);
    let b = &a * 2. - 7.;
    for stride in -10..0 {
        // both negative
        let a = a.slice(s![..;stride]);
        let b = b.slice(s![..;stride]);
        assert_approx_eq(a.dot(&b), reference_dot(&a, &b), 1e-5);
    }
    for stride in -10..0 {
        // mixed
        let a = a.slice(s![..;-stride]);
        let b = b.slice(s![..;stride]);
        assert_approx_eq(a.dot(&b), reference_dot(&a, &b), 1e-5);
    }
}

fn range_mat(m: Ix, n: Ix) -> OwnedArray<f32, (Ix, Ix)> {
    OwnedArray::linspace(0., (m * n - 1) as f32, m * n).into_shape((m, n)).unwrap()
}

#[cfg(has_assign)]
#[test]
fn mat_mul() {
    let (m, n, k) = (8, 8, 8);
    let a = range_mat(m, n);
    let b = range_mat(n, k);
    let mut b = b / 4.;
    {
        let mut c = b.column_mut(0);
        c += 1.0;
    }
    let ab = a.mat_mul(&b);

    let mut af = OwnedArray::zeros_f(a.dim());
    let mut bf = OwnedArray::zeros_f(b.dim());
    af.assign(&a);
    bf.assign(&b);

    assert_eq!(ab, a.mat_mul(&bf));
    assert_eq!(ab, af.mat_mul(&b));
    assert_eq!(ab, af.mat_mul(&bf));

    let (m, n, k) = (10, 5, 11);
    let a = range_mat(m, n);
    let b = range_mat(n, k);
    let mut b = b / 4.;
    {
        let mut c = b.column_mut(0);
        c += 1.0;
    }
    let ab = a.mat_mul(&b);

    let mut af = OwnedArray::zeros_f(a.dim());
    let mut bf = OwnedArray::zeros_f(b.dim());
    af.assign(&a);
    bf.assign(&b);

    assert_eq!(ab, a.mat_mul(&bf));
    assert_eq!(ab, af.mat_mul(&b));
    assert_eq!(ab, af.mat_mul(&bf));

    let (m, n, k) = (10, 8, 1);
    let a = range_mat(m, n);
    let b = range_mat(n, k);
    let mut b = b / 4.;
    {
        let mut c = b.column_mut(0);
        c += 1.0;
    }
    let ab = a.mat_mul(&b);

    let mut af = OwnedArray::zeros_f(a.dim());
    let mut bf = OwnedArray::zeros_f(b.dim());
    af.assign(&a);
    bf.assign(&b);

    assert_eq!(ab, a.mat_mul(&bf));
    assert_eq!(ab, af.mat_mul(&b));
    assert_eq!(ab, af.mat_mul(&bf));
}

// Check that matrix multiplication of contiguous matrices returns a
// matrix with the same order 
#[test]
fn mat_mul_order() {
    let (m, n, k) = (8, 8, 8);
    let a = range_mat(m, n);
    let b = range_mat(n, k);
    let mut af = OwnedArray::zeros_f(a.dim());
    let mut bf = OwnedArray::zeros_f(b.dim());
    af.assign(&a);
    bf.assign(&b);

    let cc = a.mat_mul(&b);
    let ff = af.mat_mul(&bf);

    assert_eq!(cc.strides()[1], 1);
    assert_eq!(ff.strides()[0], 1);
}

// Check that matrix multiplication
// supports broadcast arrays.
#[test]
fn mat_mul_broadcast() {
    let (m, n, k) = (16, 16, 16);
    let a = range_mat(m, n);
    let x1 = 1.;
    let x = OwnedArray::from_vec(vec![x1]);
    let b0 = x.broadcast((n, k)).unwrap();
    let b1 = OwnedArray::from_elem(n, x1);
    let b1 = b1.broadcast((n, k)).unwrap();
    let b2 = OwnedArray::from_elem((n, k), x1);

    let c2 = a.mat_mul(&b2);
    let c1 = a.mat_mul(&b1);
    let c0 = a.mat_mul(&b0);
    assert_eq!(c2, c1);
    assert_eq!(c2, c0);
}

// Check that matrix multiplication supports reversed axes
#[test]
fn mat_mul_rev() {
    let (m, n, k) = (16, 16, 16);
    let a = range_mat(m, n);
    let b = range_mat(n, k);
    let mut rev = OwnedArray::zeros(b.dim());
    let mut rev = rev.slice_mut(s![..;-1, ..]);
    rev.assign(&b);
    println!("{:.?}", rev);

    let c1 = a.mat_mul(&b);
    let c2 = a.mat_mul(&rev);
    assert_eq!(c1, c2);
}

#[test]
fn scaled_add() {
    let a = range_mat(16, 15);
    let mut b = range_mat(16, 15);
    b.mapv_inplace(f32::exp);

    let alpha = 0.2_f32;
    let mut c = a.clone();
    c.scaled_add(alpha, &b);

    let d = alpha * &b + &a;
    assert_eq!(c, d);

}
