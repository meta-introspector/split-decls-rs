macro_rules! deps {
    () => {
        Context!();
        Result!();
        Function!();
        Tera!();
    };
}

macro_rules! safe_function_works {
    () => {
        deps!();
        # [test] fn safe_function_works () { struct Safe ; impl crate :: Function for Safe { fn call (& self , _args : & HashMap < String , Value >) -> Result < Value > { Ok (Value :: String ("<div>Hello</div>" . to_owned ())) } fn is_safe (& self) -> bool { true } } let mut tera = Tera :: default () ; tera . register_function ("safe_function" , Safe) ; tera . add_raw_template ("test.html" , "{{ safe_function() }}") . unwrap () ; let res = tera . render ("test.html" , & Context :: new ()) ; assert_eq ! (res . unwrap () , "<div>Hello</div>") ; }
    };
}

safe_function_works!();