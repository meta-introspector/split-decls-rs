macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! Element {
    () => {
        deps!();
        # [doc = " Utility trait to allow calling `ted` functions with references or owned"] # [doc = " nodes. Do not use outside of this module."] pub trait Element { fn syntax_element (self) -> SyntaxElement ; }
    };
}

Element!();