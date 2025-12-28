macro_rules! deps {
    () => {
        PageIndex!();
        PageDataEntry!();
        SlotIndex!();
        Id!();
        PageView!();
        Slot!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'db , T : Slot > PageView < 'db , T > { # [inline] fn page_data (& self) -> & 'db [PageDataEntry < T >] { let len = self . 0 . allocated . load (Ordering :: Acquire) ; unsafe { slice :: from_raw_parts (self . 0 . data . cast :: < PageDataEntry < T > > () . as_ptr () , len) } } # [inline] fn data (& self) -> & 'db [T] { let len = self . 0 . allocated . load (Ordering :: Acquire) ; unsafe { slice :: from_raw_parts (self . 0 . data . cast :: < T > () . as_ptr () , len) } } # [doc = " Allocate a value in this page."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must be the unique writer to this page, i.e. `allocate` cannot be called"] # [doc = " concurrently by multiple threads. Concurrent readers however, are fine."] # [inline] pub (crate) unsafe fn allocate < V > (& self , page : PageIndex , value : V) -> Result < (Id , & 'db T) , V > where V : FnOnce (Id) -> T , { let index = self . 0 . allocated . load (Ordering :: Acquire) ; if index >= PAGE_LEN { return Err (value) ; } let id = make_id (page , SlotIndex :: new (index)) ; let data = self . 0 . data . cast :: < PageDataEntry < T > > () ; let entry = unsafe { & * data . as_ptr () . add (index) } ; unsafe { (* entry . get ()) . write (value (id)) } ; let value = unsafe { (* entry . get ()) . assume_init_ref () } ; self . 0 . allocated . store (index + 1 , Ordering :: Release) ; Ok ((id , value)) } }
    };
}

impl_332!();