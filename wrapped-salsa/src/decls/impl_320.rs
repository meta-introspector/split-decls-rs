macro_rules! deps {
    () => {
        SlotMemosMutFnRaw!();
        SlotMemosFnRaw!();
        Slot!();
        SlotMemosFn!();
        SlotMemosMutFn!();
        PageData!();
        SlotVTable!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl SlotVTable { const fn of < T : Slot > () -> & 'static Self { const { & Self { drop_impl : | data , initialized , memo_types | { let data = unsafe { Box :: from_raw (data . cast :: < PageData < T > > ()) } ; for i in 0 .. initialized { let item = data [i] . get () . cast :: < T > () ; unsafe { memo_types . attach_memos_mut ((* item) . memos_mut ()) . drop () ; ptr :: drop_in_place (item) ; } } } , layout : Layout :: new :: < T > () , type_name : std :: any :: type_name :: < T > , memos : unsafe { mem :: transmute :: < SlotMemosFn < T > , SlotMemosFnRaw > (T :: memos) } , memos_mut : unsafe { mem :: transmute :: < SlotMemosMutFn < T > , SlotMemosMutFnRaw > (T :: memos_mut) } , } } } }
    };
}

impl_320!();