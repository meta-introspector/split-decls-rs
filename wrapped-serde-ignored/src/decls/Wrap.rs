macro_rules! deps {
    () => {
        Path!();
        SeqAccess!();
    };
}

macro_rules! Wrap {
    () => {
        deps!();
        # [doc = " Wrapper that attaches context to a `Visitor`, `SeqAccess`, `EnumAccess` or"] # [doc = " `VariantAccess`."] struct Wrap < 'a , 'b , X , F : 'b > { delegate : X , callback : & 'b mut F , path : & 'a Path < 'a > , }
    };
}

Wrap!();