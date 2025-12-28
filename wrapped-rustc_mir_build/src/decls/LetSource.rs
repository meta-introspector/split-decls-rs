macro_rules! LetSource {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum LetSource { None , PlainLet , IfLet , IfLetGuard , LetElse , WhileLet , Else , ElseIfLet , }
    };
}

LetSource!()