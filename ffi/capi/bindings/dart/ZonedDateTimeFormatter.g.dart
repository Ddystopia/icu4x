// generated by diplomat-tool

part of 'lib.g.dart';

/// An object capable of formatting a date time with time zone to a string.
///
/// See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMDT.html), [2](https://docs.rs/icu/latest/icu/datetime/fieldsets/zone/struct.GenericShort.html)
final class ZonedDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  ZonedDateTimeFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_ZonedDateTimeFormatter_destroy_mv1));

  /// Creates a new [`ZonedDateTimeFormatter`] from locale data using compiled data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses default options
  /// for the time zone.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [DateTimeFormatterLoadError] on failure.
  factory ZonedDateTimeFormatter.withLength(Locale locale, DateTimeLength length) {
    final result = _icu4x_ZonedDateTimeFormatter_create_with_length_mv1(locale._ffi, length.index);
    if (!result.isOk) {
      throw DateTimeFormatterLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return ZonedDateTimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`ZonedDateTimeFormatter`] from locale data using a particular data source.
  ///
  /// This function has `date_length` and `time_length` arguments and uses default options
  /// for the time zone.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [DateTimeFormatterLoadError] on failure.
  factory ZonedDateTimeFormatter.withLengthAndProvider(DataProvider provider, Locale locale, DateTimeLength length) {
    final result = _icu4x_ZonedDateTimeFormatter_create_with_length_and_provider_mv1(provider._ffi, locale._ffi, length.index);
    if (!result.isOk) {
      throw DateTimeFormatterLoadError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return ZonedDateTimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`Date`] a [`Time`], and a [`TimeZoneInfo`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String format(Date date, Time time, TimeZoneInfo zone) {
    final write = _Write();
    final result = _icu4x_ZonedDateTimeFormatter_format_mv1(_ffi, date._ffi, time._ffi, zone._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }

  /// Formats an [`IsoDate`] a [`Time`], and a [`TimeZoneInfo`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  ///
  /// Throws [DateTimeFormatError] on failure.
  String formatIso(IsoDate date, Time time, TimeZoneInfo zone) {
    final write = _Write();
    final result = _icu4x_ZonedDateTimeFormatter_format_iso_mv1(_ffi, date._ffi, time._ffi, zone._ffi, write._ffi);
    if (!result.isOk) {
      throw DateTimeFormatError.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return write.finalize();
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeFormatter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_ZonedDateTimeFormatter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeFormatter_create_with_length_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_ZonedDateTimeFormatter_create_with_length_mv1(ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeFormatter_create_with_length_and_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_ZonedDateTimeFormatter_create_with_length_and_provider_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, int length);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeFormatter_format_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_ZonedDateTimeFormatter_format_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> date, ffi.Pointer<ffi.Opaque> time, ffi.Pointer<ffi.Opaque> zone, ffi.Pointer<ffi.Opaque> write);

@meta.RecordUse()
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeFormatter_format_iso_mv1')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _icu4x_ZonedDateTimeFormatter_format_iso_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> date, ffi.Pointer<ffi.Opaque> time, ffi.Pointer<ffi.Opaque> zone, ffi.Pointer<ffi.Opaque> write);
