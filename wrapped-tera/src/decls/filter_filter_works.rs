macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! filter_filter_works {
    () => {
        deps!();
        # [test] fn filter_filter_works () { # [derive (Debug , Serialize)] struct Author { id : u8 , } let mut context = Context :: new () ; context . insert ("authors" , & vec ! [Author { id : 1 } , Author { id : 2 } , Author { id : 3 }]) ; let inputs = vec ! [(r#"{{ authors | filter(attribute="id", value=1) | first | get(key="id") }}"# , "1")] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

filter_filter_works!();