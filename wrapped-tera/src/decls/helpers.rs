macro_rules! helpers {
    () => {
        # [doc = " Re-export some helper fns useful to write filters/fns/tests"] pub mod helpers { # [doc = " Functions helping writing tests"] pub mod tests { pub use crate :: builtins :: testers :: { extract_string , number_args_allowed , value_defined } ; } }
    };
}

helpers!();