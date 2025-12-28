macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! filter_on_array_literal_works {
    () => {
        deps!();
        # [test] fn filter_on_array_literal_works () { let mut context = Context :: new () ; let i : Option < usize > = None ; context . insert ("existing" , "hello") ; context . insert ("null" , & i) ; let inputs = vec ! [(r#"{{ [1, 2, 3] | length }}"# , "3") , (r#"{% set a = [1, 2, 3] | length %}{{ a }}"# , "3") , (r#"{% for a in [1, 2, 3] | slice(start=1) %}{{ a }}{% endfor %}"# , "23") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

filter_on_array_literal_works!()