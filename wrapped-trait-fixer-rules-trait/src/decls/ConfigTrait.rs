macro_rules! deps {
    () => {
        Rule!();
    };
}

macro_rules! ConfigTrait {
    () => {
        deps!();
        pub trait ConfigTrait { fn load () -> Self ; fn get_rules (& self) -> & Vec < Rule > ; }
    };
}

ConfigTrait!()