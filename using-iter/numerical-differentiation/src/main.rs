mod iterable;

use iterable::iterable::*;

#[derive(Copy, Clone)]
struct NumericalDifferentiation<'a> {
    h: f64,
    n: f64,
    f: &'a dyn Fn(f64) -> f64,
}

fn gradient<F>(n: f64, h: f64, f: F) -> f64 where F: Fn(f64) -> f64 {
    (f(n + h) - f(n)) / h
}

impl<'a> Iterable<f64> for NumericalDifferentiation<'a> {
    fn next(&self) -> NumericalDifferentiation<'a> {
        NumericalDifferentiation {
            h: self.h / 2.0,
            n: self.n,
            f: self.f,
        }
    }

    fn get(&self) -> f64 {
        gradient(self.n, self.h, self.f)
    }

}

fn within(eps: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
    let mut current = xs.next();
    while (current.next().get() - current.get()).abs() > eps {
        current = current.next();
    }
    current
}

fn relative(eps: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
    let mut current = xs.next();
    while (current.next().get() / current.get() - 1.0).abs() > eps {
        current = current.next();
    }
    current
}

#[derive(Copy, Clone)]
struct NumberIterator<'a> {
    xs: NumericalDifferentiation<'a>
}

impl<'a> Iterable<f64> for NumberIterator<'a> {
    fn next(&self) -> NumberIterator<'a> {
        NumberIterator {
            xs: self.xs.next()
        }
    }

    fn get(&self) -> f64 {
        self.xs.get()
    }
}

#[derive(Copy, Clone)]
struct MappedIterator<'a> {
    xs: NumberIterator<'a>,
    f: &'a dyn Fn(MappedIterator<'a>) -> MappedIterator<'a>,
}

trait Functor<'a> {
    fn map(&self, &'a dyn Fn(MappedIterator<'a>) -> MappedIterator<'a>) -> MappedIterator<'a>;
}

impl<'a> Functor<'a> for NumberIterator<'a> {
    fn map(&self, f: &'a dyn Fn(MappedIterator<'a>) -> MappedIterator<'a>) -> MappedIterator<'a> {
        MappedIterator {
            xs: *self,
            f: f,
        }
    }
}

impl<'a> Functor<'a> for MappedIterator<'a> {
    fn map(&self, f: &'a dyn Fn(MappedIterator<'a>) -> MappedIterator<'a>) -> MappedIterator<'a> {
        MappedIterator {
            xs: self.xs,
            f: &|iterator| f((self.f)(iterator)),
        }
    }
}

// fn eliminator(m: f64, xs: &impl Iterable<f64>) -> impl Iterable<f64> {
//     let current = xs.next();

// }

fn new_numerical_differentiation<'a>(n: f64, f: &'a dyn Fn(f64) -> f64) -> NumericalDifferentiation<'a> {
    NumericalDifferentiation {
        n: n,
        h: 1.0,
        f: f,
    }
}

fn main() {
    let function = |x| 3.0 * (x * x) + 2.0 * x + 3.0;
    let after_a_couple =
        drop(new_numerical_differentiation(5.0, &function), 10).get();
    println!("sqrt 5: {}", after_a_couple);
    let within_epsilon =
        within(
            0.00000000001,
            &new_numerical_differentiation(5.0, &function)).get();
    println!("sqrt 5: {}", within_epsilon);
    let within_relative =
        relative(
            0.00000000001,
            &new_numerical_differentiation(5.0, &function)).get();
    println!("sqrt 5: {}", within_relative);
}
