// Generated macro for impl_18 (impl)
macro_rules! Depcrate_parseimpl_18 {
() => {
// Module: crate::parse
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input , T > fmt :: Debug for Handle < 'a , 'i , I , [T] > where Handle < 'a , 'i , I , T > : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?} => " , self . source_info ()) ? ; match self . all_list_heads () { ListHead :: Cons (cons) => { for (i , (elem , rest)) in cons . enumerate () { if i > 0 { write ! (f , " | ") ? ; } enum Elem < T , L > { One (T) , Spread (L) , } impl < T : fmt :: Debug , L : fmt :: Debug > fmt :: Debug for Elem < T , L > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Elem :: One (x) => fmt :: Debug :: fmt (x , f) , Elem :: Spread (xs) => { write ! (f , "..(") ? ; fmt :: Debug :: fmt (xs , f) ? ; write ! (f , ")") } } } } f . debug_list () . entries (:: std :: iter :: once (Elem :: One (elem)) . chain (rest . map (| r | { match r { Ok (x) => Elem :: One (x) , Err (Ambiguity (xs)) => Elem :: Spread (xs) , } }))) . finish () ? ; } } ListHead :: Nil => { f . debug_list () . entries (None :: < () >) . finish () ? ; } } Ok (()) } }
};
}
