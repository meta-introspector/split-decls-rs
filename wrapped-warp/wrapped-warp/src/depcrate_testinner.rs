// Generated macro for inner (module)
macro_rules! Depcrate_testinner {
() => {
// Module: crate::test
// Provides: {"inner"}
// Dependencies: {}
mod inner { pub trait OneOrTuple { type Output ; fn one_or_tuple (self) -> Self :: Output ; } impl OneOrTuple for () { type Output = () ; fn one_or_tuple (self) -> Self :: Output { } } macro_rules ! one_or_tuple { ($ type1 : ident) => { impl <$ type1 > OneOrTuple for ($ type1 ,) { type Output = $ type1 ; fn one_or_tuple (self) -> Self :: Output { self . 0 } } } ; ($ type1 : ident , $ ($ type : ident) ,*) => { one_or_tuple ! ($ ($ type) ,*) ; impl <$ type1 , $ ($ type) ,*> OneOrTuple for ($ type1 , $ ($ type) ,*) { type Output = Self ; fn one_or_tuple (self) -> Self :: Output { self } } } } one_or_tuple ! { T1 , T2 , T3 , T4 , T5 , T6 , T7 , T8 , T9 , T10 , T11 , T12 , T13 , T14 , T15 , T16 } }
};
}
