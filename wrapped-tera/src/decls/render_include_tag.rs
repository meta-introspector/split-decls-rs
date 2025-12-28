macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_include_tag {
    () => {
        deps!();
        # [test] fn render_include_tag () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("world" , "world") , ("hello" , "<h1>Hello {% include \"world\" %}</h1>") ,]) . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello world</h1>" . to_owned ()) ; }
    };
}

render_include_tag!()