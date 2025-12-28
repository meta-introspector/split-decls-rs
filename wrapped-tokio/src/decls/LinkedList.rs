macro_rules! LinkedList {
    () => {
        # [doc = " An intrusive linked list."] # [doc = ""] # [doc = " Currently, the list is not emptied on drop. It is the caller's"] # [doc = " responsibility to ensure the list is empty before dropping it."] pub (crate) struct LinkedList < L , T > { # [doc = " Linked list head"] head : Option < NonNull < T > > , # [doc = " Linked list tail"] tail : Option < NonNull < T > > , # [doc = " Node type marker."] _marker : PhantomData < * const L > , }
    };
}

LinkedList!();