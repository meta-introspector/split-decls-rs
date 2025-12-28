macro_rules! strip_start_newline {
    () => {
        fn strip_start_newline (s : & str) -> & str { s . strip_prefix ('\n') . or_else (| | s . strip_prefix ("\r\n")) . unwrap_or (s) }
    };
}

strip_start_newline!()