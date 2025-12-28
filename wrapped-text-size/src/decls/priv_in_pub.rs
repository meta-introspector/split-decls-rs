macro_rules! priv_in_pub {
    () => {
        mod priv_in_pub { pub trait Sealed { } }
    };
}

priv_in_pub!();