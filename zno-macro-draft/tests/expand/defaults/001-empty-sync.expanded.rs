use zno::prelude::*;
use zno::zno;
struct test;
#[automatically_derived]
impl ::core::fmt::Debug for test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "test")
    }
}
#[zno::zno(name = "f")]
fn f(a: usize) -> usize {
    a * 2
}
fn main() {
    f();
}