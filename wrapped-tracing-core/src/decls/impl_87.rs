macro_rules! deps {
    () => {
        Dispatch!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Default for Dispatch { # [doc = " Returns the current default dispatcher"] fn default () -> Self { get_default (| default | default . clone ()) } }
    };
}

impl_87!()