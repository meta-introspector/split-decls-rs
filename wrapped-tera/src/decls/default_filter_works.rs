macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! default_filter_works {
    () => {
        deps!();
        # [test] fn default_filter_works () { let mut context = Context :: new () ; let i : Option < usize > = None ; context . insert ("existing" , "hello") ; context . insert ("null" , & i) ; let inputs = vec ! [(r#"{{ existing | default(value="hey") }}"# , "hello") , (r#"{{ val | default(value=1) }}"# , "1") , (r#"{{ val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ obj.val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ obj.val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ not admin | default(value=false) }}"# , "true") , (r#"{{ not admin | default(value=true) }}"# , "false") , (r#"{{ null | default(value=true) }}"# , "true") , (r#"{{ null | default(value="hey") | capitalize }}"# , "Hey") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

default_filter_works!();