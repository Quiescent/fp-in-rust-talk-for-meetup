mod iterable;

use iterable::iterable::*;

fn next_approximation(n: f64, previous: f64) -> f64 {
    (previous + n / previous) / 2.0
}

fn naive_sqrts(n: f64, len: usize) -> Vec<f64> {
    let mut approximations = Vec::new();
    approximations.push(n / 2.0);
    for i in 1..len {
        let previous = approximations[i - 1];
        approximations.push(next_approximation(n, previous));
    }
    approximations
}

fn naive_sqrt(n: f64, times: u32) -> f64 {
    let mut current  = n / 2.0;
    for _ in 1..times {
        current  = next_approximation(n, current);
    }
    current
}

struct NewtonRhapsodyApproximation {
    current: f64,
    n: f64,
}

impl Iterable<f64> for NewtonRhapsodyApproximation {
    fn new(n: f64) -> NewtonRhapsodyApproximation {
        NewtonRhapsodyApproximation {
            current: n / 2.0,
            n: n,
        }
    }

    fn next(&self) -> NewtonRhapsodyApproximation {
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
    let a_couple =
        take(NewtonRhapsodyApproximation::new(5.0), 10);
    println!("sqrt 5: {:?}", a_couple);
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

    // let start_imperative = std::time::Instant::now();
    // println!("sqrt 5: {}", naive_sqrt(5.0, 1000000000));
    // println!("duration imperative: {:?}", std::time::Instant::now().duration_since(start_imperative));

    // let start_iterator = std::time::Instant::now();
    // println!("sqrt 5: {}", drop(NewtonRhapsodyApproximation::new(5.0), 1000000000).get());
    // println!("duration iterative: {:?}", std::time::Instant::now().duration_since(start_iterator));
}
