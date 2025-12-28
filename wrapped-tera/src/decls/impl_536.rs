macro_rules! deps {
    () => {
        Result!();
        MacroDefinition!();
        Error!();
        Node!();
        Block!();
        Template!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        impl Template { # [doc = " Parse the template string given"] pub fn new (tpl_name : & str , tpl_path : Option < String > , input : & str) -> Result < Template > { let ast = remove_whitespace (parse (input) ? , None) ; let mut blocks = HashMap :: new () ; fn find_blocks (ast : & [Node] , blocks : & mut HashMap < String , Block >) -> Result < () > { for node in ast { match * node { Node :: Block (_ , ref block , _) => { if blocks . contains_key (& block . name) { return Err (Error :: msg (format ! ("Block `{}` is duplicated" , block . name))) ; } blocks . insert (block . name . to_string () , block . clone ()) ; find_blocks (& block . body , blocks) ? ; } _ => continue , } ; } Ok (()) } find_blocks (& ast , & mut blocks) ? ; let mut macros = HashMap :: new () ; let mut imported_macro_files = vec ! [] ; let mut parent = None ; for node in & ast { match * node { Node :: Extends (_ , ref name) => parent = Some (name . to_string ()) , Node :: MacroDefinition (_ , ref macro_def , _) => { if macros . contains_key (& macro_def . name) { return Err (Error :: msg (format ! ("Macro `{}` is duplicated" , macro_def . name))) ; } macros . insert (macro_def . name . clone () , macro_def . clone ()) ; } Node :: ImportMacro (_ , ref tpl_name , ref namespace) => { imported_macro_files . push ((tpl_name . to_string () , namespace . to_string ())) ; } _ => continue , } } Ok (Template { name : tpl_name . to_string () , path : tpl_path , ast , parent , blocks , macros , imported_macro_files , parents : vec ! [] , blocks_definitions : HashMap :: new () , from_extend : false , }) } }
    };
}

impl_536!();