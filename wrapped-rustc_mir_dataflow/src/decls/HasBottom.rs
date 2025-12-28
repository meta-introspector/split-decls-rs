macro_rules! HasBottom {
    () => {
        # [doc = " A set that has a \"bottom\" element, which is less than or equal to any other element."] pub trait HasBottom { const BOTTOM : Self ; fn is_bottom (& self) -> bool ; }
    };
}

HasBottom!();