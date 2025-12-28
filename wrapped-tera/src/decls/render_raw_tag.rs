macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_raw_tag {
    () => {
        deps!();
        # [test] fn render_raw_tag () { let inputs = vec ! [("{% raw %}hey{% endraw %}" , "hey") , ("{% raw %}{{hey}}{% endraw %}" , "{{hey}}") , ("{% raw %}{% if true %}{% endraw %}" , "{% if true %}") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & Context :: new ()) . unwrap () , expected) ; } }
    };
}

render_raw_tag!()