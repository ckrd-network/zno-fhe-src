use zno::prelude::*;
use zno::zno;

#[derive(Debug)]
struct test;

#[zno::zno()]
fn f(a: usize) -> usize {
    a * 2
}

fn main() {
    f(2);
}
