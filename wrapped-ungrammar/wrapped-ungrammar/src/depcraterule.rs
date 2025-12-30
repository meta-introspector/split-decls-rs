// Generated macro for Rule (enum)
macro_rules! DepcrateRule {
() => {
// Module: crate
// Provides: {"Rule"}
// Dependencies: {}
# [doc = " A production rule."] # [derive (Debug , Clone , Eq , PartialEq)] pub enum Rule { # [doc = " A labeled rule, like `a:B` (`\"a\"` is the label, `B` is the rule)."] Labeled { # [doc = " The label."] label : String , # [doc = " The rule."] rule : Box < Rule > , } , # [doc = " A node, like `A`."] Node (Node) , # [doc = " A token, like `'struct'`."] Token (Token) , # [doc = " A sequence of rules, like `'while' '(' Expr ')' Stmt`."] Seq (Vec < Rule >) , # [doc = " An alternative between many rules, like `'+' | '-' | '*' | '/'`."] Alt (Vec < Rule >) , # [doc = " An optional rule, like `A?`."] Opt (Box < Rule >) , # [doc = " A repeated rule, like `A*`."] Rep (Box < Rule >) , }
};
}
