// Generated macro for Node (enum)
macro_rules! Depcrate_parser_astNode {
() => {
// Module: crate::parser::ast
// Provides: {"Node"}
// Dependencies: {}
# [doc = " All Tera nodes that can be encountered"] # [derive (Clone , Debug , PartialEq)] pub enum Node { # [doc = " A call to `{{ super() }}` in a block"] Super , # [doc = " Some actual text"] Text (String) , # [doc = " A `{{ }}` block"] VariableBlock (WS , Expr) , # [doc = " A `{% macro hello() %}...{% endmacro %}`"] MacroDefinition (WS , MacroDefinition , WS) , # [doc = " The `{% extends \"blabla.html\" %}` node, contains the template name"] Extends (WS , String) , # [doc = " The `{% include \"blabla.html\" %}` node, contains the template name"] Include (WS , Vec < String > , bool) , # [doc = " The `{% import \"macros.html\" as macros %}`"] ImportMacro (WS , String , String) , # [doc = " The `{% set val = something %}` tag"] Set (WS , Set) , # [doc = " The text between `{% raw %}` and `{% endraw %}`"] Raw (WS , String , WS) , # [doc = " A filter section node `{{ filter name(param=\"value\") }} content {{ endfilter }}`"] FilterSection (WS , FilterSection , WS) , # [doc = " A `{% block name %}...{% endblock %}`"] Block (WS , Block , WS) , # [doc = " A `{% for i in items %}...{% endfor %}`"] Forloop (WS , Forloop , WS) , # [doc = " A if/elif/else block, WS for the if/elif/else is directly in the struct"] If (If , WS) , # [doc = " The `{% break %}` tag"] Break (WS) , # [doc = " The `{% continue %}` tag"] Continue (WS) , # [doc = " The `{# #} `comment tag and its content"] Comment (WS , String) , }
};
}
