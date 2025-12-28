macro_rules! deps {
    () => {
        AssertLintKind!();
        AssertLint!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , P : std :: fmt :: Debug > LintDiagnostic < 'a , () > for AssertLint < P > { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (match self . lint_kind { AssertLintKind :: ArithmeticOverflow => fluent :: mir_transform_arithmetic_overflow , AssertLintKind :: UnconditionalPanic => fluent :: mir_transform_operation_will_panic , }) ; let label = self . assert_kind . diagnostic_message () ; self . assert_kind . add_args (& mut | name , value | { diag . arg (name , value) ; }) ; diag . span_label (self . span , label) ; } }
    };
}

impl_64!();