fn within<'a>(mut xs: impl Iterator<Item = f64>, eps: f64) -> f64 {
    let mut current = xs.next().unwrap();
    let mut next    = xs.next().unwrap();
    while (next - current).abs() > eps {
        current = next;
        next    = xs.next().unwrap();
    }
    next
}

fn relative<'a>(mut xs: impl Iterator<Item = f64>, eps: f64) -> f64 {
    let mut current = xs.next().unwrap();
    let mut next    = xs.next().unwrap();
    while (next / current - 1.0).abs() > eps {
        current = next;
        next    = xs.next().unwrap();
    }
    next
}

fn gradient<F>(n: f64, h: f64, f: F) -> f64 where F: Fn(f64) -> f64 {
    (f(n + h) - f(n)) / h
}

use std::*;
use std::iter::*;

fn differentiate_approximations<'a>(n: f64, h0: f64, f: &'a dyn Fn(f64) -> f64) ->
    Box<dyn Iterator<Item = f64> + 'a> {
    Box::new(
        iter::successors(Some(h0), move |current| Some(current / 2.0))
            .map(move |h| gradient(n, h, f)))
}


// fn eliminator(m: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
//     let current = xs.next();
// }

fn main() {
    let function = |x| 3.0 * (x * x) + 2.0 * x + 3.0;
    let after_a_couple: Vec<_> =
        differentiate_approximations(5.0, 1.0, &function).take(5).collect();
    println!("differentiate f at 5: {}", after_a_couple.last().unwrap());
    let within_epsilon =
        within(differentiate_approximations(5.0, 1.0, &function), 0.00000000001);
    println!("sqrt 5: {}", within_epsilon);
    let within_relative =
        relative(differentiate_approximations(5.0, 1.0, &function), 0.00000000001);
    println!("sqrt 5: {}", within_relative);
}
