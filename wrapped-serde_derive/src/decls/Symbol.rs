macro_rules! Symbol {
    () => {
        # [derive (Copy , Clone)] pub struct Symbol (& 'static str) ;
    };
}

Symbol!();