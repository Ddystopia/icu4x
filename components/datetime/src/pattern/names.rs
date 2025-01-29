// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    DateTimePattern, DateTimePatternFormatter, GetNameForCyclicYearError, GetNameForDayPeriodError,
    GetNameForEraError, GetNameForMonthError, GetNameForWeekdayError, MonthPlaceholderValue,
    PatternLoadError,
};
use crate::error::ErrorField;
use crate::fieldsets::enums::CompositeDateTimeFieldSet;
use crate::input;
use crate::provider::fields::{self, FieldLength, FieldSymbol};
use crate::provider::neo::{marker_attrs, *};
use crate::provider::pattern::PatternItem;
use crate::provider::time_zones::tz;
use crate::scaffold::*;
use crate::size_test_macro::size_test;
use crate::{external_loaders::*, DateTimeFormatterPreferences};
use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU8;
use icu_calendar::types::FormattingEra;
use icu_calendar::types::MonthCode;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::options::GroupingStrategy;
use icu_decimal::provider::{DecimalDigitsV1Marker, DecimalSymbolsV2Marker};
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;

/// Choices for loading year names.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum YearNameLength {
    /// An abbreviated calendar-dependent year or era name.
    ///
    /// Examples:
    ///
    /// - "AD"
    /// - "甲子"
    Abbreviated,
    /// A wide calendar-dependent year or era name.
    ///
    /// Examples:
    ///
    /// - "Anno Domini"
    /// - "甲子"
    Wide,
    /// A narrow calendar-dependent year or era name. Not necesarily unique.
    ///
    /// Examples:
    ///
    /// - "A"
    /// - "甲子"
    Narrow,
}

impl YearNameLength {
    pub(crate) fn to_attributes(self) -> &'static DataMarkerAttributes {
        use marker_attrs::Length;
        let length = match self {
            YearNameLength::Abbreviated => Length::Abbr,
            YearNameLength::Wide => Length::Wide,
            YearNameLength::Narrow => Length::Narrow,
        };
        marker_attrs::name_attr_for(marker_attrs::Context::Format, length)
    }

    pub(crate) fn from_field_length(field_length: FieldLength) -> Option<Self> {
        // UTS 35 says that "G..GGG" and "U..UUU" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match field_length {
            FieldLength::Three => Some(YearNameLength::Abbreviated),
            FieldLength::Four => Some(YearNameLength::Wide),
            FieldLength::Five => Some(YearNameLength::Narrow),
            _ => None,
        }
    }

    /// Returns an [`ErrorField`] sufficient for error reporting.
    pub(crate) fn to_approximate_error_field(self) -> ErrorField {
        let field_length = match self {
            YearNameLength::Abbreviated => FieldLength::Three,
            YearNameLength::Wide => FieldLength::Four,
            YearNameLength::Narrow => FieldLength::Five,
        };
        ErrorField(fields::Field {
            symbol: FieldSymbol::Era,
            length: field_length,
        })
    }
}

/// Choices for loading month names.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MonthNameLength {
    /// An abbreviated calendar-dependent month name for formatting with other fields.
    ///
    /// Example: "Sep"
    Abbreviated,
    /// A wide calendar-dependent month name for formatting with other fields.
    ///
    /// Example: "September"
    Wide,
    /// A narrow calendar-dependent month name for formatting with other fields. Not necesarily unique.
    ///
    /// Example: "S"
    Narrow,
    /// An abbreviated calendar-dependent month name for stand-alone display.
    ///
    /// Example: "Sep"
    StandaloneAbbreviated,
    /// A wide calendar-dependent month name for stand-alone display.
    ///
    /// Example: "September"
    StandaloneWide,
    /// A narrow calendar-dependent month name for stand-alone display. Not necesarily unique.
    ///
    /// Example: "S"
    StandaloneNarrow,
}

impl MonthNameLength {
    pub(crate) fn to_attributes(self) -> &'static DataMarkerAttributes {
        use marker_attrs::{Context, Length};
        let (context, length) = match self {
            MonthNameLength::Abbreviated => (Context::Format, Length::Abbr),
            MonthNameLength::Wide => (Context::Format, Length::Wide),
            MonthNameLength::Narrow => (Context::Format, Length::Narrow),
            MonthNameLength::StandaloneAbbreviated => (Context::Standalone, Length::Abbr),
            MonthNameLength::StandaloneWide => (Context::Standalone, Length::Wide),
            MonthNameLength::StandaloneNarrow => (Context::Standalone, Length::Narrow),
        };
        marker_attrs::name_attr_for(context, length)
    }

    pub(crate) fn from_field(
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Option<Self> {
        use fields::Month;
        match (field_symbol, field_length) {
            (Month::Format, FieldLength::Three) => Some(MonthNameLength::Abbreviated),
            (Month::Format, FieldLength::Four) => Some(MonthNameLength::Wide),
            (Month::Format, FieldLength::Five) => Some(MonthNameLength::Narrow),
            (Month::StandAlone, FieldLength::Three) => Some(MonthNameLength::StandaloneAbbreviated),
            (Month::StandAlone, FieldLength::Four) => Some(MonthNameLength::StandaloneWide),
            (Month::StandAlone, FieldLength::Five) => Some(MonthNameLength::StandaloneNarrow),
            _ => None,
        }
    }

    /// Returns an [`ErrorField`] sufficient for error reporting.
    pub(crate) fn to_approximate_error_field(self) -> ErrorField {
        use fields::Month;
        let (field_symbol, field_length) = match self {
            MonthNameLength::Abbreviated => (Month::Format, FieldLength::Three),
            MonthNameLength::Wide => (Month::Format, FieldLength::Four),
            MonthNameLength::Narrow => (Month::Format, FieldLength::Five),
            MonthNameLength::StandaloneAbbreviated => (Month::StandAlone, FieldLength::Three),
            MonthNameLength::StandaloneWide => (Month::StandAlone, FieldLength::Four),
            MonthNameLength::StandaloneNarrow => (Month::StandAlone, FieldLength::Five),
        };
        ErrorField(fields::Field {
            symbol: FieldSymbol::Month(field_symbol),
            length: field_length,
        })
    }
}

/// Choices for loading weekday names.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeekdayNameLength {
    /// An abbreviated weekday name for formatting with other fields.
    ///
    /// Example: "Tue"
    Abbreviated,
    /// A wide weekday name for formatting with other fields.
    ///
    /// Example: "Tuesday"
    Wide,
    /// A narrow weekday name for formatting with other fields. Not necesarily unique.
    ///
    /// Example: "T"
    Narrow,
    /// A short weekday name for formatting with other fields.
    ///
    /// Example: "Tu"
    Short,
    /// An abbreviated weekday name for stand-alone display.
    ///
    /// Example: "Tue"
    StandaloneAbbreviated,
    /// A wide weekday name for stand-alone display.
    ///
    /// Example: "Tuesday"
    StandaloneWide,
    /// A narrow weekday name for stand-alone display. Not necesarily unique.
    ///
    /// Example: "T"
    StandaloneNarrow,
    /// A short weekday name for stand-alone display.
    ///
    /// Example: "Tu"
    StandaloneShort,
}

impl WeekdayNameLength {
    pub(crate) fn to_attributes(self) -> &'static DataMarkerAttributes {
        use marker_attrs::{Context, Length};
        // UTS 35 says that "e" and "E" have the same non-numeric names
        let (context, length) = match self {
            WeekdayNameLength::Abbreviated => (Context::Format, Length::Abbr),
            WeekdayNameLength::Wide => (Context::Format, Length::Wide),
            WeekdayNameLength::Narrow => (Context::Format, Length::Narrow),
            WeekdayNameLength::Short => (Context::Format, Length::Short),
            WeekdayNameLength::StandaloneAbbreviated => (Context::Standalone, Length::Abbr),
            WeekdayNameLength::StandaloneWide => (Context::Standalone, Length::Wide),
            WeekdayNameLength::StandaloneNarrow => (Context::Standalone, Length::Narrow),
            WeekdayNameLength::StandaloneShort => (Context::Standalone, Length::Short),
        };
        marker_attrs::name_attr_for(context, length)
    }

    pub(crate) fn from_field(
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Option<Self> {
        use fields::Weekday;
        // UTS 35 says that "e" and "E" have the same non-numeric names
        let field_symbol = field_symbol.to_format_symbol();
        // UTS 35 says that "E..EEE" are all Abbreviated
        // However, this doesn't apply to "e" and "c".
        let field_length = if matches!(field_symbol, fields::Weekday::Format) {
            field_length.numeric_to_abbr()
        } else {
            field_length
        };
        match (field_symbol, field_length) {
            (Weekday::Format, FieldLength::Three) => Some(WeekdayNameLength::Abbreviated),
            (Weekday::Format, FieldLength::Four) => Some(WeekdayNameLength::Wide),
            (Weekday::Format, FieldLength::Five) => Some(WeekdayNameLength::Narrow),
            (Weekday::Format, FieldLength::Six) => Some(WeekdayNameLength::Short),
            (Weekday::StandAlone, FieldLength::Three) => {
                Some(WeekdayNameLength::StandaloneAbbreviated)
            }
            (Weekday::StandAlone, FieldLength::Four) => Some(WeekdayNameLength::StandaloneWide),
            (Weekday::StandAlone, FieldLength::Five) => Some(WeekdayNameLength::StandaloneNarrow),
            (Weekday::StandAlone, FieldLength::Six) => Some(WeekdayNameLength::StandaloneShort),
            _ => None,
        }
    }

    /// Returns an [`ErrorField`] sufficient for error reporting.
    pub(crate) fn to_approximate_error_field(self) -> ErrorField {
        use fields::Weekday;
        let (field_symbol, field_length) = match self {
            WeekdayNameLength::Abbreviated => (Weekday::Format, FieldLength::Three),
            WeekdayNameLength::Wide => (Weekday::Format, FieldLength::Four),
            WeekdayNameLength::Narrow => (Weekday::Format, FieldLength::Five),
            WeekdayNameLength::Short => (Weekday::Format, FieldLength::Six),
            WeekdayNameLength::StandaloneAbbreviated => (Weekday::StandAlone, FieldLength::Three),
            WeekdayNameLength::StandaloneWide => (Weekday::StandAlone, FieldLength::Four),
            WeekdayNameLength::StandaloneNarrow => (Weekday::StandAlone, FieldLength::Five),
            WeekdayNameLength::StandaloneShort => (Weekday::StandAlone, FieldLength::Six),
        };
        ErrorField(fields::Field {
            symbol: FieldSymbol::Weekday(field_symbol),
            length: field_length,
        })
    }
}

