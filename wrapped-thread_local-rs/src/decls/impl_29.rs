macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T : Send > Default for ThreadLocal < T > { fn default () -> ThreadLocal < T > { ThreadLocal :: new () } }
    };
}

impl_29!()