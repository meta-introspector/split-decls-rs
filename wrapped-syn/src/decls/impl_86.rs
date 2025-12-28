macro_rules! deps {
    () => {
        BigInt!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl BigInt { pub (crate) fn new () -> Self { BigInt { digits : Vec :: new () } } pub (crate) fn to_string (& self) -> String { let mut repr = String :: with_capacity (self . digits . len ()) ; let mut has_nonzero = false ; for digit in self . digits . iter () . rev () { has_nonzero |= * digit != 0 ; if has_nonzero { repr . push ((* digit + b'0') as char) ; } } if repr . is_empty () { repr . push ('0') ; } repr } fn reserve_two_digits (& mut self) { let len = self . digits . len () ; let desired = len + ! self . digits . ends_with (& [0 , 0]) as usize + ! self . digits . ends_with (& [0]) as usize ; self . digits . resize (desired , 0) ; } }
    };
}

impl_86!()