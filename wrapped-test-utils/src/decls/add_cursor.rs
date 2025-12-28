macro_rules! add_cursor {
    () => {
        # [doc = " Inserts `$0` marker into the `text` at `offset`."] pub fn add_cursor (text : & str , offset : TextSize) -> String { let offset : usize = offset . into () ; let mut res = String :: new () ; res . push_str (& text [.. offset]) ; res . push_str ("$0") ; res . push_str (& text [offset ..]) ; res }
    };
}

add_cursor!()