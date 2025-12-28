macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_include_tag_missing {
    () => {
        deps!();
        # [test] fn render_include_tag_missing () { let mut tera = Tera :: default () ; tera . add_raw_template ("hello" , "<h1>Hello {% include \"world\" %}</h1>") . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) ; assert ! (result . is_err ()) ; let mut tera = Tera :: default () ; tera . add_raw_template ("hello" , "<h1>Hello {% include \"world\" ignore missing %}</h1>") . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello </h1>" . to_owned ()) ; }
    };
}

render_include_tag_missing!();