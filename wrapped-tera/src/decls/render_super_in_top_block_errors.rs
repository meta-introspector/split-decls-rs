macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_super_in_top_block_errors {
    () => {
        deps!();
        # [test] fn render_super_in_top_block_errors () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("index" , "{% block content%}{{super()}}{% endblock content %}")]) . unwrap () ; let result = tera . render ("index" , & Context :: new ()) ; assert ! (result . is_err ()) ; }
    };
}

render_super_in_top_block_errors!()