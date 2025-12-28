macro_rules! deps {
    () => {
        Action!();
        Input!();
    };
}

macro_rules! INIT {
    () => {
        deps!();
        static INIT : [(Input , Action) ; 28] = [(ConsumeDelimiter , SetState (& POSTFIX)) , (Keyword ("async") , SetState (& ASYNC)) , (Keyword ("break") , SetState (& BREAK_LABEL)) , (Keyword ("const") , SetState (& CONST)) , (Keyword ("continue") , SetState (& CONTINUE)) , (Keyword ("for") , SetState (& FOR)) , (Keyword ("if") , IncDepth) , (Keyword ("let") , SetState (& PATTERN)) , (Keyword ("loop") , SetState (& BLOCK)) , (Keyword ("match") , IncDepth) , (Keyword ("move") , SetState (& CLOSURE)) , (Keyword ("return") , SetState (& RETURN)) , (Keyword ("static") , SetState (& CLOSURE)) , (Keyword ("unsafe") , SetState (& BLOCK)) , (Keyword ("while") , IncDepth) , (Keyword ("yield") , SetState (& RETURN)) , (Keyword ("_") , SetState (& POSTFIX)) , (Punct ("!") , SetState (& INIT)) , (Punct ("#") , SetState (& [(ConsumeDelimiter , SetState (& INIT))])) , (Punct ("&") , SetState (& REFERENCE)) , (Punct ("*") , SetState (& INIT)) , (Punct ("-") , SetState (& INIT)) , (Punct ("..=") , SetState (& INIT)) , (Punct ("..") , SetState (& RANGE)) , (Punct ("|") , SetState (& CLOSURE_ARGS)) , (ConsumeLifetime , SetState (& [(Punct (":") , SetState (& INIT))])) , (ConsumeLiteral , SetState (& POSTFIX)) , (ExpectPath , SetState (& PATH)) ,] ;
    };
}

INIT!();