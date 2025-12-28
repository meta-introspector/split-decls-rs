macro_rules! read_text {
    () => {
        # [doc = " Read file and normalize newlines."] # [doc = ""] # [doc = " `rustc` seems to always normalize `\\r\\n` newlines to `\\n`:"] # [doc = ""] # [doc = " ```"] # [doc = " let s = \""] # [doc = " \";"] # [doc = " assert_eq!(s.as_bytes(), &[10]);"] # [doc = " ```"] # [doc = ""] # [doc = " so this should always be correct."] fn read_text (path : & Path) -> String { fs :: read_to_string (path) . unwrap_or_else (| _ | panic ! ("File at {path:?} should be valid")) . replace ("\r\n" , "\n") }
    };
}

read_text!()