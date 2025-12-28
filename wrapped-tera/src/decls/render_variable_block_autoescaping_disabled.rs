macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_variable_block_autoescaping_disabled {
    () => {
        deps!();
        # [test] fn render_variable_block_autoescaping_disabled () { let mut context = Context :: new () ; context . insert ("name" , & "john") ; context . insert ("malicious" , & "<html>") ; let inputs = vec ! [("{{ name }}" , "john") , ("{{ malicious }}" , "<html>") , ("{{ malicious | safe }}" , "<html>") , ("{{ malicious | upper }}" , "<HTML>") , ("{{ malicious | upper | safe }}" , "<HTML>") , ("{{ malicious | safe | upper }}" , "<HTML>") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_template ("hello.sql" , input) . unwrap () ; assert_eq ! (tera . render ("hello.sql" , & context) . unwrap () , expected) ; } }
    };
}

render_variable_block_autoescaping_disabled!();