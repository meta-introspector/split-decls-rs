macro_rules! deps {
    () => {
        Precedence!();
    };
}

macro_rules! FixupContext {
    () => {
        deps!();
        pub (crate) struct FixupContext { # [cfg (feature = "full")] previous_operator : Precedence , # [cfg (feature = "full")] next_operator : Precedence , # [cfg (feature = "full")] stmt : bool , # [cfg (feature = "full")] leftmost_subexpression_in_stmt : bool , # [cfg (feature = "full")] match_arm : bool , # [cfg (feature = "full")] leftmost_subexpression_in_match_arm : bool , # [cfg (feature = "full")] condition : bool , # [cfg (feature = "full")] rightmost_subexpression_in_condition : bool , # [cfg (feature = "full")] leftmost_subexpression_in_optional_operand : bool , # [cfg (feature = "full")] next_operator_can_begin_expr : bool , # [cfg (feature = "full")] next_operator_can_continue_expr : bool , next_operator_can_begin_generics : bool , }
    };
}

FixupContext!()