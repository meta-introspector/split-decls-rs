macro_rules! render_literal {
    () => {
        fn render_literal (literal : & str) -> String { match literal { "\n" => "newline" . to_owned () , "`" => "'`'" . to_owned () , s if s . chars () . all (| c | c . is_ascii_control ()) => { format ! ("`{}`" , s . escape_debug ()) } s => format ! ("`{s}`") , } }
    };
}

render_literal!()