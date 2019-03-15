pub use bound_int_types::bound_int_types;

#[macro_export]
macro_rules! bound_int_eval {
    ( $lhs:tt + $rhs:tt $( $op:tt $rest:tt )* ) => {{
        let sum = $lhs.plus($rhs);

        bound_int_eval!(sum $( $op $rest )*)
    }};

   ( $lhs:tt - $rhs:tt $( $op:tt $rest:tt )* ) => {{
        let sum = $lhs.minus($rhs);

        bound_int_eval!(sum $( $op $rest )*)
    }};

    ($val:expr) => {
        $val
    };
}
