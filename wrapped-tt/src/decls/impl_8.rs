macro_rules! deps {
    () => {
        TokenTreesView!();
        TtIterSavepoint!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , S : Copy > TtIterSavepoint < 'a , S > { pub fn remaining (self) -> TokenTreesView < 'a , S > { TokenTreesView :: new (self . 0) } }
    };
}

impl_8!()