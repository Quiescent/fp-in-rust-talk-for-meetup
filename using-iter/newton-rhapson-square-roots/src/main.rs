fn next_approximation(n: f64, previous: f64) -> f64 {
    (previous + n / previous) / 2.0
}

fn naive_sqrt(n: f64, eps: f64) -> f64 {
    let mut current = n / 2.0;
    let mut next    = next_approximation(n, current);
    while (next / current - 1.0).abs() > eps {
        current = next;
        next    = next_approximation(n, current);
    }
    next
}

// Doesn't work!  Please halp if you can!
// fn within(xs: impl Iterator<Item = f64>, eps: f64) -> f64 {
//     xs.zip(xs.skip(1))
//         .skip_while(|next_current| (next_current.0 - next_current.1).abs() > eps)
//         .nth(0)
//         .unwrap().0
// }

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

use std::*;
use std::iter::*;

fn square_root_approximations(n: f64) -> impl Iterator<Item = f64> {
    iter::successors(Some(n / 2.0), move |current| Some(next_approximation(n, *current)))
}

fn main() {
    println!("sqrt 5: {}", naive_sqrt(5.0, 0.00000001));
    let a_couple: Vec<_> = square_root_approximations(5.0).take(5).collect();
    println!("sqrt 5: {}", a_couple.last().unwrap());
    let within_epsilon =
        within(square_root_approximations(5.0), 0.00000000001);
    println!("sqrt 5: {}", within_epsilon);
    let within_relative =
        relative(square_root_approximations(5.0), 0.00000000001);
    println!("sqrt 5: {}", within_relative);
}
