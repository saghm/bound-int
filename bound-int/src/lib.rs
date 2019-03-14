pub use bound_int_types::bound_int_types;

#[macro_export]
macro_rules! bound_int_eval {
    ( $lhs:tt $(+ $rhs:tt)+ ) => {{
        let lhs = $lhs;

        $(
            let rhs = bound_int_eval!($rhs);
            let lhs = lhs.get_sum(rhs);
        )+

        lhs
    }};

    ($val:expr) => {
        $val
    };
}