/// Choices for loading day period names.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DayPeriodNameLength {
    /// An abbreviated 12-hour day period name, including display names for 0h and 12h.
    ///
    /// Examples:
    ///
    /// - "AM"
    /// - "mid."
    Abbreviated,
    /// A wide 12-hour day period name, including display names for 0h and 12h.
    ///
    /// The wide form may be the same as the abbreviated form if the "real" long form
    /// (eg "ante meridiem") is not customarily used.
    ///
    /// Examples:
    ///
    /// - "AM"
    /// - "mignight"
    Wide,
    /// An abbreviated 12-hour day period name, including display names for 0h and 12h.
    ///
    /// The narrow form must be unique, unlike some other fields.
    ///
    /// Examples:
    ///
    /// - "AM"
    /// - "md"
    Narrow,
}

impl DayPeriodNameLength {
    pub(crate) fn to_attributes(self) -> &'static DataMarkerAttributes {
        use marker_attrs::Length;
        let length = match self {
            DayPeriodNameLength::Abbreviated => Length::Abbr,
            DayPeriodNameLength::Wide => Length::Wide,
            DayPeriodNameLength::Narrow => Length::Narrow,
        };
        marker_attrs::name_attr_for(marker_attrs::Context::Format, length)
    }

    pub(crate) fn from_field(
        field_symbol: fields::DayPeriod,
        field_length: FieldLength,
    ) -> Option<Self> {
        use fields::DayPeriod;
        // Names for 'a' and 'b' are stored in the same data marker
        let field_symbol = match field_symbol {
            DayPeriod::NoonMidnight => DayPeriod::AmPm,
            other => other,
        };
        // UTS 35 says that "a..aaa" and "b..bbb" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match (field_symbol, field_length) {
            (DayPeriod::AmPm, FieldLength::Three) => Some(DayPeriodNameLength::Abbreviated),
            (DayPeriod::AmPm, FieldLength::Four) => Some(DayPeriodNameLength::Wide),
            (DayPeriod::AmPm, FieldLength::Five) => Some(DayPeriodNameLength::Narrow),
            _ => None,
        }
    }

    /// Returns an [`ErrorField`] sufficient for error reporting.
    pub(crate) fn to_approximate_error_field(self) -> ErrorField {
        // Names for 'a' and 'b' are stored in the same data marker
        let field_symbol = fields::DayPeriod::AmPm;
        let field_length = match self {
            DayPeriodNameLength::Abbreviated => FieldLength::Three,
            DayPeriodNameLength::Wide => FieldLength::Four,
            DayPeriodNameLength::Narrow => FieldLength::Five,
        };
        ErrorField(fields::Field {
            symbol: FieldSymbol::DayPeriod(field_symbol),
            length: field_length,
        })
    }
}

pub struct EmptyDataProvider;

impl<M> DataProvider<M> for EmptyDataProvider
where
    M: DataMarker,
{
    fn load(&self, base_req: DataRequest) -> Result<DataResponse<M>, DataError> {
        Err(DataErrorKind::MarkerNotFound.with_req(M::INFO, base_req))
    }
}

size_test!(
    TypedDateTimeNames<icu_calendar::Gregorian>,
    typed_date_time_names_size,
    312
);

/// A low-level type that formats datetime patterns with localized names.
/// The calendar should be chosen at compile time.
#[doc = typed_date_time_names_size!()]
///
/// Type parameters:
///
/// 1. The calendar chosen at compile time for additional type safety
/// 2. A components object type containing the fields that might be formatted
///
/// By default, the components object is set to [`CompositeDateTimeFieldSet`],
/// meaning that dates and times, but not time zones, are supported. A smaller
/// components object results in smaller stack size.
///
/// To support all fields including time zones, use [`CompositeFieldSet`].
///
/// [`CompositeFieldSet`]: crate::fieldsets::enums::CompositeFieldSet
/// [`CompositeDateTimeFieldSet`]: crate::fieldsets::enums::CompositeDateTimeFieldSet
///
/// # Examples
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::Date;
/// use icu::datetime::pattern::TypedDateTimeNames;
/// use icu::datetime::pattern::DateTimePattern;
/// use icu::datetime::pattern::MonthNameLength;
/// use icu::datetime::pattern::WeekdayNameLength;
/// use icu::datetime::pattern::DayPeriodNameLength;
/// use icu::locale::locale;
/// use icu::timezone::{DateTime, Time};
/// use writeable::assert_try_writeable_eq;
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian> =
///     TypedDateTimeNames::try_new(locale!("uk").into()).unwrap();
/// names
///     .include_month_names(MonthNameLength::Abbreviated)
///     .unwrap()
///     .include_weekday_names(WeekdayNameLength::Abbreviated)
///     .unwrap()
///     .include_day_period_names(DayPeriodNameLength::Abbreviated)
///     .unwrap();
///
/// // Create a pattern from a pattern string (note: K is the hour with h11 hour cycle):
/// let pattern_str = "E MMM d y -- K:mm a";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // Test it:
/// let datetime = DateTime { date: Date::try_new_gregorian(2023, 11, 20).unwrap(), time: Time::try_new(12, 35, 3, 0).unwrap() };
/// assert_try_writeable_eq!(names.with_pattern_unchecked(&pattern).format(&datetime), "пн лист. 20 2023 -- 0:35 пп");
/// ```
///
/// If the correct data is not loaded, and error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::Date;
/// use icu::datetime::DateTimeWriteError;
/// use icu::datetime::parts;
/// use icu::datetime::pattern::TypedDateTimeNames;
/// use icu::datetime::pattern::{DateTimePattern, PatternLoadError};
/// use icu::datetime::fieldsets::enums::CompositeFieldSet;
/// use icu::locale::locale;
/// use icu::timezone::{Time, TimeZoneInfo, ZonedDateTimeParser, ZonedDateTime};
/// use icu_provider_adapters::empty::EmptyDataProvider;
/// use writeable::{Part, assert_try_writeable_parts_eq};
///
/// // Unstable API used only for error construction below
/// use icu::datetime::provider::fields::{Field, FieldLength, FieldSymbol, Weekday};
///
/// // Create an instance that can format all fields (CompositeFieldSet):
/// let mut names: TypedDateTimeNames<Gregorian, CompositeFieldSet> =
///     TypedDateTimeNames::try_new(locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y G 'at' h:mm:ssSSS a zzzz";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a",
/// // but we did not load any data!
///
/// let mut dtz = ZonedDateTimeParser::new().parse("2023-11-20T11:35:03+00:00[Europe/London]", Gregorian).unwrap();
///
/// // Missing data is filled in on a best-effort basis, and an error is signaled.
/// assert_try_writeable_parts_eq!(
///     names.with_pattern_unchecked(&pattern).format(&dtz),
///     "It is: mon M11 20 2023 CE at 11:35:03.000 AM +0000",
///     Err(DateTimeWriteError::NamesNotLoaded(Field { symbol: FieldSymbol::Weekday(Weekday::Format), length: FieldLength::One }.into())),
///     [
///         (7, 10, Part::ERROR), // mon
///         (7, 10, parts::WEEKDAY), // mon
///         (11, 14, Part::ERROR), // M11
///         (11, 14, parts::MONTH), // M11
///         (15, 17, icu::decimal::parts::INTEGER), // 20
///         (15, 17, parts::DAY), // 20
///         (18, 22, icu::decimal::parts::INTEGER), // 2023
///         (18, 22, parts::YEAR), // 2023
///         (23, 25, Part::ERROR), // CE
///         (23, 25, parts::ERA), // CE
///         (29, 31, icu::decimal::parts::INTEGER), // 11
///         (29, 31, parts::HOUR), // 11
///         (32, 34, icu::decimal::parts::INTEGER), // 35
///         (32, 34, parts::MINUTE), // 35
///         (35, 41, parts::SECOND), // 03.000
///         (35, 37, icu::decimal::parts::INTEGER), // 03
///         (37, 38, icu::decimal::parts::DECIMAL), // .
///         (38, 41, icu::decimal::parts::FRACTION), // 000
///         (42, 44, Part::ERROR), // AM
///         (42, 44, parts::DAY_PERIOD), // AM
///         (45, 50, Part::ERROR), // +0000
///         (45, 50, parts::TIME_ZONE_NAME), // +0000
///     ]
/// );
///
/// // To make the error occur sooner, one can use an EmptyDataProvider:
/// let empty = EmptyDataProvider::new();
/// assert!(matches!(
///     names.load_for_pattern(&empty, &pattern),
///     Err(PatternLoadError::Data(_, _)),
/// ));
/// ```
///
/// If the pattern contains fields inconsistent with the receiver, an error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::datetime::DateTimeWriteError;
/// use icu::datetime::parts;
/// use icu::datetime::pattern::TypedDateTimeNames;
/// use icu::datetime::pattern::DateTimePattern;
/// use icu::datetime::fieldsets::O;
/// use icu::locale::locale;
/// use icu::timezone::{DateTime, TimeZoneInfo};
/// use writeable::{Part, assert_try_writeable_parts_eq};
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian, O> =
///     TypedDateTimeNames::try_new(locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y G 'at' h:mm:ssSSS a zzzz";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a",
/// // but the `TypedDateTimeNames` is configured to format only time zones!
/// // Further, the time zone we provide doesn't contain any offset into!
/// // Missing data is filled in on a best-effort basis, and an error is signaled.
/// assert_try_writeable_parts_eq!(
///     names.with_pattern_unchecked(&pattern).format(&TimeZoneInfo::unknown()),
///     "It is: {E} {M} {d} {y} {G} at {h}:{m}:{s} {a} {z}",
///     Err(DateTimeWriteError::MissingInputField("iso_weekday")),
///     [
///         (7, 10, Part::ERROR), // {E}
///         (7, 10, parts::WEEKDAY), // {E}
///         (11, 14, Part::ERROR), // {M}
///         (11, 14, parts::MONTH), // {M}
///         (15, 18, Part::ERROR), // {d}
///         (15, 18, parts::DAY), // {d}
///         (19, 22, Part::ERROR), // {y}
///         (19, 22, parts::YEAR), // {y}
///         (23, 26, Part::ERROR), // {G}
///         (23, 26, parts::ERA), // {G}
///         (30, 33, Part::ERROR), // {h}
///         (30, 33, parts::HOUR), // {h}
///         (34, 37, Part::ERROR), // {m}
///         (34, 37, parts::MINUTE), // {m}
///         (38, 41, Part::ERROR), // {s}
///         (38, 41, parts::SECOND), // {s}
///         (42, 45, Part::ERROR), // {a}
///         (42, 45, parts::DAY_PERIOD), // {a}
///         (46, 49, Part::ERROR), // {z}
///         (46, 49, parts::TIME_ZONE_NAME), // {z}
///     ]
/// );
/// ```
#[derive(Debug)]
pub struct TypedDateTimeNames<
    C: CldrCalendar,
    FSet: DateTimeNamesMarker = CompositeDateTimeFieldSet,
