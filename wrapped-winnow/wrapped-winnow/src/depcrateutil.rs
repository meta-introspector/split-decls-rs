// Generated macro for util (module)
macro_rules! Depcrateutil {
() => {
// Module: crate
// Provides: {"util"}
// Dependencies: {}
pub (crate) mod util { # [allow (dead_code)] pub (crate) fn from_fn < F : Fn (& mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result > (f : F ,) -> FromFn < F > { FromFn (f) } pub (crate) struct FromFn < F > (F) where F : Fn (& mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result ; impl < F > core :: fmt :: Debug for FromFn < F > where F : Fn (& mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { (self . 0) (f) } } impl < F > core :: fmt :: Display for FromFn < F > where F : Fn (& mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { (self . 0) (f) } } }
};
}
