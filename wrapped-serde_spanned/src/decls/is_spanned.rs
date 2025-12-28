macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! is_spanned {
    () => {
        deps!();
        # [doc = " Check if deserializing a [`Spanned`]"] pub fn is_spanned (name : & 'static str) -> bool { crate :: spanned :: is_spanned (name) }
    };
}

is_spanned!();