macro_rules! use_as_trait_name {
    () => {
        macro_rules ! use_as_trait_name { ($ ($ alias : ident => $ derive : ident) ,* $ (,) ?) => { $ (use super ::$ derive as $ alias ;) * } ; }
    };
}

use_as_trait_name!()