macro_rules! QUOTATION_MARK {
    () => {
        # [doc = " `quotation-mark = %x22            ; \"`"] pub (crate) const QUOTATION_MARK : u8 = b'"' ;
    };
}

QUOTATION_MARK!();