macro_rules! try_transmute {
    () => {
        pub (super) unsafe fn try_transmute < Src , Target : 'static > (x : Src) -> Result < Target , Src > { if nonstatic_typeid :: < Src > () == TypeId :: of :: < Target > () { let x = ManuallyDrop :: new (x) ; Ok (unsafe { mem :: transmute_copy :: < Src , Target > (& x) }) } else { Err (x) } }
    };
}

try_transmute!();