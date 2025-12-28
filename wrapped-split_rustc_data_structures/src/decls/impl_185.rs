macro_rules! deps {
    () => {
        Annotations!();
        NoAnnotations!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < N : Idx , S : Idx + Ord > Annotations < N > for NoAnnotations < S > { type SccIdx = S ; type Ann = () ; fn new (& self , _element : N) { } fn annotate_scc (& mut self , _scc : S , _annotation : ()) { } }
    };
}

impl_185!();