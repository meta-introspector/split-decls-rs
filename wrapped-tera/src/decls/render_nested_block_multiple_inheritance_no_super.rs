macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_nested_block_multiple_inheritance_no_super {
    () => {
        deps!();
        # [test] fn render_nested_block_multiple_inheritance_no_super () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("index" , "{% block content%}INDEX{% endblock content %}") , ("docs" , "{% extends \"index\" %}{% block content%}DOCS{% block more %}MORE{% endblock more %}{% endblock content %}" ,) , ("page" , "{% extends \"docs\" %}{% block more %}PAGE{% endblock more %}") ,]) . unwrap () ; let result = tera . render ("page" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "DOCSPAGE" . to_string ()) ; }
    };
}

render_nested_block_multiple_inheritance_no_super!();