macro_rules! deps {
    () => {
        Error!();
        Filter!();
    };
}

macro_rules! try_get_value {
    () => {
        deps!();
        # [doc = " Helper macro to get real values out of Value while retaining"] # [doc = " proper errors in filters"] # [doc = ""] # [doc = " Takes 4 args:"] # [doc = ""] # [doc = " - the filter name,"] # [doc = " - the variable name: use \"value\" if you are using it on the variable the filter is ran on"] # [doc = " - the expected type"] # [doc = " - the actual variable"] # [doc = ""] # [doc = " ```no_compile"] # [doc = " let arr = try_get_value!(\"first\", \"value\", Vec<Value>, value);"] # [doc = " let val = try_get_value!(\"pluralize\", \"suffix\", String, val.clone());"] # [doc = " ```"] # [macro_export] macro_rules ! try_get_value { ($ filter_name : expr , $ var_name : expr , $ ty : ty , $ val : expr) => { { match $ crate :: from_value ::<$ ty > ($ val . clone ()) { Ok (s) => s , Err (_) => { if $ var_name == "value" { return Err ($ crate :: Error :: msg (format ! ("Filter `{}` was called on an incorrect value: got `{}` but expected a {}" , $ filter_name , $ val , stringify ! ($ ty)))) ; } else { return Err ($ crate :: Error :: msg (format ! ("Filter `{}` received an incorrect type for arg `{}`: got `{}` but expected a {}" , $ filter_name , $ var_name , $ val , stringify ! ($ ty)))) ; } } } } } ; }
    };
}

try_get_value!()