fn within<'a>(mut xs: impl Iterator<Item = f64>, eps: f64) -> f64 {
    let mut current = xs.next().unwrap();
    let mut next    = xs.next().unwrap();
    while (next - current).abs() > eps {
        current = next;
        next    = xs.next().unwrap();
        println!("tick within");
    }
    next
}

fn relative<'a>(mut xs: impl Iterator<Item = f64>, eps: f64) -> f64 {
    let mut current = xs.next().unwrap();
    let mut next    = xs.next().unwrap();
    while (next / current - 1.0).abs() > eps {
        current = next;
        next    = xs.next().unwrap();
        println!("tick relative");
    }
    next
}

fn slope<F>(n: f64, h: f64, f: F) -> f64 where F: Fn(f64) -> f64 {
    (f(n + h) - f(n)) / h
}

use std::*;
use std::iter::*;

fn differentiate_approximations<'a>(n: f64, h0: f64, f: &'a dyn Fn(f64) -> f64) ->
    Box<dyn Iterator<Item = f64> + 'a> {
    Box::new(
        iter::successors(Some(h0), move |current| Some(current / 2.0))
            .map(move |h| slope(n, h, f)))
}

fn eliminate_current_error(n: i32, previous: f64, current: f64) -> f64 {
    let base: f64   = 2.0;
    let exp_term    = base.powi(n);
    let numerator   = previous * exp_term - current;
    let denominator = exp_term - 1.0;
    let res         = numerator / denominator;
    res
}

fn eliminate_error(n: i32, mut xs: impl 'static + Iterator<Item = f64>) -> Box<dyn Iterator<Item = f64>> {
    let mut previous = xs.next().unwrap();
    Box::new(
        xs.map(move |current| {
            previous = current;
            eliminate_current_error(n, previous, current)
        })
    )
}

fn order(mut xs: impl 'static + Iterator<Item = f64>) -> i32 {
    let c = xs.next().unwrap();
    let b = xs.next().unwrap();
    let a = xs.next().unwrap();
    ((a - c) / (b - c) - 1.0).log2().round() as i32
}

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
    let approximations  =
        differentiate_approximations(5.0, 1.0, &|x| 3.0 * (x * x) + 2.0 * x + 3.0);
    let n               =
        order(differentiate_approximations(5.0, 1.0, &|x| 3.0 * (x * x) + 2.0 * x + 3.0));
    println!("order: {}", n);
    let within_relative =
        relative(eliminate_error(n, approximations), 0.00000000001);
    println!("sqrt 5: {}", within_relative);
}
