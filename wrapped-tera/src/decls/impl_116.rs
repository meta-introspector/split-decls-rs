macro_rules! deps {
    () => {
        Test!();
        Result!();
        Error!();
        Template!();
        ErrorKind!();
        Filter!();
        Function!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . kind { ErrorKind :: Msg (ref message) => write ! (f , "{}" , message) , ErrorKind :: CircularExtend { ref tpl , ref inheritance_chain } => write ! (f , "Circular extend detected for template '{}'. Inheritance chain: `{:?}`" , tpl , inheritance_chain) , ErrorKind :: MissingParent { ref current , ref parent } => write ! (f , "Template '{}' is inheriting from '{}', which doesn't exist or isn't loaded." , current , parent) , ErrorKind :: TemplateNotFound (ref name) => write ! (f , "Template '{}' not found" , name) , ErrorKind :: FilterNotFound (ref name) => write ! (f , "Filter '{}' not found" , name) , ErrorKind :: TestNotFound (ref name) => write ! (f , "Test '{}' not found" , name) , ErrorKind :: FunctionNotFound (ref name) => write ! (f , "Function '{}' not found" , name) , ErrorKind :: InvalidMacroDefinition (ref info) => { write ! (f , "Invalid macro definition: `{}`" , info) } ErrorKind :: Json (ref e) => write ! (f , "{}" , e) , ErrorKind :: CallFunction (ref name) => write ! (f , "Function call '{}' failed" , name) , ErrorKind :: CallFilter (ref name) => write ! (f , "Filter call '{}' failed" , name) , ErrorKind :: CallTest (ref name) => write ! (f , "Test call '{}' failed" , name) , ErrorKind :: Io (ref io_error) => { write ! (f , "Io error while writing rendered value to output: {:?}" , io_error) } ErrorKind :: Utf8Conversion { ref context } => { write ! (f , "UTF-8 conversion error occured while rendering template: {}" , context) } ErrorKind :: __Nonexhaustive => write ! (f , "Nonexhaustive") , } } }
    };
}

impl_116!();