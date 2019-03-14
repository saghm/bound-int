use bound_int::*;

bound_int_types!(Foo, 1, 3);

fn main() {
    println!("    1 + 2 = {}", bound_int_eval!(Foo_1 + Foo_2).value());
    println!("    2 + 1 = {}", bound_int_eval!(Foo_2 + Foo_1).value());
    println!("1 + 1 + 1 = {}", bound_int_eval!(Foo_1 + Foo_1 + Foo_1).value());
}