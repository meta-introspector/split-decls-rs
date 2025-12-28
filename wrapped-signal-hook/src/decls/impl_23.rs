macro_rules! deps {
    () => {
        Exfiltrator!();
        Pending!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < E : Exfiltrator > Iterator for Pending < E > { type Item = E :: Output ; fn next (& mut self) -> Option < E :: Output > { while self . position < self . pending . slots . len () { let sig = self . position ; let slot = & self . pending . slots [sig] ; let result = self . pending . exfiltrator . load (slot , sig as c_int) ; if result . is_some () { return result ; } else { self . position += 1 ; } } None } }
    };
}

impl_23!();