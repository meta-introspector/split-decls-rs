macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! ok_many_variable_blocks {
    () => {
        deps!();
        # [test] fn ok_many_variable_blocks () { let mut context = Context :: new () ; context . insert ("username" , & "bob") ; let mut tpl = String :: new () ; for _ in 0 .. 200 { tpl . push_str ("{{ username }}") } let mut expected = String :: new () ; for _ in 0 .. 200 { expected . push_str ("bob") } assert_eq ! (render_template (& tpl , & context) . unwrap () , expected) ; }
    };
}

ok_many_variable_blocks!();