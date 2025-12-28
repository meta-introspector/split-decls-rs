macro_rules! deps {
    () => {
        DocCommentDesugarMode!();
    };
}

macro_rules! RawConverter {
    () => {
        deps!();
        # [doc = " A raw token (straight from lexer) converter"] struct RawConverter < 'a , Ctx > { lexed : parser :: LexedStr < 'a > , pos : usize , anchor : SpanAnchor , ctx : Ctx , mode : DocCommentDesugarMode , }
    };
}

RawConverter!();