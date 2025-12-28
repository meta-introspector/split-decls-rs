macro_rules! TokenKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq)] enum TokenKind { Digits , Dash , Colon , Dot , T , Space , Z , Plus , Unknown , }
    };
}

TokenKind!()