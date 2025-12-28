macro_rules! deps {
    () => {
        Cx!();
        StackEntry!();
        EvaluationResult!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < X : Cx > EvaluationResult < X > { fn finalize (final_entry : StackEntry < X > , encountered_overflow : bool , result : X :: Result ,) -> EvaluationResult < X > { EvaluationResult { encountered_overflow , required_depth : final_entry . required_depth , heads : final_entry . heads , nested_goals : final_entry . nested_goals , result , } } }
    };
}

impl_161!();