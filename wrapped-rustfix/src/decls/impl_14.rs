macro_rules! deps {
    () => {
        Data!();
        Error!();
        Span!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Data { # [doc = " Create a new data container from a slice of bytes"] pub fn new (data : & [u8]) -> Self { Data { original : data . into () , parts : vec ! [] , } } # [doc = " Commit the current changes."] pub fn commit (& mut self) { self . parts . iter_mut () . for_each (| span | span . committed = true) ; } # [doc = " Discard uncommitted changes."] pub fn restore (& mut self) { self . parts . retain (| parts | parts . committed) ; } # [doc = " Merge the original data with changes, **including** uncommitted changes."] # [doc = ""] # [doc = " See the module-level documentation for more information on why uncommitted changes are included."] pub fn to_vec (& self) -> Vec < u8 > { let mut prev_end = 0 ; let mut s = self . parts . iter () . fold (Vec :: new () , | mut acc , span | { debug_assert ! (prev_end <= span . range . start , "expected parts in sorted order") ; acc . extend_from_slice (& self . original [prev_end .. span . range . start]) ; acc . extend_from_slice (& span . data) ; prev_end = span . range . end ; acc }) ; s . extend_from_slice (& self . original [prev_end ..]) ; s } # [doc = " Record a provisional change."] # [doc = ""] # [doc = " If committed, the original data in the given `range` will be replaced by the given data."] # [doc = " If there already exist changes for data in the given range (committed or not),"] # [doc = " this method will return an error."] # [doc = " It will also return an error if the beginning of the range comes before its end,"] # [doc = " or if the range is outside that of the original data."] pub fn replace_range (& mut self , range : Range < usize > , data : & [u8]) -> Result < () , Error > { if range . start > range . end { return Err (Error :: InvalidRange (range)) ; } if range . end > self . original . len () { return Err (Error :: DataLengthExceeded (range , self . original . len ())) ; } let ins_point = self . parts . partition_point (| span | { span . range . start < range . start || (span . range . start == range . start && span . range . end < range . end) }) ; let incoming = Span :: new (range , data . as_ref ()) ; if let Some (before) = ins_point . checked_sub (1) . and_then (| i | self . parts . get (i)) { if incoming . range . start < before . range . end { return Err (Error :: AlreadyReplaced { is_identical : incoming == * before , range : incoming . range , }) ; } } if let Some (after) = self . parts . get (ins_point) { if incoming . range . end > after . range . start || incoming . range == after . range { return Err (Error :: AlreadyReplaced { is_identical : incoming == * after , range : incoming . range , }) ; } } self . parts . insert (ins_point , incoming) ; Ok (()) } }
    };
}

impl_14!();