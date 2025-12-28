macro_rules! StrStyle {
    () => {
        # [doc = " The style of a string literal, either plain quoted or a raw string like"] # [doc = " `r##\"data\"##`."] # [doc (hidden)] pub enum StrStyle { # [doc = " An ordinary string like `\"data\"`."] Cooked , # [doc = " A raw string like `r##\"data\"##`."] # [doc = ""] # [doc = " The unsigned integer is the number of `#` symbols used."] Raw (usize) , }
    };
}

StrStyle!()