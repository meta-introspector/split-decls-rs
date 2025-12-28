macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_super_in_grandchild_without_redefining_in_parent_works {
    () => {
        deps!();
        # [test] fn render_super_in_grandchild_without_redefining_in_parent_works () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block title %}Title{% endblock %}") , ("parent" , "{% extends \"grandparent\" %}") , ("child" , "{% extends \"parent\" %}{% block title %}{{ super() }} - More{% endblock %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Title - More" . to_string ()) ; }
    };
}

render_super_in_grandchild_without_redefining_in_parent_works!()