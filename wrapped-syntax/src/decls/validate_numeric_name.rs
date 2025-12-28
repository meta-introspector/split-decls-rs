macro_rules! deps {
    () => {
        SyntaxToken!();
        SyntaxError!();
    };
}

macro_rules! validate_numeric_name {
    () => {
        deps!();
        fn validate_numeric_name (name_ref : Option < ast :: NameRef > , errors : & mut Vec < SyntaxError >) { if let Some (int_token) = int_token (name_ref) && int_token . text () . chars () . any (| c | ! c . is_ascii_digit ()) { errors . push (SyntaxError :: new ("Tuple (struct) field access is only allowed through \
                decimal integers with no underscores or suffix" , int_token . text_range () ,)) ; } fn int_token (name_ref : Option < ast :: NameRef >) -> Option < SyntaxToken > { name_ref ? . syntax () . first_child_or_token () ? . into_token () . filter (| it | it . kind () == INT_NUMBER) } }
    };
}

validate_numeric_name!()