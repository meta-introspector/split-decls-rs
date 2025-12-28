macro_rules! deps {
    () => {
        ErrorSample!();
        ErrorCollection!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl ErrorCollection { pub fn new () -> Self { ErrorCollection { errors : Vec :: new () } } pub fn add_error (& mut self , error : ErrorSample) { self . errors . push (error) ; } pub fn is_empty (& self) -> bool { self . errors . is_empty () } pub async fn write_to_file (& self , path : & Path) -> anyhow :: Result < () > { let json_content = serde_json :: to_string_pretty (& self . errors) . context ("Failed to serialize error collection to JSON") ? ; tokio :: fs :: write (path , json_content) . await . context (format ! ("Failed to write error collection to file: {:?}" , path)) ? ; Ok (()) } }
    };
}

impl_14!();