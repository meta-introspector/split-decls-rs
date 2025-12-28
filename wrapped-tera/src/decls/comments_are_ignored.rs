macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! comments_are_ignored {
    () => {
        deps!();
        # [test] fn comments_are_ignored () { let inputs = vec ! [("Hello {# comment #}world" , "Hello world") , ("Hello {# comment {# nested #}world" , "Hello world") , ("My name {# was {{ name }} #}is No One." , "My name is No One.") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & Context :: new ()) . unwrap () , expected) ; } }
    };
}

comments_are_ignored!();