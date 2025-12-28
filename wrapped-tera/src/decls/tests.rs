macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: escape_html ; use super :: render_to_string ; # [test] fn test_escape_html () { let tests = vec ! [(r"" , "") , (r"a&b" , "a&amp;b") , (r"<a" , "&lt;a") , (r">a" , "&gt;a") , (r#"""# , "&quot;") , (r#"'"# , "&#x27;") , (r#"大阪"# , "大阪") ,] ; for (input , expected) in tests { assert_eq ! (escape_html (input) , expected) ; } let empty = String :: new () ; assert_eq ! (escape_html (& empty) , empty) ; } # [test] fn test_render_to_string () { use std :: io :: Write ; let string = render_to_string (| | panic ! () , | w | write ! (w , "test")) . unwrap () ; assert_eq ! (string , "test" . to_owned ()) ; } }
    };
}

tests!()