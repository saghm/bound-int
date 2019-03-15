use bound_int::*;

bound_int_types!(Foo, 1, 3);

macro_rules! print_expr {
    ($fmt:expr, $( $eval:tt )+) => {{}
        println!("{:>25} = {}", $fmt, bound_int_eval!($( $eval )+).value());
    }
}

fn main() {
    print_expr!("1 + 1", Foo_1 + Foo_1);
    print_expr!("1 + 2", Foo_1 + Foo_2);
    print_expr!("2 + 1", Foo_2 + Foo_1);
    print_expr!("1 + 1 + 1", Foo_1 + Foo_1 + Foo_1);
    println!();

    print_expr!("2 - 1", Foo_2 - Foo_1);
    print_expr!("3 - 1", Foo_3 - Foo_1);
    print_expr!("3 - 2", Foo_3 - Foo_2);

    print_expr!("3 - 1 + 1", Foo_3 - Foo_1 + Foo_1);
//    print_expr!(
//        "1 - 1 + 2 - 2 + 3 - 3 - 2 + 3",
//        Foo_1 - Foo_1 + Foo_2 - Foo_2 + Foo_3 - Foo_3 - Foo_2 + Foo_3
//    );
}