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

fn main() {
    println!("sqrt 5: {:?}", naive_sqrts(5.0, 10));
    let after_a_couple =
        drop(NewtonRhapsodyApproximation::new(5.0), 10).get();
    println!("sqrt 5: {}", after_a_couple);
}
