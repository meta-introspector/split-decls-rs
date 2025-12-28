macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! render_to_string {
    () => {
        deps!();
        pub (crate) fn render_to_string < C , F , E > (context : C , render : F) -> Result < String , Error > where C : FnOnce () -> String , F : FnOnce (& mut Vec < u8 >) -> Result < () , E > , Error : From < E > , { let mut buffer = Vec :: new () ; render (& mut buffer) . map_err (Error :: from) ? ; buffer_to_string (context , buffer) }
    };
}

render_to_string!();