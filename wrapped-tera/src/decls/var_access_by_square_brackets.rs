macro_rules! deps {
    () => {
        Context!();
        Test!();
        Tera!();
    };
}

macro_rules! var_access_by_square_brackets {
    () => {
        deps!();
        # [test] fn var_access_by_square_brackets () { let mut context = Context :: new () ; context . insert ("var" , & Test { a : "hi" . into () , b : "i_am_actually_b" . into () , c : vec ! ["fred" . into ()] } ,) ; context . insert ("zero" , & 0) ; context . insert ("a" , "b") ; let mut map = HashMap :: new () ; map . insert ("true" , "yes") ; map . insert ("false" , "no") ; map . insert ("with space" , "works") ; map . insert ("with/slash" , "works") ; let mut deep_map = HashMap :: new () ; deep_map . insert ("inner_map" , & map) ; context . insert ("map" , & map) ; context . insert ("deep_map" , & deep_map) ; context . insert ("bool_vec" , & vec ! ["true" , "false"]) ; let inputs = vec ! [("{{var.a}}" , "hi") , ("{{var['a']}}" , "hi") , ("{{var[\"a\"]}}" , "hi") , ("{{var['c'][0]}}" , "fred") , ("{{var['c'][zero]}}" , "fred") , ("{{var[a]}}" , "i_am_actually_b") , ("{{map['with space']}}" , "works") , ("{{map['with/slash']}}" , "works") , ("{{deep_map['inner_map'][bool_vec[zero]]}}" , "yes") ,] ; for (input , expected) in inputs { let result = Tera :: one_off (input , & context , true) . unwrap () ; println ! ("{:?} -> {:?} = {:?}" , input , expected , result) ; assert_eq ! (result , expected) ; } }
    };
}

var_access_by_square_brackets!();