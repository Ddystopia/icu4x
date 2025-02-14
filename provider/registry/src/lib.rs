// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Exposes the list of all known `DataMarker`s.
//!
//! This is modeled as a macro that accepts a callback macro of the shape:
//!
//! ```
//! macro_rules! cb {
//!     ($($marker_ty:ty:$marker:ident,)+ #[experimental] $($emarker_ty:ty:$emarker:ident,)+) => {
//!         // Do something for each marker, or each experimental marker
//!     };
//! }
//! ```
//!
//! Calling this as `registry!(cb);` evaluates `cb` with the list of markers.

#[macro_export]
/// See the crate docs.
macro_rules! registry(
    ($cb:ident) => {
        cb!(
            icu::calendar::provider::ChineseCacheV1: ChineseCacheV1,
            icu::calendar::provider::DangiCacheV1: DangiCacheV1,
            icu::calendar::provider::JapaneseErasV1: JapaneseErasV1,
            icu::calendar::provider::IslamicObservationalCacheV1: IslamicObservationalCacheV1,
            icu::calendar::provider::IslamicUmmAlQuraCacheV1: IslamicUmmAlQuraCacheV1,
            icu::calendar::provider::JapaneseExtendedErasV1: JapaneseExtendedErasV1,
            icu::calendar::provider::WeekDataV2: WeekDataV2,
            icu::casemap::provider::CaseMapV1: CaseMapV1,
            icu::casemap::provider::CaseMapUnfoldV1: CaseMapUnfoldV1,
            icu::collator::provider::CollationRootV1: CollationRootV1,
            icu::collator::provider::CollationTailoringV1: CollationTailoringV1,
            icu::collator::provider::CollationDiacriticsV1: CollationDiacriticsV1,
            icu::collator::provider::CollationJamoV1: CollationJamoV1,
            icu::collator::provider::CollationMetadataV1: CollationMetadataV1,
            icu::collator::provider::CollationReorderingV1: CollationReorderingV1,
            icu::collator::provider::CollationSpecialPrimariesV1: CollationSpecialPrimariesV1,
            icu::datetime::provider::time_zones::LocationsV1: LocationsV1,
            icu::datetime::provider::time_zones::LocationsRootV1: LocationsRootV1,
            icu::datetime::provider::time_zones::ExemplarCitiesV1: ExemplarCitiesV1,
            icu::datetime::provider::time_zones::ExemplarCitiesRootV1: ExemplarCitiesRootV1,
            icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1: MetazoneGenericNamesLongV1,
            icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1: MetazoneGenericNamesShortV1,
            icu::datetime::provider::time_zones::MetazonePeriodV1: MetazonePeriodV1,
            icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1: MetazoneSpecificNamesLongV1,
            icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1: MetazoneSpecificNamesShortV1,
            icu::datetime::provider::time_zones::TimeZoneEssentialsV1: TimeZoneEssentialsV1,
            icu::time::provider::ZoneOffsetPeriodV1: ZoneOffsetPeriodV1,
            icu::decimal::provider::DecimalDigitsV1: DecimalDigitsV1,
            icu::decimal::provider::DecimalSymbolsV2: DecimalSymbolsV2,
            icu::list::provider::ListAndV2: ListAndV2,
            icu::list::provider::ListOrV2: ListOrV2,
            icu::list::provider::ListUnitV2: ListUnitV2,
            icu::locale::provider::AliasesV2: AliasesV2,
            icu::locale::provider::ExemplarCharactersAuxiliaryV1: ExemplarCharactersAuxiliaryV1,
            icu::locale::provider::ExemplarCharactersIndexV1: ExemplarCharactersIndexV1,
            icu::locale::provider::ExemplarCharactersMainV1: ExemplarCharactersMainV1,
            icu::locale::provider::ExemplarCharactersNumbersV1: ExemplarCharactersNumbersV1,
            icu::locale::provider::ExemplarCharactersPunctuationV1: ExemplarCharactersPunctuationV1,
            icu::locale::provider::LikelySubtagsExtendedV1: LikelySubtagsExtendedV1,
            icu::locale::provider::LikelySubtagsForLanguageV1: LikelySubtagsForLanguageV1,
            icu::locale::provider::LikelySubtagsForScriptRegionV1: LikelySubtagsForScriptRegionV1,
            icu::locale::provider::ParentsV1: ParentsV1,
            icu::locale::provider::ScriptDirectionV1: ScriptDirectionV1,
            icu::normalizer::provider::CanonicalCompositionsV1: CanonicalCompositionsV1,
            icu::normalizer::provider::CanonicalDecompositionDataV2: CanonicalDecompositionDataV2,
            icu::normalizer::provider::CanonicalDecompositionTablesV1: CanonicalDecompositionTablesV1,
            icu::normalizer::provider::CompatibilityDecompositionDataV2: CompatibilityDecompositionDataV2,
            icu::normalizer::provider::CompatibilityDecompositionTablesV1: CompatibilityDecompositionTablesV1,
            icu::normalizer::provider::NonRecursiveDecompositionSupplementV1: NonRecursiveDecompositionSupplementV1,
            icu::normalizer::provider::Uts46DecompositionDataV2: Uts46DecompositionDataV2,
            icu::plurals::provider::CardinalV1: CardinalV1,
            icu::plurals::provider::OrdinalV1: OrdinalV1,
            icu::properties::provider::AlnumV1: AlnumV1,
            icu::properties::provider::AlphabeticV1: AlphabeticV1,
            icu::properties::provider::AsciiHexDigitV1: AsciiHexDigitV1,
            icu::properties::provider::BasicEmojiV1: BasicEmojiV1,
            icu::properties::provider::BidiMirroringGlyphV1: BidiMirroringGlyphV1,
            icu::properties::provider::BidiClassV1: BidiClassV1,
            icu::properties::provider::BidiControlV1: BidiControlV1,
            icu::properties::provider::BidiMirroredV1: BidiMirroredV1,
            icu::properties::provider::BlankV1: BlankV1,
            icu::properties::provider::CanonicalCombiningClassV1: CanonicalCombiningClassV1,
            icu::properties::provider::CasedV1: CasedV1,
            icu::properties::provider::CaseIgnorableV1: CaseIgnorableV1,
            icu::properties::provider::CaseSensitiveV1: CaseSensitiveV1,
            icu::properties::provider::ChangesWhenCasefoldedV1: ChangesWhenCasefoldedV1,
            icu::properties::provider::ChangesWhenCasemappedV1: ChangesWhenCasemappedV1,
            icu::properties::provider::ChangesWhenLowercasedV1: ChangesWhenLowercasedV1,
            icu::properties::provider::ChangesWhenNfkcCasefoldedV1: ChangesWhenNfkcCasefoldedV1,
            icu::properties::provider::ChangesWhenTitlecasedV1: ChangesWhenTitlecasedV1,
            icu::properties::provider::ChangesWhenUppercasedV1: ChangesWhenUppercasedV1,
            icu::properties::provider::DashV1: DashV1,
            icu::properties::provider::DefaultIgnorableCodePointV1: DefaultIgnorableCodePointV1,
            icu::properties::provider::DeprecatedV1: DeprecatedV1,
            icu::properties::provider::DiacriticV1: DiacriticV1,
            icu::properties::provider::EastAsianWidthV1: EastAsianWidthV1,
            icu::properties::provider::EmojiComponentV1: EmojiComponentV1,
            icu::properties::provider::EmojiModifierBaseV1: EmojiModifierBaseV1,
            icu::properties::provider::EmojiModifierV1: EmojiModifierV1,
            icu::properties::provider::EmojiPresentationV1: EmojiPresentationV1,
            icu::properties::provider::EmojiV1: EmojiV1,
            icu::properties::provider::ExtendedPictographicV1: ExtendedPictographicV1,
            icu::properties::provider::ExtenderV1: ExtenderV1,
            icu::properties::provider::FullCompositionExclusionV1: FullCompositionExclusionV1,
            icu::properties::provider::GeneralCategoryV1: GeneralCategoryV1,
            icu::properties::provider::GraphemeBaseV1: GraphemeBaseV1,
            icu::properties::provider::GraphemeClusterBreakV1: GraphemeClusterBreakV1,
            icu::properties::provider::GraphemeExtendV1: GraphemeExtendV1,
            icu::properties::provider::GraphemeLinkV1: GraphemeLinkV1,
            icu::properties::provider::GraphV1: GraphV1,
            icu::properties::provider::HangulSyllableTypeV1: HangulSyllableTypeV1,
            icu::properties::provider::HexDigitV1: HexDigitV1,
            icu::properties::provider::HyphenV1: HyphenV1,
            icu::properties::provider::IdContinueV1: IdContinueV1,
            icu::properties::provider::IdeographicV1: IdeographicV1,
            icu::properties::provider::IdsBinaryOperatorV1: IdsBinaryOperatorV1,
            icu::properties::provider::IdStartV1: IdStartV1,
            icu::properties::provider::IdsTrinaryOperatorV1: IdsTrinaryOperatorV1,
            icu::properties::provider::IndicSyllabicCategoryV1: IndicSyllabicCategoryV1,
            icu::properties::provider::JoinControlV1: JoinControlV1,
            icu::properties::provider::JoiningTypeV1: JoiningTypeV1,
            icu::properties::provider::LineBreakV1: LineBreakV1,
            icu::properties::provider::LogicalOrderExceptionV1: LogicalOrderExceptionV1,
            icu::properties::provider::LowercaseV1: LowercaseV1,
            icu::properties::provider::MathV1: MathV1,
            icu::properties::provider::BidiClassNameToValueV2: BidiClassNameToValueV2,
            icu::properties::provider::BidiClassValueToLongNameV1: BidiClassValueToLongNameV1,
            icu::properties::provider::BidiClassValueToShortNameV1: BidiClassValueToShortNameV1,
            icu::properties::provider::CanonicalCombiningClassNameToValueV2: CanonicalCombiningClassNameToValueV2,
            icu::properties::provider::CanonicalCombiningClassValueToLongNameV1: CanonicalCombiningClassValueToLongNameV1,
            icu::properties::provider::CanonicalCombiningClassValueToShortNameV1: CanonicalCombiningClassValueToShortNameV1,
            icu::properties::provider::EastAsianWidthNameToValueV2: EastAsianWidthNameToValueV2,
            icu::properties::provider::EastAsianWidthValueToLongNameV1: EastAsianWidthValueToLongNameV1,
            icu::properties::provider::EastAsianWidthValueToShortNameV1: EastAsianWidthValueToShortNameV1,
            icu::properties::provider::GeneralCategoryMaskNameToValueV2: GeneralCategoryMaskNameToValueV2,
            icu::properties::provider::GeneralCategoryNameToValueV2: GeneralCategoryNameToValueV2,
            icu::properties::provider::GeneralCategoryValueToLongNameV1: GeneralCategoryValueToLongNameV1,
            icu::properties::provider::GeneralCategoryValueToShortNameV1: GeneralCategoryValueToShortNameV1,
            icu::properties::provider::GraphemeClusterBreakNameToValueV2: GraphemeClusterBreakNameToValueV2,
            icu::properties::provider::GraphemeClusterBreakValueToLongNameV1: GraphemeClusterBreakValueToLongNameV1,
            icu::properties::provider::GraphemeClusterBreakValueToShortNameV1: GraphemeClusterBreakValueToShortNameV1,
            icu::properties::provider::HangulSyllableTypeNameToValueV2: HangulSyllableTypeNameToValueV2,
            icu::properties::provider::HangulSyllableTypeValueToLongNameV1: HangulSyllableTypeValueToLongNameV1,
            icu::properties::provider::HangulSyllableTypeValueToShortNameV1: HangulSyllableTypeValueToShortNameV1,
            icu::properties::provider::IndicSyllabicCategoryNameToValueV2: IndicSyllabicCategoryNameToValueV2,
            icu::properties::provider::IndicSyllabicCategoryValueToLongNameV1: IndicSyllabicCategoryValueToLongNameV1,
            icu::properties::provider::IndicSyllabicCategoryValueToShortNameV1: IndicSyllabicCategoryValueToShortNameV1,
            icu::properties::provider::JoiningTypeNameToValueV2: JoiningTypeNameToValueV2,
            icu::properties::provider::JoiningTypeValueToLongNameV1: JoiningTypeValueToLongNameV1,
            icu::properties::provider::JoiningTypeValueToShortNameV1: JoiningTypeValueToShortNameV1,
            icu::properties::provider::LineBreakNameToValueV2: LineBreakNameToValueV2,
            icu::properties::provider::LineBreakValueToLongNameV1: LineBreakValueToLongNameV1,
            icu::properties::provider::LineBreakValueToShortNameV1: LineBreakValueToShortNameV1,
            icu::properties::provider::ScriptNameToValueV2: ScriptNameToValueV2,
            icu::properties::provider::ScriptValueToLongNameV1: ScriptValueToLongNameV1,
            icu::properties::provider::ScriptValueToShortNameV1: ScriptValueToShortNameV1,
            icu::properties::provider::SentenceBreakNameToValueV2: SentenceBreakNameToValueV2,
            icu::properties::provider::SentenceBreakValueToLongNameV1: SentenceBreakValueToLongNameV1,
            icu::properties::provider::SentenceBreakValueToShortNameV1: SentenceBreakValueToShortNameV1,
            icu::properties::provider::WordBreakNameToValueV2: WordBreakNameToValueV2,
            icu::properties::provider::WordBreakValueToLongNameV1: WordBreakValueToLongNameV1,
            icu::properties::provider::WordBreakValueToShortNameV1: WordBreakValueToShortNameV1,
            icu::properties::provider::NfcInertV1: NfcInertV1,
            icu::properties::provider::NfdInertV1: NfdInertV1,
            icu::properties::provider::NfkcInertV1: NfkcInertV1,
            icu::properties::provider::NfkdInertV1: NfkdInertV1,
            icu::properties::provider::NoncharacterCodePointV1: NoncharacterCodePointV1,
            icu::properties::provider::PatternSyntaxV1: PatternSyntaxV1,
            icu::properties::provider::PatternWhiteSpaceV1: PatternWhiteSpaceV1,
            icu::properties::provider::PrependedConcatenationMarkV1: PrependedConcatenationMarkV1,
            icu::properties::provider::PrintV1: PrintV1,
            icu::properties::provider::QuotationMarkV1: QuotationMarkV1,
            icu::properties::provider::RadicalV1: RadicalV1,
            icu::properties::provider::RegionalIndicatorV1: RegionalIndicatorV1,
            icu::properties::provider::ScriptV1: ScriptV1,
            icu::properties::provider::ScriptWithExtensionsPropertyV1: ScriptWithExtensionsPropertyV1,
            icu::properties::provider::SegmentStarterV1: SegmentStarterV1,
            icu::properties::provider::SentenceBreakV1: SentenceBreakV1,
            icu::properties::provider::SentenceTerminalV1: SentenceTerminalV1,
            icu::properties::provider::SoftDottedV1: SoftDottedV1,
            icu::properties::provider::TerminalPunctuationV1: TerminalPunctuationV1,
            icu::properties::provider::UnifiedIdeographV1: UnifiedIdeographV1,
            icu::properties::provider::UppercaseV1: UppercaseV1,
            icu::properties::provider::VariationSelectorV1: VariationSelectorV1,
            icu::properties::provider::WhiteSpaceV1: WhiteSpaceV1,
            icu::properties::provider::WordBreakV1: WordBreakV1,
            icu::properties::provider::XdigitV1: XdigitV1,
            icu::properties::provider::XidContinueV1: XidContinueV1,
            icu::properties::provider::XidStartV1: XidStartV1,
            icu::segmenter::provider::DictionaryForWordLineExtendedV1: DictionaryForWordLineExtendedV1,
            icu::segmenter::provider::DictionaryForWordOnlyAutoV1: DictionaryForWordOnlyAutoV1,
            icu::segmenter::provider::GraphemeClusterBreakDataV2: GraphemeClusterBreakDataV2,
            icu::segmenter::provider::LineBreakDataV2: LineBreakDataV2,
            icu::segmenter::provider::LstmForWordLineAutoV1: LstmForWordLineAutoV1,
            icu::segmenter::provider::SentenceBreakDataOverrideV1: SentenceBreakDataOverrideV1,
            icu::segmenter::provider::SentenceBreakDataV2: SentenceBreakDataV2,
            icu::segmenter::provider::WordBreakDataOverrideV1: WordBreakDataOverrideV1,
            icu::segmenter::provider::WordBreakDataV2: WordBreakDataV2,
            icu::time::provider::names::Bcp47ToIanaMapV1: Bcp47ToIanaMapV1,
            icu::time::provider::names::IanaToBcp47MapV3: IanaToBcp47MapV3,
            icu::time::provider::windows::WindowsZonesToBcp47MapV1: WindowsZonesToBcp47MapV1,
            icu::datetime::provider::neo::WeekdayNamesV1: WeekdayNamesV1,
            icu::datetime::provider::neo::DayPeriodNamesV1: DayPeriodNamesV1,
            icu::datetime::provider::neo::GluePatternV1: GluePatternV1,
            icu::datetime::provider::neo::BuddhistYearNamesV1: BuddhistYearNamesV1,
            icu::datetime::provider::neo::ChineseYearNamesV1: ChineseYearNamesV1,
            icu::datetime::provider::neo::CopticYearNamesV1: CopticYearNamesV1,
            icu::datetime::provider::neo::DangiYearNamesV1: DangiYearNamesV1,
            icu::datetime::provider::neo::EthiopianYearNamesV1: EthiopianYearNamesV1,
            icu::datetime::provider::neo::GregorianYearNamesV1: GregorianYearNamesV1,
            icu::datetime::provider::neo::HebrewYearNamesV1: HebrewYearNamesV1,
            icu::datetime::provider::neo::IndianYearNamesV1: IndianYearNamesV1,
            icu::datetime::provider::neo::IslamicYearNamesV1: IslamicYearNamesV1,
            icu::datetime::provider::neo::JapaneseYearNamesV1: JapaneseYearNamesV1,
            icu::datetime::provider::neo::JapaneseExtendedYearNamesV1: JapaneseExtendedYearNamesV1,
            icu::datetime::provider::neo::PersianYearNamesV1: PersianYearNamesV1,
            icu::datetime::provider::neo::RocYearNamesV1: RocYearNamesV1,
            icu::datetime::provider::neo::BuddhistMonthNamesV1: BuddhistMonthNamesV1,
            icu::datetime::provider::neo::ChineseMonthNamesV1: ChineseMonthNamesV1,
            icu::datetime::provider::neo::CopticMonthNamesV1: CopticMonthNamesV1,
            icu::datetime::provider::neo::DangiMonthNamesV1: DangiMonthNamesV1,
            icu::datetime::provider::neo::EthiopianMonthNamesV1: EthiopianMonthNamesV1,
            icu::datetime::provider::neo::GregorianMonthNamesV1: GregorianMonthNamesV1,
            icu::datetime::provider::neo::HebrewMonthNamesV1: HebrewMonthNamesV1,
            icu::datetime::provider::neo::IndianMonthNamesV1: IndianMonthNamesV1,
            icu::datetime::provider::neo::IslamicMonthNamesV1: IslamicMonthNamesV1,
            icu::datetime::provider::neo::JapaneseMonthNamesV1: JapaneseMonthNamesV1,
            icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1: JapaneseExtendedMonthNamesV1,
            icu::datetime::provider::neo::PersianMonthNamesV1: PersianMonthNamesV1,
            icu::datetime::provider::neo::RocMonthNamesV1: RocMonthNamesV1,
            icu::datetime::provider::BuddhistDateNeoSkeletonPatternsV1: BuddhistDateNeoSkeletonPatternsV1,
            icu::datetime::provider::ChineseDateNeoSkeletonPatternsV1: ChineseDateNeoSkeletonPatternsV1,
            icu::datetime::provider::CopticDateNeoSkeletonPatternsV1: CopticDateNeoSkeletonPatternsV1,
            icu::datetime::provider::DangiDateNeoSkeletonPatternsV1: DangiDateNeoSkeletonPatternsV1,
            icu::datetime::provider::EthiopianDateNeoSkeletonPatternsV1: EthiopianDateNeoSkeletonPatternsV1,
            icu::datetime::provider::GregorianDateNeoSkeletonPatternsV1: GregorianDateNeoSkeletonPatternsV1,
            icu::datetime::provider::HebrewDateNeoSkeletonPatternsV1: HebrewDateNeoSkeletonPatternsV1,
            icu::datetime::provider::IndianDateNeoSkeletonPatternsV1: IndianDateNeoSkeletonPatternsV1,
            icu::datetime::provider::IslamicDateNeoSkeletonPatternsV1: IslamicDateNeoSkeletonPatternsV1,
            icu::datetime::provider::JapaneseDateNeoSkeletonPatternsV1: JapaneseDateNeoSkeletonPatternsV1,
            icu::datetime::provider::JapaneseExtendedDateNeoSkeletonPatternsV1: JapaneseExtendedDateNeoSkeletonPatternsV1,
            icu::datetime::provider::PersianDateNeoSkeletonPatternsV1: PersianDateNeoSkeletonPatternsV1,
            icu::datetime::provider::RocDateNeoSkeletonPatternsV1: RocDateNeoSkeletonPatternsV1,
            icu::datetime::provider::TimeNeoSkeletonPatternsV1: TimeNeoSkeletonPatternsV1,
            #[experimental]
            icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1: LongCompactDecimalFormatDataV1,
            icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1: ShortCompactDecimalFormatDataV1,
            icu::experimental::dimension::provider::currency_compact::ShortCurrencyCompactV1: ShortCurrencyCompactV1,
            icu::experimental::dimension::provider::currency_displayname::CurrencyDisplaynameV1: CurrencyDisplaynameV1,
            icu::experimental::dimension::provider::currency::CurrencyEssentialsV1: CurrencyEssentialsV1,
            icu::experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1: CurrencyPatternsDataV1,
            icu::experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1: CurrencyExtendedDataV1,
            icu::experimental::dimension::provider::percent::PercentEssentialsV1: PercentEssentialsV1,
            icu::experimental::dimension::provider::units::UnitsDisplayNameV1: UnitsDisplayNameV1,
            icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1: UnitsEssentialsV1,
            icu::experimental::duration::provider::DigitalDurationDataV1: DigitalDurationDataV1,
            icu::experimental::displaynames::provider::RegionDisplayNamesV1: RegionDisplayNamesV1,
            icu::experimental::displaynames::provider::LanguageDisplayNamesV1: LanguageDisplayNamesV1,
            icu::experimental::displaynames::provider::LocaleDisplayNamesV1: LocaleDisplayNamesV1,
            icu::experimental::displaynames::provider::ScriptDisplayNamesV1: ScriptDisplayNamesV1,
            icu::experimental::displaynames::provider::VariantDisplayNamesV1: VariantDisplayNamesV1,
            icu::experimental::measure::provider::trie::UnitsTrieV1: UnitsTrieV1,
            icu::experimental::relativetime::provider::LongSecondRelativeV1: LongSecondRelativeV1,
            icu::experimental::relativetime::provider::ShortSecondRelativeV1: ShortSecondRelativeV1,
            icu::experimental::relativetime::provider::NarrowSecondRelativeV1: NarrowSecondRelativeV1,
            icu::experimental::relativetime::provider::LongMinuteRelativeV1: LongMinuteRelativeV1,
            icu::experimental::relativetime::provider::ShortMinuteRelativeV1: ShortMinuteRelativeV1,
            icu::experimental::relativetime::provider::NarrowMinuteRelativeV1: NarrowMinuteRelativeV1,
            icu::experimental::relativetime::provider::LongHourRelativeV1: LongHourRelativeV1,
            icu::experimental::relativetime::provider::ShortHourRelativeV1: ShortHourRelativeV1,
            icu::experimental::relativetime::provider::NarrowHourRelativeV1: NarrowHourRelativeV1,
            icu::experimental::relativetime::provider::LongDayRelativeV1: LongDayRelativeV1,
            icu::experimental::relativetime::provider::ShortDayRelativeV1: ShortDayRelativeV1,
            icu::experimental::relativetime::provider::NarrowDayRelativeV1: NarrowDayRelativeV1,
            icu::experimental::relativetime::provider::LongWeekRelativeV1: LongWeekRelativeV1,
            icu::experimental::relativetime::provider::ShortWeekRelativeV1: ShortWeekRelativeV1,
            icu::experimental::relativetime::provider::NarrowWeekRelativeV1: NarrowWeekRelativeV1,
            icu::experimental::relativetime::provider::LongMonthRelativeV1: LongMonthRelativeV1,
            icu::experimental::relativetime::provider::ShortMonthRelativeV1: ShortMonthRelativeV1,
            icu::experimental::relativetime::provider::NarrowMonthRelativeV1: NarrowMonthRelativeV1,
            icu::experimental::relativetime::provider::LongQuarterRelativeV1: LongQuarterRelativeV1,
            icu::experimental::relativetime::provider::ShortQuarterRelativeV1: ShortQuarterRelativeV1,
            icu::experimental::relativetime::provider::NarrowQuarterRelativeV1: NarrowQuarterRelativeV1,
            icu::experimental::relativetime::provider::LongYearRelativeV1: LongYearRelativeV1,
            icu::experimental::relativetime::provider::ShortYearRelativeV1: ShortYearRelativeV1,
            icu::experimental::relativetime::provider::NarrowYearRelativeV1: NarrowYearRelativeV1,
            icu::experimental::personnames::provider::PersonNamesFormatV1: PersonNamesFormatV1,
            icu::experimental::transliterate::provider::TransliteratorRulesV1: TransliteratorRulesV1,
            icu::experimental::units::provider::UnitsInfoV1: UnitsInfoV1,
            icu::plurals::provider::PluralRangesV1: PluralRangesV1,
        );
    }
);

macro_rules! cb {
    ($($marker_ty:ty:$marker:ident,)+ #[experimental] $($emarker_ty:ty:$emarker:ident,)+) => {
        #[test]
        fn no_marker_collisions() {
            use icu_provider::prelude::*;

            let mut map = std::collections::BTreeMap::new();
            let mut failed = false;
            for marker in [
                $(
                    <$marker_ty>::INFO,
                )+
                $(
                    <$emarker_ty>::INFO,
                )+
            ] {
                if let Some(colliding_marker) = map.insert(marker.id.hashed(), marker) {
                    println!(
                        "{:?} and {:?} collide at {:?}",
                        marker.id,
                        colliding_marker.id,
                        marker.id.hashed(),
                    );
                    failed = true;
                }
            }
            if failed {
                panic!();
            }
        }
    }
}

registry!(cb);
