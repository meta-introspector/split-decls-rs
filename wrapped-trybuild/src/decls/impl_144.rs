macro_rules! deps {
    () => {
        Variations!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Variations { pub fn preferred (& self) -> & str { self . variations . last () . unwrap () } pub fn any < F : FnMut (& str) -> bool > (& self , mut f : F) -> bool { self . variations . iter () . any (| stderr | f (stderr)) } pub fn concat (& mut self , other : & Self) { for (this , other) in self . variations . iter_mut () . zip (& other . variations) { if ! this . is_empty () && ! other . is_empty () { this . push ('\n') ; } this . push_str (other) ; } } }
    };
}

impl_144!();