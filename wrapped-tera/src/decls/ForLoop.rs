macro_rules! deps {
    () => {
        ForLoopState!();
        ForLoopKind!();
        ForLoopValues!();
    };
}

macro_rules! ForLoop {
    () => {
        deps!();
        # [derive (Debug)] pub struct ForLoop < 'a > { # [doc = " The key name when iterate as a Key-Value, ie in `{% for i, person in people %}` it would be `i`"] pub key_name : Option < String > , # [doc = " The value name, ie in `{% for person in people %}` it would be `person`"] pub value_name : String , # [doc = " What's the current loop index (0-indexed)"] pub current : usize , # [doc = " A list of (key, value) for the forloop. The key is `None` for `ForLoopKind::Value`"] pub values : ForLoopValues < 'a > , # [doc = " Value or KeyValue?"] pub kind : ForLoopKind , # [doc = " Has the for loop encountered break or continue?"] pub state : ForLoopState , }
    };
}

ForLoop!()