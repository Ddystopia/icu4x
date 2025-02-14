// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::time::provider::names::*;
use icu::time::TimeZone;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::hash::Hasher;
use zerotrie::ZeroAsciiIgnoreCaseTrie;
use zerovec::{ZeroSlice, ZeroVec};

impl DataProvider<IanaToBcp47MapV3> for SourceDataProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<IanaToBcp47MapV3>, DataError> {
        let iana2bcp = self.iana_to_bcp47_map()?;

        // Sort and deduplicate the BCP-47 IDs:
        let bcp_set: BTreeSet<TimeZone> = iana2bcp.values().copied().collect();
        let bcp47_ids: ZeroVec<TimeZone> = bcp_set.into_iter().collect();
        let bcp47_ids_checksum = compute_bcp47_ids_hash(&bcp47_ids);

        // Get the canonical IANA names.
        // Note: The BTreeMap retains the order of the aliases, which is important for establishing
        // the canonical order of the IANA names.
        let bcp2iana = self.bcp47_to_canonical_iana_map()?;

        // Transform the map to use BCP indices:
        #[allow(clippy::unwrap_used)] // structures are derived from each other
        let map: BTreeMap<Vec<u8>, usize> = iana2bcp
            .iter()
            .map(|(iana, bcp)| {
                let is_canonical = bcp2iana.get(bcp) == Some(iana);
                let index = bcp47_ids.binary_search(bcp).unwrap();
                (
                    if iana.contains('/') {
                        iana.to_owned()
                    } else {
                        format!(
                            "{}{iana}",
                            char::from_u32(
                                icu::time::provider::names::NON_REGION_CITY_PREFIX as u32
                            )
                            .unwrap()
                        )
                    }
                    .into_bytes(),
                    (index << 1) | (is_canonical as usize),
                )
            })
            .collect();

