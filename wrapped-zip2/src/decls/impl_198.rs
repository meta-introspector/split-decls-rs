macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl Default for DateTime { # [doc = " Constructs an 'default' datetime of 1980-01-01 00:00:00"] fn default () -> DateTime { DateTime { datepart : 0b0000000000100001 , timepart : 0 , } } }
    };
}

impl_198!();