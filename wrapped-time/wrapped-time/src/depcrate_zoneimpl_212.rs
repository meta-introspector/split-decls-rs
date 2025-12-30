// Generated macro for impl_212 (impl)
macro_rules! Depcrate_zoneimpl_212 {
() => {
// Module: crate::zone
// Provides: {"impl_212"}
// Dependencies: {}
impl TimeZoneInfo < models :: Base > { # [doc = " Creates a time zone info with no information."] pub const fn unknown () -> Self { Self { id : TimeZone :: UNKNOWN , offset : None , zone_name_timestamp : () , variant : () , } } # [doc = " Creates a new [`TimeZoneInfo`] for the UTC time zone."] pub const fn utc () -> Self { TimeZoneInfo { id : TimeZone (subtag ! ("utc")) , offset : Some (UtcOffset :: zero ()) , zone_name_timestamp : () , variant : () , } } # [doc = " Sets the [`ZoneNameTimestamp`] field."] pub fn with_zone_name_timestamp (self , zone_name_timestamp : ZoneNameTimestamp ,) -> TimeZoneInfo < models :: AtTime > { TimeZoneInfo { offset : self . offset , id : self . id , zone_name_timestamp , variant : () , } } # [doc = " Sets the [`ZoneNameTimestamp`] to the given datetime."] # [doc = ""] # [doc = " If the offset is knonw, the datetime is interpreted as a local time,"] # [doc = " otherwise as UTC. This produces correct results for the vast majority"] # [doc = " of cases, however close to metazone changes (Eastern Time -> Central Time)"] # [doc = " it might be incorrect if the offset is not known."] # [doc = ""] # [doc = " Also see [`Self::with_zone_name_timestamp`]."] pub fn at_date_time_iso (self , date_time : DateTime < Iso >) -> TimeZoneInfo < models :: AtTime > { Self :: with_zone_name_timestamp (self , ZoneNameTimestamp :: from_zoned_date_time_iso (crate :: ZonedDateTime { date : date_time . date , time : date_time . time , zone : self . offset . unwrap_or (UtcOffset :: zero ()) , }) ,) } }
};
}
