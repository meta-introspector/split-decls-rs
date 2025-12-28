macro_rules! impl_481 {
    () => {
        impl PanicStrategy { pub const fn desc_symbol (& self) -> Symbol { match * self { PanicStrategy :: Unwind => sym :: unwind , PanicStrategy :: Abort => sym :: abort , } } pub const fn all () -> [Symbol ; 2] { [Self :: Abort . desc_symbol () , Self :: Unwind . desc_symbol ()] } }
    };
}

impl_481!();