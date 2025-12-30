// Generated macro for impl_312 (impl)
macro_rules! Depcrate_de_parser_devalueimpl_312 {
() => {
// Module: crate::de::parser::devalue
// Provides: {"impl_312"}
// Dependencies: {}
impl core :: fmt :: Display for DeInteger < '_ > { fn fmt (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . radix { 2 => "0b" . fmt (formatter) ? , 8 => "0o" . fmt (formatter) ? , 10 => { } 16 => "0x" . fmt (formatter) ? , _ => { unreachable ! ("we should only ever have 2, 8, 10, and 16 radix, not {}" , self . radix) } } self . as_str () . fmt (formatter) ? ; Ok (()) } }
};
}
