macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! sym {
    () => {
        deps!();
        # [doc = " This module contains all the defined non-keyword `Symbol`s."] # [doc = ""] # [doc = " Given that `sym` is imported, use them like `sym::symbol_name`."] # [doc = " For example `sym::rustfmt` or `sym::u8`."] pub mod sym { use super :: Symbol ; pub use super :: kw :: MacroRules as macro_rules ; # [doc (inline)] pub use super :: sym_generated :: * ; # [doc = " Get the symbol for an integer."] # [doc = ""] # [doc = " The first few non-negative integers each have a static symbol and therefore"] # [doc = " are fast."] pub fn integer < N : TryInto < usize > + Copy + itoa :: Integer > (n : N) -> Symbol { if let Result :: Ok (idx) = n . try_into () { if idx < 10 { return Symbol :: new (super :: SYMBOL_DIGITS_BASE + idx as u32) ; } } let mut buffer = itoa :: Buffer :: new () ; let printed = buffer . format (n) ; Symbol :: intern (printed) } }
    };
}

sym!();