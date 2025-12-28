macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl Default for BoundLifetimes { fn default () -> Self { BoundLifetimes { for_token : Default :: default () , lt_token : Default :: default () , lifetimes : Punctuated :: new () , gt_token : Default :: default () , } } }
    };
}

impl_318!();