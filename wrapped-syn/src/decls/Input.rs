macro_rules! Input {
    () => {
        enum Input { Keyword (& 'static str) , Punct (& 'static str) , ConsumeAny , ConsumeBinOp , ConsumeBrace , ConsumeDelimiter , ConsumeIdent , ConsumeLifetime , ConsumeLiteral , ConsumeNestedBrace , ExpectPath , ExpectTurbofish , ExpectType , CanBeginExpr , Otherwise , Empty , }
    };
}

Input!();