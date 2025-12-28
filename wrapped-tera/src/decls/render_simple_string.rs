macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_simple_string {
    () => {
        deps!();
        # [test] fn render_simple_string () { let result = render_template ("<h1>Hello world</h1>" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "<h1>Hello world</h1>" . to_owned ()) ; }
    };
}

render_simple_string!()