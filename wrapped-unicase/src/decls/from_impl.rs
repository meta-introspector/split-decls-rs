macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! from_impl {
    () => {
        deps!();
        macro_rules ! from_impl { ($ from : ty => $ to : ty ; $ by : ident) => (impl <'a > From <$ from > for UniCase <$ to > { fn from (s : $ from) -> Self { UniCase :: unicode (s .$ by ()) } }) ; ($ from : ty => $ to : ty) => (from_impl ! ($ from => $ to ; into) ;) }
    };
}

from_impl!()