> {
    prefs: DateTimeFormatterPreferences,
    inner: RawDateTimeNames<FSet>,
    _calendar: PhantomData<C>,
}

pub(crate) struct RawDateTimeNames<FSet: DateTimeNamesMarker> {
    year_names: <FSet::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container,
    month_names:
        <FSet::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container,
    weekday_names:
        <FSet::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container,
    dayperiod_names: <FSet::DayPeriodNames as NamesContainer<
        DayPeriodNamesV1Marker,
        DayPeriodNameLength,
    >>::Container,
    zone_essentials:
        <FSet::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container,
    locations_root:
        <FSet::ZoneLocationsRoot as NamesContainer<tz::LocationsRootV1Marker, ()>>::Container,
    locations: <FSet::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container,
    exemplars_root:
        <FSet::ZoneExemplarsRoot as NamesContainer<tz::ExemplarCitiesRootV1Marker, ()>>::Container,
    exemplars: <FSet::ZoneExemplars as NamesContainer<tz::ExemplarCitiesV1Marker, ()>>::Container,
    mz_generic_long:
        <FSet::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container,
    mz_generic_short:
        <FSet::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container,
    mz_specific_long:
        <FSet::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container,
    mz_specific_short:
        <FSet::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container,
    mz_periods: <FSet::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container,
    // TODO(#4340): Make the FixedDecimalFormatter optional
    fixed_decimal_formatter: Option<FixedDecimalFormatter>,
    _marker: PhantomData<FSet>,
}

// Need a custom impl because not all of the associated types impl Debug
impl<FSet: DateTimeNamesMarker> fmt::Debug for RawDateTimeNames<FSet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawDateTimeNames")
            .field("year_names", &self.year_names)
            .field("month_names", &self.month_names)
            .field("weekday_names", &self.weekday_names)
            .field("dayperiod_names", &self.dayperiod_names)
            .field("zone_essentials", &self.zone_essentials)
            .field("locations", &self.locations)
            .field("mz_generic_long", &self.mz_generic_long)
            .field("mz_generic_short", &self.mz_generic_short)
            .field("mz_specific_long", &self.mz_specific_long)
            .field("mz_specific_short", &self.mz_specific_short)
            .field("fixed_decimal_formatter", &self.fixed_decimal_formatter)
            .finish()
    }
}

