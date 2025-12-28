macro_rules! deps {
    () => {
        Link!();
        LinkedList!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl < L : Link > Default for LinkedList < L , L :: Target > { fn default () -> Self { Self :: new () } }
    };
}

impl_307!()