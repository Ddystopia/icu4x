// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
///
/// See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html) for more information.
final class LineSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  LineSegmenter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_LineSegmenter_destroy_mv1));

  /// Construct a [`LineSegmenter`] with default options (no locale-based tailoring) using compiled data. It automatically loads the best
  /// available payload data for Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
  factory LineSegmenter.auto() {
    final result = _icu4x_LineSegmenter_create_auto_mv1();
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with default options (no locale-based tailoring) and LSTM payload data for
  /// Burmese, Khmer, Lao, and Thai, using compiled data.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
  factory LineSegmenter.lstm() {
    final result = _icu4x_LineSegmenter_create_lstm_mv1();
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with default options (no locale-based tailoring) and dictionary payload data for
  /// Burmese, Khmer, Lao, and Thai, using compiled data
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
  factory LineSegmenter.dictionary() {
    final result = _icu4x_LineSegmenter_create_dictionary_mv1();
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with custom options using compiled data. It automatically loads the best
  /// available payload data for Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
  factory LineSegmenter.autoWithOptions(LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_auto_with_options_v2_mv1(contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with custom options. It automatically loads the best
  /// available payload data for Burmese, Khmer, Lao, and Thai, using a particular data source.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
  ///
  /// Throws [DataError] on failure.
  factory LineSegmenter.autoWithOptionsAndProvider(DataProvider provider, LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1(provider._ffi, contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return LineSegmenter._fromFfi(result.union.ok, []);
  }

  /// Construct a [`LineSegmenter`] with custom options and LSTM payload data for
  /// Burmese, Khmer, Lao, and Thai, using compiled data.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
  factory LineSegmenter.lstmWithOptions(LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_lstm_with_options_v2_mv1(contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with custom options and LSTM payload data for
  /// Burmese, Khmer, Lao, and Thai, using a particular data source.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
  ///
  /// Throws [DataError] on failure.
  factory LineSegmenter.lstmWithOptionsAndProvider(DataProvider provider, LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1(provider._ffi, contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return LineSegmenter._fromFfi(result.union.ok, []);
  }

  /// Construct a [`LineSegmenter`] with custom options and dictionary payload data for
  /// Burmese, Khmer, Lao, and Thai, using compiled data.
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
  factory LineSegmenter.dictionaryWithOptions(LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1(contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    return LineSegmenter._fromFfi(result, []);
  }

  /// Construct a [`LineSegmenter`] with custom options and dictionary payload data for
  /// Burmese, Khmer, Lao, and Thai, using a particular data source.
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
  ///
  /// Throws [DataError] on failure.
  factory LineSegmenter.dictionaryWithOptionsAndProvider(DataProvider provider, LineBreakOptions options, [Locale? contentLocale]) {
    final temp = _FinalizedArena();
    final result = _icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1(provider._ffi, contentLocale?._ffi ?? ffi.Pointer.fromAddress(0), options._toFfi(temp.arena));
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return LineSegmenter._fromFfi(result.union.ok, []);
  }

  /// Segments a string.
  ///
  /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_utf16) for more information.
  LineBreakIteratorUtf16 segment(String input) {
    final inputArena = _FinalizedArena();
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this, inputArena];
    final result = _icu4x_LineSegmenter_segment_utf16_mv1(_ffi, input._utf16AllocIn(inputArena.arena));
    return LineBreakIteratorUtf16._fromFfi(result, [], aEdges);
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_LineSegmenter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_auto_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_auto_mv1();

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_lstm_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_lstm_mv1();

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_dictionary_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_dictionary_mv1();

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_auto_with_options_v2_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_auto_with_options_v2_mv1(ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_LineSegmenter_create_auto_with_options_v2_and_provider_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_lstm_with_options_v2_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_lstm_with_options_v2_mv1(ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_LineSegmenter_create_lstm_with_options_v2_and_provider_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_create_dictionary_with_options_v2_mv1(ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _LineBreakOptionsFfi)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_LineSegmenter_create_dictionary_with_options_v2_and_provider_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> contentLocale, _LineBreakOptionsFfi options);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, _SliceUtf16)>(isLeaf: true, symbol: 'icu4x_LineSegmenter_segment_utf16_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_LineSegmenter_segment_utf16_mv1(ffi.Pointer<ffi.Opaque> self, _SliceUtf16 input);
