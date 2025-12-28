macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_tests {
    () => {
        deps!();
        # [test] fn render_tests () { let mut context = Context :: new () ; context . insert ("is_true" , & true) ; context . insert ("is_false" , & false) ; context . insert ("age" , & 18) ; context . insert ("name" , & "john") ; let mut map = HashMap :: new () ; map . insert (0 , 1) ; context . insert ("map" , & map) ; context . insert ("numbers" , & vec ! [1 , 2 , 3]) ; context . insert :: < Option < usize > , _ > ("maybe" , & None) ; let inputs = vec ! [("{% if is_true is defined %}Admin{% endif %}" , "Admin") , ("{% if hello is undefined %}Admin{% endif %}" , "Admin") , ("{% if name is string %}Admin{% endif %}" , "Admin") , ("{% if age is number %}Admin{% endif %}" , "Admin") , ("{% if age is even %}Admin{% endif %}" , "Admin") , ("{% if age is odd %}Admin{%else%}even{% endif %}" , "even") , ("{% if age is divisibleby(2) %}Admin{% endif %}" , "Admin") , ("{% if numbers is iterable %}Admin{% endif %}" , "Admin") , ("{% if map is iterable %}Admin{% endif %}" , "Admin") , ("{% if map is object %}Admin{% endif %}" , "Admin") , ("{% if name is starting_with('j') %}Admin{% endif %}" , "Admin") , ("{% if name is ending_with('n') %}Admin{% endif %}" , "Admin") , ("{% if numbers is containing(2) %}Admin{% endif %}" , "Admin") , ("{% if name is matching('^j.*') %}Admin{% endif %}" , "Admin") , ("{% if maybe is defined %}Admin{% endif %}" , "Admin") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

render_tests!();