impl<FSet: DateTimeNamesMarker> RawDateTimeNames<FSet> {
    pub(crate) fn with_fset<FSet2: DateTimeNamesFrom<FSet>>(self) -> RawDateTimeNames<FSet2> {
        RawDateTimeNames {
            year_names: FSet2::map_year_names(self.year_names),
            month_names: FSet2::map_month_names(self.month_names),
            weekday_names: FSet2::map_weekday_names(self.weekday_names),
            dayperiod_names: FSet2::map_day_period_names(self.dayperiod_names),
            zone_essentials: FSet2::map_zone_essentials(self.zone_essentials),
            locations_root: FSet2::map_zone_locations_root(self.locations_root),
            locations: FSet2::map_zone_locations(self.locations),
            exemplars_root: FSet2::map_zone_exemplars_root(self.exemplars_root),
            exemplars: FSet2::map_zone_exemplars(self.exemplars),
            mz_generic_long: FSet2::map_zone_generic_long(self.mz_generic_long),
            mz_generic_short: FSet2::map_zone_generic_short(self.mz_generic_short),
            mz_specific_long: FSet2::map_zone_specific_long(self.mz_specific_long),
            mz_specific_short: FSet2::map_zone_specific_short(self.mz_specific_short),
            mz_periods: FSet2::map_metazone_lookup(self.mz_periods),
            fixed_decimal_formatter: self.fixed_decimal_formatter,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimeNamesBorrowed<'l> {
    year_names: OptionalNames<YearNameLength, &'l YearNamesV1<'l>>,
    month_names: OptionalNames<MonthNameLength, &'l MonthNamesV1<'l>>,
    weekday_names: OptionalNames<WeekdayNameLength, &'l LinearNamesV1<'l>>,
    dayperiod_names: OptionalNames<DayPeriodNameLength, &'l LinearNamesV1<'l>>,
    zone_essentials: OptionalNames<(), &'l tz::EssentialsV1<'l>>,
    locations_root: OptionalNames<(), &'l tz::LocationsV1<'l>>,
    locations: OptionalNames<(), &'l tz::LocationsV1<'l>>,
    exemplars_root: OptionalNames<(), &'l tz::ExemplarCitiesV1<'l>>,
    exemplars: OptionalNames<(), &'l tz::ExemplarCitiesV1<'l>>,
    mz_generic_long: OptionalNames<(), &'l tz::MzGenericV1<'l>>,
    mz_generic_short: OptionalNames<(), &'l tz::MzGenericV1<'l>>,
    mz_specific_long: OptionalNames<(), &'l tz::MzSpecificV1<'l>>,
    mz_specific_short: OptionalNames<(), &'l tz::MzSpecificV1<'l>>,
    mz_periods: OptionalNames<(), &'l tz::MzPeriodV1<'l>>,
    pub(crate) fixed_decimal_formatter: Option<&'l FixedDecimalFormatter>,
}

impl<C: CldrCalendar, FSet: DateTimeNamesMarker> TypedDateTimeNames<C, FSet> {
    /// Constructor that takes a selected locale and creates an empty instance.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(prefs: DateTimeFormatterPreferences) -> Result<Self, DataError> {
        let mut names = Self {
            prefs,
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        };
        names.include_fixed_decimal_formatter()?;
        Ok(names)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        prefs: DateTimeFormatterPreferences,
    ) -> Result<Self, DataError>
    where
        P: DataProvider<DecimalSymbolsV2Marker> + DataProvider<DecimalDigitsV1Marker> + ?Sized,
    {
        let mut names = Self {
            prefs,
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        };
        names.load_fixed_decimal_formatter(provider)?;
        Ok(names)
    }

    /// Creates a completely empty instance, not even with number formatting.
    ///
    /// # Examples
    ///
    /// Errors occur if a number formatter is not loaded but one is required:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Date;
    /// use icu::datetime::parts;
    /// use icu::datetime::DateTimeWriteError;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::fieldsets::enums::DateFieldSet;
    /// use icu::locale::locale;
    /// use writeable::{Part, assert_try_writeable_parts_eq};
    ///
    /// // Create an instance that can format only date fields:
    /// let names: TypedDateTimeNames<Gregorian, DateFieldSet> =
    ///     TypedDateTimeNames::new_without_number_formatting(locale!("en").into());
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'It is:' y-MM-dd";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // The pattern string contains lots of numeric symbols,
    /// // but we did not load any data!
    ///
    /// let date = Date::try_new_gregorian(2024, 7, 1).unwrap();
    ///
    /// // Missing data is filled in on a best-effort basis, and an error is signaled.
    /// // (note that the padding is ignored in this fallback mode)
    /// assert_try_writeable_parts_eq!(
    ///     names.with_pattern_unchecked(&pattern).format(&date),
    ///     "It is: 2024-07-01",
    ///     Err(DateTimeWriteError::FixedDecimalFormatterNotLoaded),
    ///     [
    ///         (7, 11, Part::ERROR), // 2024
    ///         (7, 11, parts::YEAR), // 2024
    ///         (12, 14, Part::ERROR), // 07
    ///         (12, 14, parts::MONTH), // 07
    ///         (15, 17, Part::ERROR), // 01
    ///         (15, 17, parts::DAY), // 01
    ///     ]
    /// );
    /// ```
    pub fn new_without_number_formatting(prefs: DateTimeFormatterPreferences) -> Self {
        Self {
            prefs,
            inner: RawDateTimeNames::new_without_number_formatting(),
            _calendar: PhantomData,
        }
    }

    /// Loads year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_year_names<P>(
        &mut self,
        provider: &P,
        length: YearNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<C::YearNamesV1Marker> + ?Sized,
    {
        self.inner.load_year_names(
            &C::YearNamesV1Marker::bind(provider),
            self.prefs,
            length,
            length.to_approximate_error_field(),
        )?;
        Ok(self)
    }

    /// Includes year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::PatternLoadError;
    /// use icu::datetime::pattern::YearNameLength;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_year_names(YearNameLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_year_names(YearNameLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_year_names(YearNameLength::Abbreviated),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_year_names(
        &mut self,
        length: YearNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::YearNamesV1Marker>,
    {
        self.load_year_names(&crate::provider::Baked, length)
    }

    /// Loads month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_month_names<P>(
        &mut self,
        provider: &P,
        length: MonthNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<C::MonthNamesV1Marker> + ?Sized,
    {
        self.inner.load_month_names(
            &C::MonthNamesV1Marker::bind(provider),
            self.prefs,
            length,
            length.to_approximate_error_field(),
        )?;
        Ok(self)
    }

    /// Includes month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::MonthNameLength;
    /// use icu::datetime::pattern::PatternLoadError;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names
    ///     .include_month_names(MonthNameLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_month_names(MonthNameLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_month_names(MonthNameLength::StandaloneWide),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_month_names(MonthNameLength::Abbreviated),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_month_names(
        &mut self,
        length: MonthNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::MonthNamesV1Marker>,
    {
        self.load_month_names(&crate::provider::Baked, length)
    }

    /// Loads day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        length: DayPeriodNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<DayPeriodNamesV1Marker> + ?Sized,
    {
        let provider = DayPeriodNamesV1Marker::bind(provider);
        self.inner.load_day_period_names(
            &provider,
            self.prefs,
            length,
            length.to_approximate_error_field(),
        )?;
        Ok(self)
    }

    /// Includes day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::DayPeriodNameLength;
    /// use icu::datetime::pattern::PatternLoadError;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_day_period_names(DayPeriodNameLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_day_period_names(DayPeriodNameLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_day_period_names(DayPeriodNameLength::Abbreviated),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_day_period_names(
        &mut self,
        length: DayPeriodNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<DayPeriodNamesV1Marker>,
    {
        self.load_day_period_names(&crate::provider::Baked, length)
    }

    /// Loads weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_weekday_names<P>(
        &mut self,
        provider: &P,
        length: WeekdayNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<WeekdayNamesV1Marker> + ?Sized,
    {
        self.inner.load_weekday_names(
            &WeekdayNamesV1Marker::bind(provider),
            self.prefs,
            length,
            length.to_approximate_error_field(),
        )?;
        Ok(self)
    }

    /// Includes weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::PatternLoadError;
    /// use icu::datetime::pattern::WeekdayNameLength;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names
    ///     .include_weekday_names(WeekdayNameLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_weekday_names(WeekdayNameLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_weekday_names(WeekdayNameLength::StandaloneWide),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_weekday_names(WeekdayNameLength::Abbreviated),
    ///     Err(PatternLoadError::ConflictingField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_weekday_names(
        &mut self,
        length: WeekdayNameLength,
    ) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<WeekdayNamesV1Marker>,
    {
        self.load_weekday_names(&crate::provider::Baked, length)
    }

    /// Loads shared essential patterns for time zone formatting.
    pub fn load_time_zone_essentials<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::EssentialsV1Marker> + ?Sized,
    {
        self.inner
            .load_time_zone_essentials(&tz::EssentialsV1Marker::bind(provider), self.prefs)?;
        Ok(self)
    }

    /// Includes shared essential patterns for time zone formatting.
    ///
    /// This data should always be loaded when performing time zone formatting.
    /// By itself, it allows localized offset formats.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    /// let mut zone_london_summer = ZonedDateTimeParser::new()
    ///     .parse("2024-07-01T00:00:00+01:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    ///
    /// // Create a pattern with symbol `OOOO`:
    /// let pattern_str = "'Your time zone is:' OOOO";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: GMT+01:00",
    /// );
    ///
    /// // Now try `V`:
    /// let pattern_str = "'Your time zone is:' V";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: gblon",
    /// );
    ///
    /// // Now try `Z`:
    /// let pattern_str = "'Your time zone is:' Z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: +0000",
    /// );
    ///
    /// // Now try `ZZZZZ`:
    /// let pattern_str = "'Your time zone is:' ZZZZZ";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: Z",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: +01:00",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_essentials(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::EssentialsV1Marker>,
    {
        self.load_time_zone_essentials(&crate::provider::Baked)
    }

