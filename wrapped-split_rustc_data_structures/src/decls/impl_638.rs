macro_rules! deps {
    () => {
        UnordItems!();
    };
}

macro_rules! impl_638 {
    () => {
        deps!();
        impl < T , I : Iterator < Item = T > > UnordItems < T , I > { # [inline] pub fn map < U , F : Fn (T) -> U > (self , f : F) -> UnordItems < U , impl Iterator < Item = U > > { UnordItems (self . 0 . map (f)) } # [inline] pub fn all < F : Fn (T) -> bool > (mut self , f : F) -> bool { self . 0 . all (f) } # [inline] pub fn any < F : Fn (T) -> bool > (mut self , f : F) -> bool { self . 0 . any (f) } # [inline] pub fn filter < F : Fn (& T) -> bool > (self , f : F) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . filter (f)) } # [inline] pub fn filter_map < U , F : Fn (T) -> Option < U > > (self , f : F ,) -> UnordItems < U , impl Iterator < Item = U > > { UnordItems (self . 0 . filter_map (f)) } # [inline] pub fn max (self) -> Option < T > where T : Ord , { self . 0 . max () } # [inline] pub fn min (self) -> Option < T > where T : Ord , { self . 0 . min () } # [inline] pub fn sum < S > (self) -> S where S : Sum < T > , { self . 0 . sum () } # [inline] pub fn product < S > (self) -> S where S : Product < T > , { self . 0 . product () } # [inline] pub fn count (self) -> usize { self . 0 . count () } # [inline] pub fn flat_map < U , F , O > (self , f : F) -> UnordItems < O , impl Iterator < Item = O > > where U : IntoIterator < Item = O > , F : Fn (T) -> U , { UnordItems (self . 0 . flat_map (f)) } pub fn collect < C : From < UnordItems < T , I > > > (self) -> C { self . into () } # [doc = " If the iterator has only one element, returns it, otherwise returns `None`."] # [track_caller] pub fn get_only (mut self) -> Option < T > { let item = self . 0 . next () ; if self . 0 . next () . is_some () { return None ; } item } }
    };
}

impl_638!()