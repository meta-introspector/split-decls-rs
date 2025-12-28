macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! default {
    () => {
        deps!();
        # [test] fn default () { assert_eq ! (Bom :: default () , Bom :: Null) ; }
    };
}

default!()