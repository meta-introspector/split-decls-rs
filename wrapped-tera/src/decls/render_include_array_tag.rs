macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_include_array_tag {
    () => {
        deps!();
        # [test] fn render_include_array_tag () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("world" , "world") , ("hello" , "<h1>Hello {% include [\"custom/world\", \"world\"] %}</h1>") ,]) . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello world</h1>" . to_owned ()) ; tera . add_raw_template ("custom/world" , "custom world") . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello custom world</h1>" . to_owned ()) ; }
    };
}

render_include_array_tag!();