// Generated macro for impl_write_smart_pointer (macro)
macro_rules! Depcrate_implsimpl_write_smart_pointer {
() => {
// Module: crate::impls
// Provides: {"impl_write_smart_pointer"}
// Dependencies: {}
# [cfg (feature = "alloc")] macro_rules ! impl_write_smart_pointer { ($ ty : path , T : $ extra_bound : path) => { impl <'a , T : ? Sized + Writeable + $ extra_bound > Writeable for $ ty { # [inline] fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { core :: borrow :: Borrow ::< T >:: borrow (self) . write_to (sink) } # [inline] fn write_to_parts < W : PartsWrite + ? Sized > (& self , sink : & mut W) -> fmt :: Result { core :: borrow :: Borrow ::< T >:: borrow (self) . write_to_parts (sink) } # [inline] fn writeable_length_hint (& self) -> LengthHint { core :: borrow :: Borrow ::< T >:: borrow (self) . writeable_length_hint () } # [inline] fn writeable_borrow (& self) -> Option <& str > { core :: borrow :: Borrow ::< T >:: borrow (self) . writeable_borrow () } # [inline] fn write_to_string (& self) -> Cow <'_ , str > { core :: borrow :: Borrow ::< T >:: borrow (self) . write_to_string () } } } ; ($ ty : path) => { impl_write_smart_pointer ! ($ ty , T : Writeable) ; } ; }
};
}
