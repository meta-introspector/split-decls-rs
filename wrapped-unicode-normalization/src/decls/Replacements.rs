macro_rules! Replacements {
    () => {
        # [doc = " External iterator for replacements for a string's characters."] # [derive (Clone)] pub struct Replacements < I > { iter : I , buffer : Option < char > , }
    };
}

Replacements!()