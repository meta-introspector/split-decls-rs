macro_rules! deps {
    () => {
        Error!();
        Tera!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        # [doc = " The kind of an error (non-exhaustive)"] # [derive (Debug)] # [allow (clippy :: manual_non_exhaustive)] pub enum ErrorKind { # [doc = " Generic error"] Msg (String) , # [doc = " A loop was found while looking up the inheritance chain"] CircularExtend { # [doc = " Name of the template with the loop"] tpl : String , # [doc = " All the parents templates we found so far"] inheritance_chain : Vec < String > , } , # [doc = " A template is extending a template that wasn't found in the Tera instance"] MissingParent { # [doc = " The template we are currently looking at"] current : String , # [doc = " The missing template"] parent : String , } , # [doc = " A template was missing (more generic version of MissingParent)"] TemplateNotFound (String) , # [doc = " A filter wasn't found"] FilterNotFound (String) , # [doc = " A test wasn't found"] TestNotFound (String) , # [doc = " A macro was defined in the middle of a template"] InvalidMacroDefinition (String) , # [doc = " A function wasn't found"] FunctionNotFound (String) , # [doc = " An error happened while serializing JSON"] Json (serde_json :: Error) , # [doc = " An error occured while executing a function."] CallFunction (String) , # [doc = " An error occured while executing a filter."] CallFilter (String) , # [doc = " An error occured while executing a test."] CallTest (String) , # [doc = " An IO error occured"] Io (std :: io :: ErrorKind) , # [doc = " UTF-8 conversion error"] # [doc = ""] # [doc = " This should not occur unless invalid UTF8 chars are rendered"] Utf8Conversion { # [doc = " The context that indicates where the error occurs in the rendering process"] context : String , } , # [doc = " This enum may grow additional variants, so this makes sure clients"] # [doc = " don't count on exhaustive matching. (Otherwise, adding a new variant"] # [doc = " could break existing code.)"] # [doc (hidden)] __Nonexhaustive , }
    };
}

ErrorKind!()