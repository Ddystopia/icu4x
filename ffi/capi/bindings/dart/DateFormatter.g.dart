// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMD.html)
final class DateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  DateFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_DateFormatter_destroy_mv1));

  /// Creates a new [`DateFormatter`] using compiled data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [DateTimeFormatterLoadError] on failure.
  factory DateFormatter.withLength(Locale locale, DateTimeLength length) {
    final result = _icu4x_DateFormatter_create_with_length_mv1(locale._ffi, length.index);
    if (!result.isOk) {
      throw DateTimeFormatterLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DateFormatter._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`DateFormatter`] using a particular data source.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [DateTimeFormatterLoadError] on failure.
  factory DateFormatter.withLengthAndProvider(DataProvider provider, Locale locale, DateTimeLength length) {
    final result = _icu4x_DateFormatter_create_with_length_and_provider_mv1(provider._ffi, locale._ffi, length.index);
    if (!result.isOk) {
      throw DateTimeFormatterLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return DateFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`Date`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String format(Date value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Formats a [`IsoDate`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatIso(IsoDate value) {
    final write = _Write();
    final result = _icu4x_DateFormatter_format_iso_mv1(_ffi, value._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Returns the calendar system used in this formatter.
  ///
  /// See the [Rust documentation for `calendar`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.calendar) for more information.
  Calendar calendar() {
    final result = _icu4x_DateFormatter_calendar_mv1(_ffi);
    return Calendar._fromFfi(result, []);
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_DateFormatter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_DateFormatter_create_with_length_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_DateFormatter_create_with_length_mv1(ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_DateFormatter_create_with_length_and_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_DateFormatter_create_with_length_and_provider_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_format_iso_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_DateFormatter_format_iso_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> value, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_DateFormatter_calendar_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_DateFormatter_calendar_mv1(ffi.Pointer<ffi.Opaque> self);
