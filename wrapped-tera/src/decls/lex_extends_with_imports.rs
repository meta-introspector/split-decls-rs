macro_rules! lex_extends_with_imports {
    () => {
        # [test] fn lex_extends_with_imports () { let sample = r#"
{% extends "base.html" %}

{% import "macros/image.html" as image %}
{% import "macros/masonry.html" as masonry %}
{% import "macros/breadcrumb.html" as breadcrumb %}
{% import "macros/ul_links.html" as ul_links %}
{% import "macros/location.html" as location %}
         "# ; assert_lex_rule ! (Rule :: template , sample) ; }
    };
}

lex_extends_with_imports!();