    /// Loads location names for time zone formatting.
    pub fn load_time_zone_location_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::LocationsV1Marker> + DataProvider<tz::LocationsRootV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_location_names(
            &tz::LocationsV1Marker::bind(provider),
            &tz::LocationsRootV1Marker::bind(provider),
            self.prefs,
        )?;
        Ok(self)
    }

    /// Includes location names for time zone formatting.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_location_names().unwrap();
    ///
    /// // Try `VVVV`:
    /// let pattern_str = "'Your time zone is:' VVVV";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: UK Time",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_location_names(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericShortV1Marker>,
    {
        self.load_time_zone_location_names(&crate::provider::Baked)
    }

    /// Loads generic non-location long time zone names.
    pub fn load_time_zone_generic_long_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::MzGenericLongV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_generic_long_names(
            &tz::MzGenericLongV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            self.prefs,
        )?;
        Ok(self)
    }

    /// Includes generic non-location long time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    /// let mut zone_london_summer = ZonedDateTimeParser::new()
    ///     .parse("2024-07-01T00:00:00+01:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_generic_long_names().unwrap();
    ///
    /// // Create a pattern with symbol `vvvv`:
    /// let pattern_str = "'Your time zone is:' vvvv";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: Greenwich Mean Time",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: Greenwich Mean Time", // TODO
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_generic_long_names(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericLongV1Marker>,
    {
        self.load_time_zone_generic_long_names(&crate::provider::Baked)
    }

    /// Loads generic non-location short time zone names.
    pub fn load_time_zone_generic_short_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::MzGenericShortV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_generic_short_names(
            &tz::MzGenericShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            self.prefs,
        )?;
        Ok(self)
    }

    /// Includes generic non-location short time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    /// let mut zone_london_summer = ZonedDateTimeParser::new()
    ///     .parse("2024-07-01T00:00:00+01:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_generic_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `v`:
    /// let pattern_str = "'Your time zone is:' v";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: GMT", // TODO
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_generic_short_names(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzGenericShortV1Marker>,
    {
        self.load_time_zone_generic_short_names(&crate::provider::Baked)
    }

    /// Loads specific non-location long time zone names.
    pub fn load_time_zone_specific_long_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::MzSpecificLongV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_specific_long_names(
            &tz::MzSpecificLongV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            self.prefs,
        )?;
        Ok(self)
    }

    /// Includes specific non-location long time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    /// let mut zone_london_summer = ZonedDateTimeParser::new()
    ///     .parse("2024-07-01T00:00:00+01:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_long_names().unwrap();
    ///
    /// // Create a pattern with symbol `zzzz`:
    /// let pattern_str = "'Your time zone is:' zzzz";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: Greenwich Mean Time",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: British Summer Time",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_specific_long_names(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzSpecificLongV1Marker>,
    {
        self.load_time_zone_specific_long_names(&crate::provider::Baked)
    }

    /// Loads specific non-location short time zone names.
    pub fn load_time_zone_specific_short_names<P>(
        &mut self,
        provider: &P,
    ) -> Result<&mut Self, PatternLoadError>
    where
        P: DataProvider<tz::MzSpecificShortV1Marker> + DataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        self.inner.load_time_zone_specific_short_names(
            &tz::MzSpecificShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            self.prefs,
        )?;
        Ok(self)
    }

    /// Includes specific non-location short time zone names.
    ///
    /// Important: When performing manual time zone data loading, in addition to the
    /// specific time zone format data, also call either:
    ///
    /// - [`TypedDateTimeNames::include_time_zone_essentials`]
    /// - [`TypedDateTimeNames::load_time_zone_essentials`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fieldsets::enums::ZoneFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::ZonedDateTimeParser;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut zone_london_winter = ZonedDateTimeParser::new()
    ///     .parse("2024-01-01T00:00:00+00:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    /// let mut zone_london_summer = ZonedDateTimeParser::new()
    ///     .parse("2024-07-01T00:00:00+01:00[Europe/London]", Gregorian)
    ///     .unwrap()
    ///     .zone;
    ///
    /// let mut names = TypedDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(
    ///     locale!("en-GB").into(),
    /// )
    /// .unwrap();
    ///
    /// names.include_time_zone_essentials().unwrap();
    /// names.include_time_zone_specific_short_names().unwrap();
    ///
    /// // Create a pattern with symbol `z`:
    /// let pattern_str = "'Your time zone is:' z";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_winter),
    ///     "Your time zone is: GMT",
    /// );
    /// assert_try_writeable_eq!(
    ///     names
    ///         .with_pattern_unchecked(&pattern)
    ///         .format(&zone_london_summer),
    ///     "Your time zone is: BST",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_time_zone_specific_short_names(&mut self) -> Result<&mut Self, PatternLoadError>
    where
        crate::provider::Baked: icu_provider::DataProvider<tz::MzSpecificShortV1Marker>,
    {
        self.load_time_zone_specific_short_names(&crate::provider::Baked)
    }

    /// Loads a [`FixedDecimalFormatter`] from a data provider.
    #[inline]
    pub fn load_fixed_decimal_formatter<P>(&mut self, provider: &P) -> Result<&mut Self, DataError>
    where
        P: DataProvider<DecimalSymbolsV2Marker> + DataProvider<DecimalDigitsV1Marker> + ?Sized,
    {
        self.inner
            .load_fixed_decimal_formatter(&ExternalLoaderUnstable(provider), self.prefs)?;
        Ok(self)
    }

    /// Loads a [`FixedDecimalFormatter`] with compiled data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::Time;
    /// use icu::datetime::fieldsets::enums::TimeFieldSet;
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<(), TimeFieldSet>::try_new(locale!("bn").into())
    ///         .unwrap();
    /// names.include_fixed_decimal_formatter();
    ///
    /// // Create a pattern for the time, which is all numbers
    /// let pattern_str = "'The current 24-hour time is:' HH:mm";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// let time = Time::try_new(6, 40, 33, 0).unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     names.with_pattern_unchecked(&pattern).format(&time),
    ///     "The current 24-hour time is: ০৬:৪০",
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn include_fixed_decimal_formatter(&mut self) -> Result<&mut Self, DataError> {
        self.inner
            .load_fixed_decimal_formatter(&ExternalLoaderCompiledData, self.prefs)?;
        Ok(self)
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// without checking that all necessary data is loaded.
    #[inline]
    pub fn with_pattern_unchecked<'l>(
        &'l self,
        pattern: &'l DateTimePattern,
    ) -> DateTimePatternFormatter<'l, C, FSet> {
        DateTimePatternFormatter::new(pattern.as_borrowed(), self.inner.as_borrowed())
    }

    /// Associates this [`TypedDateTimeNames`] with a datetime pattern
    /// and loads all data required for that pattern.
    ///
    /// Does not duplicate textual field symbols. See #4337
    pub fn load_for_pattern<'l, P>(
        &'l mut self,
        provider: &P,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C, FSet>, PatternLoadError>
    where
        P: DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            + DataProvider<tz::EssentialsV1Marker>
            + DataProvider<tz::LocationsV1Marker>
            + DataProvider<tz::LocationsRootV1Marker>
            + DataProvider<tz::ExemplarCitiesV1Marker>
            + DataProvider<tz::ExemplarCitiesRootV1Marker>
            + DataProvider<tz::MzGenericLongV1Marker>
            + DataProvider<tz::MzGenericShortV1Marker>
            + DataProvider<tz::MzSpecificLongV1Marker>
            + DataProvider<tz::MzSpecificShortV1Marker>
            + DataProvider<tz::MzPeriodV1Marker>
            + DataProvider<DecimalSymbolsV2Marker>
            + DataProvider<DecimalDigitsV1Marker>
            + ?Sized,
    {
        let locale = self.prefs;
        self.inner.load_for_pattern(
            &C::YearNamesV1Marker::bind(provider),
            &C::MonthNamesV1Marker::bind(provider),
            &WeekdayNamesV1Marker::bind(provider),
            &DayPeriodNamesV1Marker::bind(provider),
            // TODO: Consider making time zone name loading optional here (lots of data)
            &tz::EssentialsV1Marker::bind(provider),
            &tz::LocationsRootV1Marker::bind(provider),
            &tz::LocationsV1Marker::bind(provider),
            &tz::ExemplarCitiesRootV1Marker::bind(provider),
            &tz::ExemplarCitiesV1Marker::bind(provider),
            &tz::MzGenericLongV1Marker::bind(provider),
            &tz::MzGenericShortV1Marker::bind(provider),
            &tz::MzSpecificLongV1Marker::bind(provider),
            &tz::MzSpecificShortV1Marker::bind(provider),
            &tz::MzPeriodV1Marker::bind(provider),
            &ExternalLoaderUnstable(provider),
            locale,
            pattern.iter_items(),
        )?;
        Ok(DateTimePatternFormatter::new(
            pattern.as_borrowed(),
            self.inner.as_borrowed(),
        ))
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// and includes all data required for that pattern.
    ///
    /// Does not support duplicate textual field symbols. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::locale::locale;
    /// use icu::timezone::{DateTime, Time};
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(locale!("en").into()).unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "MMM d (EEEE) 'of year' y G 'at' h:mm a";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Load data for the pattern and format:
    /// let datetime =
    ///     DateTime { date: Date::try_new_gregorian(2023, 12, 5).unwrap(), time: Time::try_new(17, 43, 12, 0).unwrap() };
    /// assert_try_writeable_eq!(
    ///     names
    ///         .include_for_pattern(&pattern)
    ///         .unwrap()
    ///         .format(&datetime),
    ///     "Dec 5 (Tuesday) of year 2023 AD at 5:43 PM"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_for_pattern<'l>(
        &'l mut self,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C, FSet>, PatternLoadError>
    where
        crate::provider::Baked:
            DataProvider<C::YearNamesV1Marker> + DataProvider<C::MonthNamesV1Marker>,
    {
        let locale = self.prefs;
        self.inner.load_for_pattern(
            &C::YearNamesV1Marker::bind(&crate::provider::Baked),
            &C::MonthNamesV1Marker::bind(&crate::provider::Baked),
            &WeekdayNamesV1Marker::bind(&crate::provider::Baked),
            &DayPeriodNamesV1Marker::bind(&crate::provider::Baked),
            &tz::EssentialsV1Marker::bind(&crate::provider::Baked),
            &tz::LocationsV1Marker::bind(&crate::provider::Baked),
            &tz::LocationsRootV1Marker::bind(&crate::provider::Baked),
            &tz::ExemplarCitiesV1Marker::bind(&crate::provider::Baked),
            &tz::ExemplarCitiesRootV1Marker::bind(&crate::provider::Baked),
            &tz::MzGenericLongV1Marker::bind(&crate::provider::Baked),
            &tz::MzGenericShortV1Marker::bind(&crate::provider::Baked),
            &tz::MzSpecificLongV1Marker::bind(&crate::provider::Baked),
            &tz::MzSpecificShortV1Marker::bind(&crate::provider::Baked),
            &tz::MzPeriodV1Marker::bind(&crate::provider::Baked),
            &ExternalLoaderCompiledData,
            locale,
            pattern.iter_items(),
        )?;
        Ok(DateTimePatternFormatter::new(
            pattern.as_borrowed(),
            self.inner.as_borrowed(),
        ))
    }

    /// Maps a [`TypedDateTimeNames`] of a specific `FSet` to a more general `FSet`.
    ///
    /// For example, this can transform a formatter for [`DateFieldSet`] to one for
    /// [`CompositeDateTimeFieldSet`].
    ///
    /// [`DateFieldSet`]: crate::fieldsets::enums::DateFieldSet
    /// [`CompositeDateTimeFieldSet`]: crate::fieldsets::enums::CompositeDateTimeFieldSet
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::calendar::Date;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::datetime::pattern::MonthNameLength;
    /// use icu::datetime::fieldsets::enums::{DateFieldSet, CompositeDateTimeFieldSet};
    /// use icu::datetime::pattern::DateTimePattern;
    /// use icu::locale::locale;
    /// use icu::timezone::{DateTime, Time};
    /// use writeable::assert_try_writeable_eq;
    ///
    /// // Create an instance that can format abbreviated month names:
    /// let mut names: TypedDateTimeNames<Gregorian, DateFieldSet> =
    ///     TypedDateTimeNames::try_new(locale!("uk").into()).unwrap();
    /// names
    ///     .include_month_names(MonthNameLength::Abbreviated)
    ///     .unwrap();
    ///
    /// // Test it with a pattern:
    /// let pattern_str = "MMM d y";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    /// let datetime = DateTime { date: Date::try_new_gregorian(2023, 11, 20).unwrap(), time: Time::midnight() };
    /// assert_try_writeable_eq!(names.with_pattern_unchecked(&pattern).format(&datetime), "лист. 20 2023");
    ///
    /// // Convert the field set to `CompositeDateTimeFieldSet`:
    /// let composite_names = names.with_fset::<CompositeDateTimeFieldSet>();
    ///
    /// // It should still work:
    /// assert_try_writeable_eq!(composite_names.with_pattern_unchecked(&pattern).format(&datetime), "лист. 20 2023");
    /// ```
    ///
    /// Converting into a narrower type is not supported:
    ///
    /// ```compile_fail,E0277
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::pattern::TypedDateTimeNames;
    /// use icu::datetime::fieldsets::enums::{DateFieldSet, CompositeDateTimeFieldSet};
    ///
    /// let composite_names: TypedDateTimeNames<Gregorian, CompositeDateTimeFieldSet> = todo!();
    ///
    /// // error[E0277]: the trait bound `(): From<DataPayloadWithVariables<DayPeriodNamesV1Marker, FieldLength>>` is not satisfied
    /// let narrow_names = composite_names.with_fset::<DateFieldSet>();
    /// ```
    pub fn with_fset<FSet2: DateTimeNamesFrom<FSet>>(self) -> TypedDateTimeNames<C, FSet2> {
        TypedDateTimeNames {
            prefs: self.prefs,
            inner: self.inner.with_fset(),
            _calendar: PhantomData,
        }
    }
}

