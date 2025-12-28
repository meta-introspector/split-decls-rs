macro_rules! deps {
    () => {
        TextSize!();
        TextRange!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [doc = " Identity methods."] impl TextRange { # [doc = " The start point of this range."] # [inline] pub const fn start (self) -> TextSize { self . start } # [doc = " The end point of this range."] # [inline] pub const fn end (self) -> TextSize { self . end } # [doc = " The size of this range."] # [inline] pub const fn len (self) -> TextSize { TextSize { raw : self . end () . raw - self . start () . raw , } } # [doc = " Check if this range is empty."] # [inline] pub const fn is_empty (self) -> bool { self . start () . raw == self . end () . raw } }
    };
}

impl_3!()