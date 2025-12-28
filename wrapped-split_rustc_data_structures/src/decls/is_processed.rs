macro_rules! is_processed {
    () => {
        # [inline] fn is_processed (v : PreorderIndex , lastlinked : Option < PreorderIndex >) -> bool { if let Some (ll) = lastlinked { v >= ll } else { false } }
    };
}

is_processed!();