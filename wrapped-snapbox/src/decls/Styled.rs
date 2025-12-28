macro_rules! Styled {
    () => {
        # [derive (Debug)] pub struct Styled < D > { display : D , style : Style , }
    };
}

Styled!()