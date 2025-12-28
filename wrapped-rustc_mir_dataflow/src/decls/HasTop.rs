macro_rules! HasTop {
    () => {
        # [doc = " A set that has a \"top\" element, which is greater than or equal to any other element."] pub trait HasTop { const TOP : Self ; }
    };
}

HasTop!();