mod iterable;

use iterable::iterable::*;

fn next_approximation(n: f64, previous: f64) -> f64 {
    (previous + n / previous) / 2.0
}

struct NumericalDifferentiation {
    current: f64,
    n: f64,
    f: dyn Fn(f64) -> f64,
}

impl Iterable<f64> for NumericalDifferentiation {
    fn next(&self) -> &NumericalDifferentiation {
        NewtonRhapsodyApproximation {
            current: next_approximation(self.n, self.current),
            n: self.n,
        }
    }

    fn get(&self) -> f64 {
        self.current
    }

}

fn within(eps: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
    let mut current  = xs.next();
    while (current.next().get() - current.get()).abs() > eps {
        current = current.next();
    }
    current
}

fn relative(eps: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
    let mut current  = xs.next();
    while (current.next().get() / current.get() - 1.0).abs() > eps {
        current = current.next();
    }
    current
}

fn main() {
    println!("sqrt 5: {:?}", naive_sqrts(5.0, 10));
    let after_a_couple =
        drop(NewtonRhapsodyApproximation::new(5.0), 10).get();
    println!("sqrt 5: {}", after_a_couple);
    let within_epsilon =
        within(
            0.00000000001,
            &NewtonRhapsodyApproximation::new(5.0)).get();
    println!("sqrt 5: {}", within_epsilon);
    let within_relative =
        relative(
            0.00000000001,
            &NewtonRhapsodyApproximation::new(5.0)).get();
    println!("sqrt 5: {}", within_relative);
}
