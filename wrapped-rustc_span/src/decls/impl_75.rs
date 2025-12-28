macro_rules! deps {
    () => {
        DesugaringKind!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl DesugaringKind { # [doc = " The description wording should combine well with \"desugaring of {}\"."] pub fn descr (self) -> & 'static str { match self { DesugaringKind :: Async => "`async` block or function" , DesugaringKind :: Await => "`await` expression" , DesugaringKind :: QuestionMark => "operator `?`" , DesugaringKind :: TryBlock => "`try` block" , DesugaringKind :: YeetExpr => "`do yeet` expression" , DesugaringKind :: OpaqueTy => "`impl Trait`" , DesugaringKind :: ForLoop => "`for` loop" , DesugaringKind :: WhileLoop => "`while` loop" , DesugaringKind :: BoundModifier => "trait bound modifier" , DesugaringKind :: Contract => "contract check" , DesugaringKind :: PatTyRange => "pattern type" , DesugaringKind :: FormatLiteral { source : true } => "format string literal" , DesugaringKind :: FormatLiteral { source : false } => { "expression that expanded into a format string literal" } } } # [doc = " For use with `rustc_unimplemented` to support conditions"] # [doc = " like `from_desugaring = \"QuestionMark\"`"] pub fn matches (& self , value : & str) -> bool { match self { DesugaringKind :: Async => value == "Async" , DesugaringKind :: Await => value == "Await" , DesugaringKind :: QuestionMark => value == "QuestionMark" , DesugaringKind :: TryBlock => value == "TryBlock" , DesugaringKind :: YeetExpr => value == "YeetExpr" , DesugaringKind :: OpaqueTy => value == "OpaqueTy" , DesugaringKind :: ForLoop => value == "ForLoop" , DesugaringKind :: WhileLoop => value == "WhileLoop" , DesugaringKind :: BoundModifier => value == "BoundModifier" , DesugaringKind :: Contract => value == "Contract" , DesugaringKind :: PatTyRange => value == "PatTyRange" , DesugaringKind :: FormatLiteral { .. } => value == "FormatLiteral" , } } }
    };
}

impl_75!();