use rand::random; 
use std::iter::from_fn;
use num::Complex;
use std::iter::successors;

pub fn iterator2_work(){


    let lengths: Vec<f64> =
        from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();

    for len in lengths{
        println!("{}", len);
    }
    let c = Complex::new(2.0, 3.0);
    let _a = escape_time(c, 5);

    assert_eq!(fibonacci().take(8).collect::<Vec<_>>(),
           vec![1, 1, 2, 3, 5, 8, 13, 21]);

}


fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex { re: 0.0, im: 0.0 };

    successors(Some(zero), |&z| { Some(z * z + c) })
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}

fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);
    
    std::iter::from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

