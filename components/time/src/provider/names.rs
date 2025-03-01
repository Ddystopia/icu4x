// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Property names-related data for this component
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::TimeZone;
use icu_provider::prelude::*;
use zerotrie::ZeroAsciiIgnoreCaseTrie;
use zerovec::{VarZeroVec, ZeroVec};

/// [`IanaToBcp47Map`]'s trie cannot handle differently-cased prefixes, like `Mexico/BajaSur`` and `MET`.
///
/// Therefore, any ID that is not of the shape `{region}/{city}` gets prefixed with this character
/// inside the trie.
///
/// During lookup, if the input is not of the shape `{region}/{city}`, the trie cursor has to be advanced over
/// this byte.
pub const NON_REGION_CITY_PREFIX: u8 = b'_';

/// A mapping from normal-case IANA time zone identifiers to BCP-47 time zone identifiers.
///
/// Multiple IANA time zone IDs can map to the same BCP-47 time zone ID.
///
/// This markers uses a checksum to ensure consistency with [`Bcp47ToIanaMap`].
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(marker(
    IanaToBcp47MapV3,
    "time_zone/iana_to_bcp47@3",
    singleton,
    has_checksum
))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct IanaToBcp47Map<'data> {
    /// A map from normal-case IANA time zone identifiers to indexes of BCP-47 time zone
    /// identifiers along with a canonical flag. The IANA identifiers are normal-case.
    ///
    /// The `usize` values stored in the trie have the following form:
    ///
    /// - Lowest bit: 1 if canonical, 0 if not canonical
    /// - All remaining bits: index into `bcp47_ids`
    ///
    /// For example, in CLDR 44, `"Africa/Abidjan"` has value 221, which means it is canonical
    /// (low bit is 1 == odd number) and the index into `bcp47_ids` is 110 (221 >> 1).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroAsciiIgnoreCaseTrie<ZeroVec<'data, u8>>,
    /// A sorted list of BCP-47 time zone identifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    // Note: this is 9739B as `ZeroVec<TimeZone>` (`ZeroVec<TinyStr8>`)
    // and 9335B as `VarZeroVec<str>`
    pub bcp47_ids: ZeroVec<'data, TimeZone>,
}

/// A mapping from IANA time zone identifiers to BCP-47 time zone identifiers.
///
/// The BCP-47 time zone ID maps to the default IANA time zone ID according to the CLDR data.
///
/// This markers uses a checksum to ensure consistency with [`IanaToBcp47Map`].
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(marker(
    Bcp47ToIanaMapV1,
    "time_zone/bcp47_to_iana@1",
    singleton,
    has_checksum
))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct Bcp47ToIanaMap<'data> {
    /// The IANA time zone identifier corresponding to the BCP-47 ID in
    /// [`IanaToBcp47Map::bcp47_ids`].
    ///
    /// Since there can be more than one IANA identifier for a particular
    /// BCP-47 identifier, this list contains only the current canonical
    /// IANA identifier.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub canonical_iana_ids: VarZeroVec<'data, str>,
}
