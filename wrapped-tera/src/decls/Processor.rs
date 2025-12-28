macro_rules! deps {
    () => {
        Template!();
        CallStack!();
        Tera!();
        If!();
        MacroCollection!();
    };
}

macro_rules! Processor {
    () => {
        deps!();
        # [doc = " Processes the ast and renders the output"] pub struct Processor < 'a > { # [doc = " The template we're trying to render"] template : & 'a Template , # [doc = " Root template of template to render - contains ast to use for rendering"] # [doc = " Can be the same as `template` if a template has no inheritance"] template_root : & 'a Template , # [doc = " The Tera object with template details"] tera : & 'a Tera , # [doc = " The call stack for processing"] call_stack : CallStack < 'a > , # [doc = " The macros organised by template and namespaces"] macros : MacroCollection < 'a > , # [doc = " If set, rendering should be escaped"] should_escape : bool , # [doc = " Used when super() is used in a block, to know where we are in our stack of"] # [doc = " definitions and for which block"] # [doc = " Vec<(block name, tpl_name, level)>"] blocks : Vec < (& 'a str , & 'a str , usize) > , }
    };
}

Processor!();