impl<FSet: DateTimeNamesMarker> RawDateTimeNames<FSet> {
    pub(crate) fn new_without_number_formatting() -> Self {
        Self {
            year_names: <FSet::YearNames as NamesContainer<
                YearNamesV1Marker,
                YearNameLength,
            >>::Container::new_empty(),
            month_names: <FSet::MonthNames as NamesContainer<
                MonthNamesV1Marker,
                MonthNameLength,
            >>::Container::new_empty(),
            weekday_names: <FSet::WeekdayNames as NamesContainer<
                WeekdayNamesV1Marker,
                WeekdayNameLength,
            >>::Container::new_empty(),
            dayperiod_names: <FSet::DayPeriodNames as NamesContainer<
                DayPeriodNamesV1Marker,
                DayPeriodNameLength,
            >>::Container::new_empty(),
            zone_essentials: <FSet::ZoneEssentials as NamesContainer<
                tz::EssentialsV1Marker,
                (),
            >>::Container::new_empty(),
            locations_root: <FSet::ZoneLocationsRoot as NamesContainer<
                tz::LocationsRootV1Marker,
                (),
            >>::Container::new_empty(),
            locations: <FSet::ZoneLocations as NamesContainer<
                tz::LocationsV1Marker,
                (),
            >>::Container::new_empty(),
            exemplars: <FSet::ZoneExemplars as NamesContainer<
                tz::ExemplarCitiesV1Marker,
                (),
            >>::Container::new_empty(),
            exemplars_root: <FSet::ZoneExemplarsRoot as NamesContainer<
                tz::ExemplarCitiesRootV1Marker,
                (),
            >>::Container::new_empty(),
            mz_generic_long: <FSet::ZoneGenericLong as NamesContainer<
                tz::MzGenericLongV1Marker,
                (),
            >>::Container::new_empty(),
            mz_generic_short: <FSet::ZoneGenericShort as NamesContainer<
                tz::MzGenericShortV1Marker,
                (),
            >>::Container::new_empty(),
            mz_specific_long: <FSet::ZoneSpecificLong as NamesContainer<
                tz::MzSpecificLongV1Marker,
                (),
            >>::Container::new_empty(),
            mz_specific_short: <FSet::ZoneSpecificShort as NamesContainer<
                tz::MzSpecificShortV1Marker,
                (),
            >>::Container::new_empty(),
            mz_periods: <FSet::MetazoneLookup as NamesContainer<
                tz::MzPeriodV1Marker,
                (),
            >>::Container::new_empty(),
            fixed_decimal_formatter: None,
            _marker: PhantomData,
        }
    }

    pub(crate) fn as_borrowed(&self) -> RawDateTimeNamesBorrowed {
        RawDateTimeNamesBorrowed {
            year_names: self.year_names.get().inner,
            month_names: self.month_names.get().inner,
            weekday_names: self.weekday_names.get().inner,
            dayperiod_names: self.dayperiod_names.get().inner,
            zone_essentials: self.zone_essentials.get().inner,
            locations_root: self.locations_root.get().inner,
            locations: self.locations.get().inner,
            exemplars_root: self.exemplars_root.get().inner,
            exemplars: self.exemplars.get().inner,
            mz_generic_long: self.mz_generic_long.get().inner,
            mz_generic_short: self.mz_generic_short.get().inner,
            mz_specific_long: self.mz_specific_long.get().inner,
            mz_specific_short: self.mz_specific_short.get().inner,
            mz_periods: self.mz_periods.get().inner,
            fixed_decimal_formatter: self.fixed_decimal_formatter.as_ref(),
        }
    }

