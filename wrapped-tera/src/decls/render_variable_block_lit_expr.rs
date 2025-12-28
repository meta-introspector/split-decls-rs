macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_variable_block_lit_expr {
    () => {
        deps!();
        # [test] fn render_variable_block_lit_expr () { let inputs = vec ! [("{{ 1 }}" , "1") , ("{{ 3.18 }}" , "3.18") , ("{{ \"hey\" }}" , "hey") , (r#"{{ "{{ hey }}" }}"# , "{{ hey }}") , ("{{ true }}" , "true") , ("{{ false }}" , "false") , ("{{ false and true or true }}" , "true") , ("{{ 1 + 1 }}" , "2") , ("{{ 1 + 1.1 }}" , "2.1") , ("{{ 3 - 1 }}" , "2") , ("{{ 3 - 1.1 }}" , "1.9") , ("{{ 2 * 5 }}" , "10") , ("{{ 10 / 5 }}" , "2") , ("{{ 2.1 * 5 }}" , "10.5") , ("{{ 2.1 * 5.05 }}" , "10.605") , ("{{ 2 / 0.5 }}" , "4") , ("{{ 2.1 / 0.5 }}" , "4.2") , ("{{ 2 + 1 * 2 }}" , "4") , ("{{ (2 + 1) * 2 }}" , "6") , ("{{ 2 * 4 % 8 }}" , "0") , ("{{ 2.8 * 2 | round }}" , "6") , ("{{ 1 / 0 }}" , "NaN") , ("{{ true and 10 }}" , "true") , ("{{ true and not 10 }}" , "false") , ("{{ not true }}" , "false") , ("{{ [1, 2, 3] }}" , "[1, 2, 3]") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & Context :: new ()) . unwrap () , expected) ; } }
    };
}

render_variable_block_lit_expr!();