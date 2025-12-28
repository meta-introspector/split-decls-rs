macro_rules! deps {
    () => {
        Value!();
        Configuration!();
        Revision!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < C > Value < C > where C : Configuration , { # [doc = " Fields of this tracked struct."] # [doc = ""] # [doc = " They can change across revisions, but they do not change within"] # [doc = " a particular revision."] # [cfg_attr (not (feature = "salsa_unstable") , doc (hidden))] pub fn fields (& self) -> & C :: Fields < '_ > { unsafe { mem :: transmute :: < & C :: Fields < 'static > , & C :: Fields < '_ > > (& self . fields) } } fn memo_table_mut (& mut self) -> & mut MemoTable { assert ! (self . updated_at . load () . is_none ()) ; & mut self . memos } fn read_lock (& self , current_revision : Revision) { loop { match self . updated_at . load () { None => { panic ! ("access to field whilst the value is being initialized") ; } Some (r) => { if r == current_revision { return ; } if self . updated_at . compare_exchange (Some (r) , Some (current_revision)) . is_ok () { break ; } } } } } # [doc = " Returns memory usage information about the tracked struct."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `MemoTable` must belong to a `Value` of the correct type."] # [cfg (feature = "salsa_unstable")] unsafe fn memory_usage (& self , memo_table_types : & MemoTableTypes) -> crate :: database :: SlotInfo { let heap_size = C :: heap_size (self . fields ()) ; let memos = unsafe { memo_table_types . attach_memos (& self . memos) } ; crate :: database :: SlotInfo { debug_name : C :: DEBUG_NAME , size_of_metadata : mem :: size_of :: < Self > () - mem :: size_of :: < C :: Fields < '_ > > () , size_of_fields : mem :: size_of :: < C :: Fields < '_ > > () , heap_size_of_fields : heap_size , memos : memos . memory_usage () , } } }
    };
}

impl_372!()