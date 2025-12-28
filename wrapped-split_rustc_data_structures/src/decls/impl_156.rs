macro_rules! deps {
    () => {
        MaxReached!();
        Annotations!();
        Maxes!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl Annotations < usize > for Maxes { fn new (& self , element : usize) -> MaxReached { MaxReached (self . 1 (element)) } fn annotate_scc (& mut self , scc : usize , annotation : MaxReached) { let i = self . 0 . push (annotation) ; assert ! (i == scc) ; } type Ann = MaxReached ; type SccIdx = usize ; }
    };
}

impl_156!();