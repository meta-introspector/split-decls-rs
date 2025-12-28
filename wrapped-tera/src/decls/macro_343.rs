macro_rules! macro_343 {
    () => {
        lazy_static ! { static ref MATH_PARSER : PrattParser < Rule > = PrattParser :: new () . op (Op :: infix (Rule :: op_plus , Assoc :: Left) | Op :: infix (Rule :: op_minus , Assoc :: Left)) . op (Op :: infix (Rule :: op_times , Assoc :: Left) | Op :: infix (Rule :: op_slash , Assoc :: Left) | Op :: infix (Rule :: op_modulo , Assoc :: Left)) ; static ref COMPARISON_EXPR_PARSER : PrattParser < Rule > = PrattParser :: new () . op (Op :: infix (Rule :: op_lt , Assoc :: Left) | Op :: infix (Rule :: op_lte , Assoc :: Left) | Op :: infix (Rule :: op_gt , Assoc :: Left) | Op :: infix (Rule :: op_gte , Assoc :: Left) | Op :: infix (Rule :: op_eq , Assoc :: Left) | Op :: infix (Rule :: op_ineq , Assoc :: Left)) ; static ref LOGIC_EXPR_PARSER : PrattParser < Rule > = PrattParser :: new () . op (Op :: infix (Rule :: op_or , Assoc :: Left)) . op (Op :: infix (Rule :: op_and , Assoc :: Left)) ; }
    };
}

macro_343!()