        let data_struct = IanaToBcp47Map {
            map: ZeroAsciiIgnoreCaseTrie::try_from(&map)
                .map_err(|e| {
                    DataError::custom("Could not create ZeroTrie from timezone.json data")
                        .with_display_context(&e)
                })?
                .convert_store(),
            bcp47_ids,
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(bcp47_ids_checksum),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<IanaToBcp47MapV3> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<Bcp47ToIanaMapV1> for SourceDataProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<Bcp47ToIanaMapV1>, DataError> {
        // Note: The BTreeMap retains the order of the aliases, which is important for establishing
        // the canonical order of the IANA names.
        let bcp2iana = self.bcp47_to_canonical_iana_map()?;
        let bcp47_ids: ZeroVec<TimeZone> = bcp2iana.keys().copied().collect();
        let bcp47_ids_checksum = compute_bcp47_ids_hash(&bcp47_ids);

        // Make the VarZeroVec of canonical IANA names.
        // Note: we can't build VarZeroVec from an iterator yet.
        let iana_vec: Vec<&String> = bcp2iana.values().collect();
        let canonical_iana_ids = iana_vec.as_slice().into();

        let data_struct = Bcp47ToIanaMap { canonical_iana_ids };
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(bcp47_ids_checksum),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<Bcp47ToIanaMapV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

fn create_hasher() -> impl std::hash::Hasher {
    twox_hash::XxHash64::with_seed(0)
}

fn compute_bcp47_ids_hash(bcp47_ids: &ZeroSlice<TimeZone>) -> u64 {
    let mut hasher = create_hasher();
    hasher.write(bcp47_ids.as_bytes());
    hasher.finish()
}

#[test]
fn test_compute_bcp47_ids_hash() {
    let bcp47_ids: ZeroVec<TimeZone> = [
        TimeZone(tinystr::tinystr!(8, "aedxb")),
        TimeZone(tinystr::tinystr!(8, "brfor")),
        TimeZone(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();

    // Checksum 1: the default output of the zeroslice hashing function
    let checksum1 = compute_bcp47_ids_hash(&bcp47_ids);
    assert_eq!(checksum1, 0x66FA043B31200DCB); // stability

    // Checksum 2: hashing of each individual element
    // (should equal 1)
    let mut hasher = create_hasher();
    for bcp47 in bcp47_ids.iter() {
        hasher.write(bcp47.0.all_bytes());
    }
    let checksum2 = hasher.finish();
    assert_eq!(checksum1, checksum2);

    // Checksum 3: hashing of a ZeroVec in a different order
    // (should not equal 1)
    let bcp47_ids_rev: ZeroVec<TimeZone> = [
        TimeZone(tinystr::tinystr!(8, "usinvev")),
        TimeZone(tinystr::tinystr!(8, "aedxb")),
        TimeZone(tinystr::tinystr!(8, "brfor")),
    ]
    .into_iter()
    .collect();
    let checksum3 = compute_bcp47_ids_hash(&bcp47_ids_rev);
    assert_ne!(checksum1, checksum3);

    // Checksum 4: moving letters between the elements should change the hash
    // (should not equal 1)
    let bcp47_ids_roll: ZeroVec<TimeZone> = [
        TimeZone(tinystr::tinystr!(8, "aedx")),
        TimeZone(tinystr::tinystr!(8, "bbrfor")),
        TimeZone(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();
    let checksum4 = compute_bcp47_ids_hash(&bcp47_ids_roll);
    assert_ne!(checksum1, checksum4);

    // Checksum 5: empty strings at the end should change the hash
    // (should not equal 1)
    let bcp47_ids_empty_end: ZeroVec<TimeZone> = [
        TimeZone(tinystr::tinystr!(8, "aedxb")),
        TimeZone(tinystr::tinystr!(8, "brfor")),
        TimeZone(tinystr::tinystr!(8, "usinvev")),
        TimeZone(tinystr::tinystr!(8, "")),
    ]
    .into_iter()
    .collect();
    let checksum5 = compute_bcp47_ids_hash(&bcp47_ids_empty_end);
    assert_ne!(checksum1, checksum5);

    // Checksum 6: empty strings in the middle should change the hash
    // (should not equal 1 or 5)
    let bcp47_ids_empty_middle: ZeroVec<TimeZone> = [
        TimeZone(tinystr::tinystr!(8, "aedxb")),
        TimeZone(tinystr::tinystr!(8, "")),
        TimeZone(tinystr::tinystr!(8, "brfor")),
        TimeZone(tinystr::tinystr!(8, "usinvev")),
    ]
    .into_iter()
    .collect();
    let checksum6 = compute_bcp47_ids_hash(&bcp47_ids_empty_middle);
    assert_ne!(checksum1, checksum6);
    assert_ne!(checksum5, checksum6);

    // Additional coverage
    assert_ne!(checksum2, checksum3);
    assert_ne!(checksum2, checksum4);
    assert_ne!(checksum2, checksum5);
    assert_ne!(checksum2, checksum6);
    assert_ne!(checksum3, checksum4);
    assert_ne!(checksum3, checksum5);
    assert_ne!(checksum3, checksum6);
    assert_ne!(checksum4, checksum5);
    assert_ne!(checksum4, checksum6);
}

/// Tests that all IANA time zone IDs normalize and canonicalize to their correct form.
#[test]
fn test_normalize_canonicalize_iana_coverage() {
    let provider = crate::SourceDataProvider::new_testing();

    let iana2bcp = provider.iana_to_bcp47_map().unwrap();

    let mapper = icu::time::zone::IanaParser::try_new_unstable(&provider).unwrap();
    let mapper = mapper.as_borrowed();

    for iana_id in iana2bcp.keys() {
        let normalized = mapper.normalize_iana(iana_id).unwrap().0;
        assert_eq!(&normalized, iana_id);
    }

    let bcp2iana = provider.bcp47_to_canonical_iana_map().unwrap();
    for (iana_id, bcp47_id) in iana2bcp.iter() {
        let canonicalized = mapper.canonicalize_iana(iana_id).unwrap().0;
        assert_eq!(&canonicalized, bcp2iana.get(bcp47_id).unwrap());
    }
}
