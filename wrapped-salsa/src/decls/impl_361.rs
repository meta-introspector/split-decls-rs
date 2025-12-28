macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
        Identity!();
        TrackedEntry!();
        Id!();
        IdentityMap!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl IdentityMap { # [doc = " Seeds the identity map with the IDs from a previous revision."] pub (crate) fn seed (& mut self , source : & [(Identity , Id)]) { for & (key , id) in source { self . insert_entry (key , id , false) ; } } pub (crate) fn mark_all_active (& mut self , items : impl IntoIterator < Item = (Identity , Id) >) { for (key , id) in items { self . insert_entry (key , id , true) ; } } # [doc = " Insert a tracked struct identity into the map with the given ID."] pub (crate) fn insert (& mut self , key : Identity , id : Id) -> Option < Id > { self . insert_entry (key , id , true) } fn insert_entry (& mut self , key : Identity , id : Id , active : bool) -> Option < Id > { let entry = self . table . entry (key . hash , | entry | entry . identity == key , | entry | entry . identity . hash ,) ; match entry { Entry :: Vacant (entry) => { entry . insert (TrackedEntry { identity : key , id , active , }) ; None } Entry :: Occupied (mut entry) => { let tracked = entry . get_mut () ; tracked . active = active ; Some (std :: mem :: replace (& mut tracked . id , id)) } } } # [doc = " Reuses an existing identity if it already exists in the map, marking it as active."] # [doc = ""] # [doc = " Returns the existing ID, or `None` if no ID for the given identity exists."] pub (crate) fn reuse (& mut self , key : & Identity) -> Option < Id > { self . table . find_mut (key . hash , | entry | key == & entry . identity) . map (| entry | { entry . active = true ; entry . id }) } # [doc = " Returns `true` if the given tracked struct key was created in the current query execution."] pub (crate) fn is_active (& self , key : DatabaseKeyIndex) -> bool { self . table . iter () . find (| entry | { entry . id == key . key_index () && entry . identity . ingredient_index () == key . ingredient_index () }) . is_some_and (| entry | entry . active) } # [doc = " Drains the [`IdentityMap`] into a tuple of active and stale tracked structs."] # [doc = ""] # [doc = " The first entry contains the identity and IDs of any tracked structs that were"] # [doc = " created by the current execution of the query, while the second entry contains any"] # [doc = " tracked structs that were created in a previous execution but not the current one."] # [expect (clippy :: type_complexity)] pub (crate) fn drain (& mut self) -> (ThinVec < (Identity , Id) > , Vec < (Identity , Id) >) { if self . table . is_empty () { return (ThinVec :: new () , Vec :: new ()) ; } let mut stale = Vec :: new () ; let mut active = ThinVec :: with_capacity (self . table . len ()) ; for entry in self . table . drain () { if entry . active { active . push ((entry . identity , entry . id)) ; } else { stale . push ((entry . identity , entry . id)) ; } } stale . sort_unstable_by (| a , b | { (a . 0 . ingredient_index () , a . 1) . cmp (& (b . 0 . ingredient_index () , b . 1)) }) ; (active , stale) } pub (crate) fn is_empty (& self) -> bool { self . table . is_empty () } pub (crate) fn clear (& mut self) { self . table . clear () } }
    };
}

impl_361!();