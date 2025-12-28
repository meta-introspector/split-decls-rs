macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! add_set_values_in_context {
    () => {
        deps!();
        # [test] fn add_set_values_in_context () { let mut context = Context :: new () ; context . insert ("my_var" , & "hey") ; context . insert ("malicious" , & "<html>") ; context . insert ("admin" , & true) ; context . insert ("num" , & 1) ; let inputs = vec ! [("{% set i = 1 %}{{ i }}" , "1") , ("{% set i = 1 + 2 %}{{ i }}" , "3") , (r#"{% set i = "hey" %}{{ i }}"# , "hey") , (r#"{% set i = "<html>" %}{{ i | safe }}"# , "<html>") , (r#"{% set i = "<html>" %}{{ i }}"# , "&lt;html&gt;") , ("{% set i = my_var %}{{ i }}" , "hey") , ("{% set i = malicious %}{{ i | safe }}" , "<html>") , ("{% set i = malicious %}{{ i }}" , "&lt;html&gt;") , ("{% set i = my_var | upper %}{{ i }}" , "HEY") , ("{% set i = range(end=3) %}{{ i }}" , "[0, 1, 2]") , ("{% set i = admin or true %}{{ i }}" , "true") , ("{% set i = admin and num > 0 %}{{ i }}" , "true") , ("{% set i = 0 / 0 %}{{ i }}" , "NaN") , ("{% set i = [1,2] %}{{ i }}" , "[1, 2]") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

add_set_values_in_context!();