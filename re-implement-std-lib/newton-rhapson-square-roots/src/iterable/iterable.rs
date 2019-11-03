pub trait Iterable<T> {
    fn new(n: T) -> Self;
    fn next(&self) -> Self;
    fn get(&self) -> T;
}

pub fn take<T>(xs: impl Iterable<T>, x: usize) -> Vec<T> {
    let mut ys = Vec::new();
    (0..x).fold(xs,
                |res, _| {
                    ys.push(res.get());
                    res.next()
                });
    ys
}

pub fn drop<T>(xs: impl Iterable<T>, x: usize) -> impl Iterable<T> {
    ((0..x).fold(xs,|res, _| res.next()))
}

// Doesn't work! :/
// pub fn drop_while<T>(xs: impl Iterable<T>, p: impl Fn(dyn Iterable<T>) -> bool) -> impl Iterable<T> {
//     let mut current  = xs.next();
//     while p(current) {
//         current = current.next();
//     }
//     current
// }
