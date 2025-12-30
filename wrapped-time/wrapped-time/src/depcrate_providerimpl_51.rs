// Generated macro for impl_51 (impl)
macro_rules! Depcrate_providerimpl_51 {
() => {
// Module: crate::provider
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl serde :: Serialize for TimezonePeriods < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { use serde :: ser :: SerializeMap ; if serializer . is_human_readable () { let mut map = serializer . serialize_map (None) ? ; for (tz , idx) in self . index . iter () { if let Some (value) = self . list . get (idx) { map . serialize_entry (& tz , & [ZoneNameTimestamp :: far_in_past ()] . into_iter () . chain (value . variable . iter () . map (| (t , _ , _) | t . 0)) . map (| t | { use icu_locale_core :: subtags :: Subtag ; # [allow (clippy :: unwrap_used)] let (os , mz_info) = self . get (TimeZone (Subtag :: try_from_str (& tz) . unwrap ()) , t) . unwrap () ; (t , (os , mz_info . map (| i | { (i . id , match i . kind { MetazoneMembershipKind :: BehavesLikeGolden => { [] . as_slice () } MetazoneMembershipKind :: CustomVariants => { & ["custom variants"] } MetazoneMembershipKind :: CustomTransitions => { & ["custom transitions"] } } ,) }) ,) ,) }) . collect :: < alloc :: collections :: BTreeMap < _ , _ > > () ,) ? ; } } map . end () } else { TimeZonePeriodsSerde { list : self . list . clone () , index : self . index . clone () , offsets : self . offsets . clone () , } . serialize (serializer) } } }
};
}