    pub(crate) fn load_year_names<P>(
        &mut self,
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        length: YearNameLength,
        error_field: ErrorField,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<YearNamesV1Marker> + ?Sized,
    {
        let attributes = length.to_attributes();
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
            ..Default::default()
        };
        self.year_names
            .load_put(provider, req, length)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_month_names<P>(
        &mut self,
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        length: MonthNameLength,
        error_field: ErrorField,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<MonthNamesV1Marker> + ?Sized,
    {
        let attributes = length.to_attributes();
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
            ..Default::default()
        };
        self.month_names
            .load_put(provider, req, length)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        length: DayPeriodNameLength,
        error_field: ErrorField,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<DayPeriodNamesV1Marker> + ?Sized,
    {
        let attributes = length.to_attributes();
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
            ..Default::default()
        };
        self.dayperiod_names
            .load_put(provider, req, length)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_weekday_names<P>(
        &mut self,
        provider: &P,
        prefs: DateTimeFormatterPreferences,
        length: WeekdayNameLength,
        error_field: ErrorField,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<WeekdayNamesV1Marker> + ?Sized,
    {
        let attributes = length.to_attributes();
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
            ..Default::default()
        };
        self.weekday_names
            .load_put(provider, req, length)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_time_zone_essentials<P>(
        &mut self,
        provider: &P,
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<tz::EssentialsV1Marker> + ?Sized,
    {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::LocalizedOffset),
            length: FieldLength::Four,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.zone_essentials
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_time_zone_location_names<P, P2>(
        &mut self,
        provider: &P,
        root_provider: &P2,
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<tz::LocationsV1Marker> + ?Sized,
        P2: BoundDataProvider<tz::LocationsRootV1Marker> + ?Sized,
    {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::Location),
            length: FieldLength::Four,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.locations_root
            .load_put(root_provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.locations
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_time_zone_exemplar_cities_names<P, P2>(
        &mut self,
        provider: &P,
        root_provider: &P2,
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<tz::ExemplarCitiesV1Marker> + ?Sized,
        P2: BoundDataProvider<tz::ExemplarCitiesRootV1Marker> + ?Sized,
    {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::Location),
            length: FieldLength::Three,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.exemplars_root
            .load_put(root_provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.exemplars
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    fn load_mz_periods<P>(
        &mut self,
        provider: &P,
        error_field: ErrorField,
    ) -> Result<(), PatternLoadError>
    where
        P: BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized,
    {
        let variables = ();
        self.mz_periods
            .load_put(provider, Default::default(), variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        Ok(())
    }

    pub(crate) fn load_time_zone_generic_long_names(
        &mut self,
        provider: &(impl BoundDataProvider<tz::MzGenericLongV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::GenericNonLocation),
            length: FieldLength::Four,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.mz_generic_long
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.load_mz_periods(mz_period_provider, error_field)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_generic_short_names(
        &mut self,
        provider: &(impl BoundDataProvider<tz::MzGenericShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::GenericNonLocation),
            length: FieldLength::One,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.mz_generic_short
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.load_mz_periods(mz_period_provider, error_field)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_specific_long_names(
        &mut self,
        provider: &(impl BoundDataProvider<tz::MzSpecificLongV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::SpecificNonLocation),
            length: FieldLength::Four,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.mz_specific_long
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.load_mz_periods(mz_period_provider, error_field)?;
        Ok(())
    }

    pub(crate) fn load_time_zone_specific_short_names(
        &mut self,
        provider: &(impl BoundDataProvider<tz::MzSpecificShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), PatternLoadError> {
        let locale = provider
            .bound_marker()
            .make_locale(prefs.locale_preferences);
        let error_field = ErrorField(fields::Field {
            symbol: FieldSymbol::TimeZone(fields::TimeZone::SpecificNonLocation),
            length: FieldLength::One,
        });
        let variables = ();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        self.mz_specific_short
            .load_put(provider, req, variables)
            .map_err(|e| MaybePayloadError::into_load_error(e, error_field))?
            .map_err(|e| PatternLoadError::Data(e, error_field))?;
        self.load_mz_periods(mz_period_provider, error_field)?;
        Ok(())
    }

    pub(crate) fn load_fixed_decimal_formatter(
        &mut self,
        loader: &impl FixedDecimalFormatterLoader,
        prefs: DateTimeFormatterPreferences,
    ) -> Result<(), DataError> {
        if self.fixed_decimal_formatter.is_some() {
            return Ok(());
        }
        let mut options = FixedDecimalFormatterOptions::default();
        options.grouping_strategy = GroupingStrategy::Never;
        self.fixed_decimal_formatter = Some(FixedDecimalFormatterLoader::load(
            loader,
            (&prefs).into(),
            options,
        )?);
        Ok(())
    }

    /// Loads all data required for formatting the given [`PatternItem`]s.
    ///
    /// This function has a lot of arguments because many of the arguments are generic,
    /// and pulling them out to an options struct would be cumbersome.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn load_for_pattern(
        &mut self,
        year_provider: &(impl BoundDataProvider<YearNamesV1Marker> + ?Sized),
        month_provider: &(impl BoundDataProvider<MonthNamesV1Marker> + ?Sized),
        weekday_provider: &(impl BoundDataProvider<WeekdayNamesV1Marker> + ?Sized),
        dayperiod_provider: &(impl BoundDataProvider<DayPeriodNamesV1Marker> + ?Sized),
        zone_essentials_provider: &(impl BoundDataProvider<tz::EssentialsV1Marker> + ?Sized),
        locations_provider: &(impl BoundDataProvider<tz::LocationsV1Marker> + ?Sized),
        locations_root_provider: &(impl BoundDataProvider<tz::LocationsRootV1Marker> + ?Sized),
        exemplar_cities_provider: &(impl BoundDataProvider<tz::ExemplarCitiesV1Marker> + ?Sized),
        exemplar_cities_root_provider: &(impl BoundDataProvider<tz::ExemplarCitiesRootV1Marker>
              + ?Sized),
        mz_generic_long_provider: &(impl BoundDataProvider<tz::MzGenericLongV1Marker> + ?Sized),
        mz_generic_short_provider: &(impl BoundDataProvider<tz::MzGenericShortV1Marker> + ?Sized),
        mz_specific_long_provider: &(impl BoundDataProvider<tz::MzSpecificLongV1Marker> + ?Sized),
        mz_specific_short_provider: &(impl BoundDataProvider<tz::MzSpecificShortV1Marker> + ?Sized),
        mz_period_provider: &(impl BoundDataProvider<tz::MzPeriodV1Marker> + ?Sized),
        fixed_decimal_formatter_loader: &impl FixedDecimalFormatterLoader,
        prefs: DateTimeFormatterPreferences,
        pattern_items: impl Iterator<Item = PatternItem>,
    ) -> Result<(), PatternLoadError> {
        let mut numeric_field = None;

        for item in pattern_items {
            let PatternItem::Field(field) = item else {
                continue;
            };
            let error_field = ErrorField(field);

            use crate::provider::fields::*;
            use FieldLength::*;
            use FieldSymbol as FS;

            match (field.symbol, field.length) {
                ///// Textual symbols /////

                // G..GGGGG
                (FS::Era, One | Two | Three | Four | Five) => {
                    self.load_year_names(
                        year_provider,
                        prefs,
                        YearNameLength::from_field_length(field.length)
                            .ok_or(PatternLoadError::UnsupportedLength(error_field))?,
                        error_field,
                    )?;
                }

                // U..UUUUU
                (FS::Year(Year::Cyclic), One | Two | Three | Four | Five) => {
                    numeric_field = Some(field);
                    self.load_year_names(
                        year_provider,
                        prefs,
                        YearNameLength::from_field_length(field.length)
                            .ok_or(PatternLoadError::UnsupportedLength(error_field))?,
                        error_field,
                    )?;
                }

                // MMM..MMMMM, LLL..LLLLL
                (
                    FS::Month(field_symbol @ Month::Format | field_symbol @ Month::StandAlone),
                    Three | Four | Five,
                ) => {
                    self.load_month_names(
                        month_provider,
                        prefs,
                        MonthNameLength::from_field(field_symbol, field.length)
                            .ok_or(PatternLoadError::UnsupportedLength(error_field))?,
                        error_field,
                    )?;
                }

                // e..ee, c..cc
                (FS::Weekday(Weekday::Local | Weekday::StandAlone), One | Two) => {
                    // TODO(#5643): Requires locale-aware day-of-week calculation
                    return Err(PatternLoadError::UnsupportedLength(ErrorField(field)));
                }

                // E..EEEEEE, eee..eeeeee, ccc..cccccc
                (FS::Weekday(field_symbol), One | Two | Three | Four | Five | Six) => {
                    self.load_weekday_names(
                        weekday_provider,
                        prefs,
                        WeekdayNameLength::from_field(field_symbol, field.length)
                            .ok_or(PatternLoadError::UnsupportedLength(error_field))?,
                        error_field,
                    )?;
                }

                // a..aaaaa, b..bbbbb
                (FS::DayPeriod(field_symbol), One | Two | Three | Four | Five) => {
                    self.load_day_period_names(
                        dayperiod_provider,
                        prefs,
                        DayPeriodNameLength::from_field(field_symbol, field.length)
                            .ok_or(PatternLoadError::UnsupportedLength(error_field))?,
                        error_field,
                    )?;
                }

                ///// Time zone symbols /////

                // z..zzz
                (FS::TimeZone(TimeZone::SpecificNonLocation), One | Two | Three) => {
                    numeric_field = Some(field);
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    self.load_time_zone_specific_short_names(
                        mz_specific_short_provider,
                        mz_period_provider,
                        prefs,
                    )?;
                }
                // zzzz
                (FS::TimeZone(TimeZone::SpecificNonLocation), Four) => {
                    numeric_field = Some(field);
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    self.load_time_zone_specific_long_names(
                        mz_specific_long_provider,
                        mz_period_provider,
                        prefs,
                    )?;
                    self.load_time_zone_location_names(
                        locations_provider,
                        locations_root_provider,
                        prefs,
                    )?;
                }

                // v
                (FS::TimeZone(TimeZone::GenericNonLocation), One) => {
                    numeric_field = Some(field);
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    self.load_time_zone_generic_short_names(
                        mz_generic_short_provider,
                        mz_period_provider,
                        prefs,
                    )?;
                    // For fallback:
                    self.load_time_zone_location_names(
                        locations_provider,
                        locations_root_provider,
                        prefs,
                    )?;
                }
                // vvvv
                (FS::TimeZone(TimeZone::GenericNonLocation), Four) => {
                    numeric_field = Some(field);
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    self.load_time_zone_generic_long_names(
                        mz_generic_long_provider,
                        mz_period_provider,
                        prefs,
                    )?;
                    // For fallback:
                    self.load_time_zone_location_names(
                        locations_provider,
                        locations_root_provider,
                        prefs,
                    )?;
                }

                // V
                (FS::TimeZone(TimeZone::Location), One) => {
                    // no data required
                }
                // VVV
                (FS::TimeZone(TimeZone::Location), Three) => {
                    self.load_time_zone_location_names(
                        locations_provider,
                        locations_root_provider,
                        prefs,
                    )?;
                    self.load_time_zone_exemplar_cities_names(
                        exemplar_cities_provider,
                        exemplar_cities_root_provider,
                        prefs,
                    )?;
                }
                // VVVV
                (FS::TimeZone(TimeZone::Location), Four) => {
                    numeric_field = Some(field);
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    self.load_time_zone_location_names(
                        locations_provider,
                        locations_root_provider,
                        prefs,
                    )?;
                }

                // O, OOOO
                (FS::TimeZone(TimeZone::LocalizedOffset), One | Four) => {
                    self.load_time_zone_essentials(zone_essentials_provider, prefs)?;
                    numeric_field = Some(field);
                }

                // X..XXXXX, x..xxxxx
                (
                    FS::TimeZone(TimeZone::IsoWithZ | TimeZone::Iso),
                    One | Two | Three | Four | Five,
                ) => {
                    // no data required
                }

                ///// Numeric symbols /////

                // y+
                (FS::Year(Year::Calendar), _) => numeric_field = Some(field),
                // r+
                (FS::Year(Year::RelatedIso), _) => {
                    // always formats as ASCII
                }

                // M..MM, L..LL
                (FS::Month(_), One | Two) => numeric_field = Some(field),

                // d..dd
                (FS::Day(Day::DayOfMonth), One | Two) => numeric_field = Some(field),
                // D..DDD
                (FS::Day(Day::DayOfYear), One | Two | Three) => numeric_field = Some(field),
                // F
                (FS::Day(Day::DayOfWeekInMonth), One) => numeric_field = Some(field),

                // K..KK, h..hh, H..HH, k..kk
                (FS::Hour(_), One | Two) => numeric_field = Some(field),

                // m..mm
                (FS::Minute, One | Two) => numeric_field = Some(field),

                // s..ss
                (FS::Second(Second::Second), One | Two) => numeric_field = Some(field),

                // A+
                (FS::Second(Second::MillisInDay), _) => numeric_field = Some(field),

                // s.S+, ss.S+ (s is modelled by length, S+ by symbol)
                (FS::DecimalSecond(_), One | Two) => numeric_field = Some(field),

                ///// Unsupported symbols /////
                _ => {
                    return Err(PatternLoadError::UnsupportedLength(ErrorField(field)));
                }
            }
        }

        if let Some(field) = numeric_field {
            self.load_fixed_decimal_formatter(fixed_decimal_formatter_loader, prefs)
                .map_err(|e| PatternLoadError::Data(e, ErrorField(field)))?;
        }

        Ok(())
    }
}

impl RawDateTimeNamesBorrowed<'_> {
    pub(crate) fn get_name_for_month(
        &self,
        field_symbol: fields::Month,
        field_length: FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetNameForMonthError> {
        let month_name_length = MonthNameLength::from_field(field_symbol, field_length)
            .ok_or(GetNameForMonthError::InvalidFieldLength)?;
        let month_names = self
            .month_names
            .get_with_variables(month_name_length)
            .ok_or(GetNameForMonthError::NotLoaded)?;
        let Some((month_number, is_leap)) = code.parsed() else {
            return Err(GetNameForMonthError::InvalidMonthCode);
        };
        let Some(month_index) = month_number.checked_sub(1) else {
            return Err(GetNameForMonthError::InvalidMonthCode);
        };
        let month_index = usize::from(month_index);
        let name = match month_names {
            MonthNamesV1::Linear(linear) => {
                if is_leap {
                    None
                } else {
                    linear.get(month_index)
                }
            }
            MonthNamesV1::LeapLinear(leap_linear) => {
                let num_months = leap_linear.len() / 2;
                if is_leap {
                    leap_linear.get(month_index + num_months)
                } else if month_index < num_months {
                    leap_linear.get(month_index)
                } else {
                    None
                }
            }
            MonthNamesV1::LeapNumeric(leap_numeric) => {
                if is_leap {
                    return Ok(MonthPlaceholderValue::NumericPattern(leap_numeric));
                } else {
                    return Ok(MonthPlaceholderValue::Numeric);
                }
            }
        };
        // Note: Always return `false` for the second argument since neo MonthNames
        // knows how to handle leap months and we don't need the fallback logic
        name.map(MonthPlaceholderValue::PlainString)
            .ok_or(GetNameForMonthError::InvalidMonthCode)
    }

    pub(crate) fn get_name_for_weekday(
        &self,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetNameForWeekdayError> {
        let weekday_name_length = WeekdayNameLength::from_field(field_symbol, field_length)
            .ok_or(GetNameForWeekdayError::InvalidFieldLength)?;
        let weekday_names = self
            .weekday_names
            .get_with_variables(weekday_name_length)
            .ok_or(GetNameForWeekdayError::NotLoaded)?;
        weekday_names
            .names
            .get((day as usize) % 7)
            // TODO: make weekday_names length 7 in the type system
            .ok_or(GetNameForWeekdayError::NotLoaded)
    }

    /// Gets the era symbol, or `None` if data is loaded but symbol isn't found.
    ///
    /// `None` should fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    pub(crate) fn get_name_for_era(
        &self,
        field_length: FieldLength,
        era: FormattingEra,
    ) -> Result<&str, GetNameForEraError> {
        let year_name_length = YearNameLength::from_field_length(field_length)
            .ok_or(GetNameForEraError::InvalidFieldLength)?;
        let year_names = self
            .year_names
            .get_with_variables(year_name_length)
            .ok_or(GetNameForEraError::NotLoaded)?;

        match (year_names, era) {
            (YearNamesV1::VariableEras(era_names), FormattingEra::Code(era_code)) => {
                crate::provider::neo::get_year_name_from_map(era_names, era_code.0.as_str().into())
                    .ok_or(GetNameForEraError::InvalidEraCode)
            }
            (YearNamesV1::FixedEras(era_names), FormattingEra::Index(index, _fallback)) => {
                era_names
                    .get(index.into())
                    .ok_or(GetNameForEraError::InvalidEraCode)
            }
            _ => Err(GetNameForEraError::InvalidEraCode),
        }
    }

    pub(crate) fn get_name_for_cyclic(
        &self,
        field_length: FieldLength,
        cyclic: NonZeroU8,
    ) -> Result<&str, GetNameForCyclicYearError> {
        let year_name_length = YearNameLength::from_field_length(field_length)
            .ok_or(GetNameForCyclicYearError::InvalidFieldLength)?;
        let year_names = self
            .year_names
            .get_with_variables(year_name_length)
            .ok_or(GetNameForCyclicYearError::NotLoaded)?;

        let YearNamesV1::Cyclic(cyclics) = year_names else {
            return Err(GetNameForCyclicYearError::InvalidYearNumber { max: 0 });
        };

        cyclics.get((cyclic.get() as usize) - 1).ok_or(
            GetNameForCyclicYearError::InvalidYearNumber {
                max: cyclics.len() + 1,
            },
        )
    }

    pub(crate) fn get_name_for_day_period(
        &self,
        field_symbol: fields::DayPeriod,
        field_length: FieldLength,
        hour: icu_timezone::types::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetNameForDayPeriodError> {
        use fields::DayPeriod::NoonMidnight;
        let day_period_name_length = DayPeriodNameLength::from_field(field_symbol, field_length)
            .ok_or(GetNameForDayPeriodError::InvalidFieldLength)?;
        let dayperiod_names = self
            .dayperiod_names
            .get_with_variables(day_period_name_length)
            .ok_or(GetNameForDayPeriodError::NotLoaded)?;
        let option_value: Option<&str> = match (field_symbol, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => dayperiod_names.midnight().or_else(|| dayperiod_names.am()),
            (NoonMidnight, 12, true) => dayperiod_names.noon().or_else(|| dayperiod_names.pm()),
            (_, hour, _) if hour < 12 => dayperiod_names.am(),
            _ => dayperiod_names.pm(),
        };
        option_value.ok_or(GetNameForDayPeriodError::NotLoaded)
    }
}

/// A container contains all data payloads for time zone formatting (borrowed version).
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TimeZoneDataPayloadsBorrowed<'a> {
    /// The data that contains meta information about how to display content.
    pub(crate) essentials: Option<&'a tz::EssentialsV1<'a>>,
    /// The root location names, e.g. Italy
    pub(crate) locations_root: Option<&'a tz::LocationsV1<'a>>,
    /// The language specific location names, e.g. Italia
    pub(crate) locations: Option<&'a tz::LocationsV1<'a>>,
    /// The root exemplar city names, e.g. Rome
    pub(crate) exemplars_root: Option<&'a tz::ExemplarCitiesV1<'a>>,
    /// The language specific exemplar names, e.g. Roma
    pub(crate) exemplars: Option<&'a tz::ExemplarCitiesV1<'a>>,
    /// The generic long metazone names, e.g. Pacific Time
    pub(crate) mz_generic_long: Option<&'a tz::MzGenericV1<'a>>,
    /// The generic short metazone names, e.g. PT
    pub(crate) mz_generic_short: Option<&'a tz::MzGenericV1<'a>>,
    /// The specific long metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_long: Option<&'a tz::MzSpecificV1<'a>>,
    /// The specific short metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_short: Option<&'a tz::MzSpecificV1<'a>>,
    /// The metazone lookup
    pub(crate) mz_periods: Option<&'a tz::MzPeriodV1<'a>>,
}

impl<'data> RawDateTimeNamesBorrowed<'data> {
    pub(crate) fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data> {
        TimeZoneDataPayloadsBorrowed {
            essentials: self.zone_essentials.get_option(),
            locations_root: self.locations_root.get_option(),
            locations: self.locations.get_option(),
            exemplars: self.exemplars.get_option(),
            exemplars_root: self.exemplars_root.get_option(),
            mz_generic_long: self.mz_generic_long.get_option(),
            mz_generic_short: self.mz_generic_short.get_option(),
            mz_specific_long: self.mz_specific_long.get_option(),
            mz_specific_short: self.mz_specific_short.get_option(),
            mz_periods: self.mz_periods.get_option(),
        }
    }
}
