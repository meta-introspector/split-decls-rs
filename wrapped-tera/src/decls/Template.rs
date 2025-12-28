macro_rules! deps {
    () => {
        Node!();
        MacroDefinition!();
        Block!();
    };
}

macro_rules! Template {
    () => {
        deps!();
        # [doc = " This is the parsed equivalent of a template file."] # [doc = " It also does some pre-processing to ensure it does as little as possible at runtime"] # [doc = " Not meant to be used directly."] # [derive (Debug , Clone)] pub struct Template { # [doc = " Name of the template, usually very similar to the path"] pub name : String , # [doc = " Original path of the file. A template doesn't necessarily have"] # [doc = " a file associated with it though so it's optional."] pub path : Option < String > , # [doc = " Parsed AST, after whitespace removal"] pub ast : Vec < Node > , # [doc = " Whether this template came from a call to `Tera::extend`, so we do"] # [doc = " not remove it when we are doing a template reload"] pub from_extend : bool , # [doc = " Macros defined in that file: name -> definition ast"] pub macros : HashMap < String , MacroDefinition > , # [doc = " (filename, namespace) for the macros imported in that file"] pub imported_macro_files : Vec < (String , String) > , # [doc = " Only used during initial parsing. Rendering will use `self.parents`"] pub parent : Option < String > , # [doc = " Only used during initial parsing. Rendering will use `self.blocks_definitions`"] pub blocks : HashMap < String , Block > , # [doc = " The full list of parent templates"] pub parents : Vec < String > , # [doc = " The definition of all the blocks for the current template and the definition of those blocks"] # [doc = " in parent templates if there are some."] # [doc = " Needed for super() to work without having to find them each time."] # [doc = " The type corresponds to the following `block_name -> [(template name, definition)]`"] # [doc = " The order of the Vec is from the first in hierarchy to the current template and the template"] # [doc = " name is needed in order to load its macros if necessary."] pub blocks_definitions : HashMap < String , Vec < (String , Block) > > , }
    };
}

Template!();