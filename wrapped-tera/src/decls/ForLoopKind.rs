macro_rules! ForLoopKind {
    () => {
        # [doc = " Enumerates the two types of for loops"] # [derive (Debug , PartialEq)] pub enum ForLoopKind { # [doc = " Loop over values, eg an `Array`"] Value , # [doc = " Loop over key value pairs, eg a `HashMap` or `Object` style iteration"] KeyValue , }
    };
}

ForLoopKind!()