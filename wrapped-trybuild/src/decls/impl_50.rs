macro_rules! deps {
    () => {
        Edition!();
        EditionOrInherit!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Default for EditionOrInherit { fn default () -> Self { EditionOrInherit :: Edition (Edition :: default ()) } }
    };
}

impl_50!();