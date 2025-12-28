macro_rules! Element {
    () => {
        type Element = (usize , & 'static str) ;
    };
}

Element!();