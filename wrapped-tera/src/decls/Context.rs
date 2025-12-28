macro_rules! Context {
    () => {
        # [doc = " The struct that holds the context of a template rendering."] # [doc = ""] # [doc = " Light wrapper around a `BTreeMap` for easier insertions of Serializable"] # [doc = " values"] # [derive (Debug , Clone , PartialEq)] pub struct Context { data : BTreeMap < String , Value > , }
    };
}

Context!();