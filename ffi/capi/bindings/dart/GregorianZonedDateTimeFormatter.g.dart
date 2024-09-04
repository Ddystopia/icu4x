// generated by diplomat-tool

part of 'lib.g.dart';

/// An object capable of formatting a date time with time zone to a string.
///
/// See the [Rust documentation for `TypedZonedDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html) for more information.
final class GregorianZonedDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  GregorianZonedDateTimeFormatter._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_GregorianZonedDateTimeFormatter_destroy_mv1));

  /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses default options
  /// for the time zone.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory GregorianZonedDateTimeFormatter.withLengths(DataProvider provider, Locale locale, DateLength? dateLength, TimeLength? timeLength) {
    final result = _icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_mv1(provider._ffi, locale._ffi, dateLength != null ? _ResultInt32Void.ok(dateLength.index) : _ResultInt32Void.err(), timeLength != null ? _ResultInt32Void.ok(timeLength.index) : _ResultInt32Void.err());
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return GregorianZonedDateTimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`GregorianZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
  /// fallback for the time zone with the given configurations.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new) for more information.
  ///
  /// Throws [Error] on failure.
  factory GregorianZonedDateTimeFormatter.withLengthsAndIso8601TimeZoneFallback(DataProvider provider, Locale locale, DateLength? dateLength, TimeLength? timeLength, IsoTimeZoneOptions zoneOptions) {
    final temp = _FinalizedArena();
    final result = _icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_mv1(provider._ffi, locale._ffi, dateLength != null ? _ResultInt32Void.ok(dateLength.index) : _ResultInt32Void.err(), timeLength != null ? _ResultInt32Void.ok(timeLength.index) : _ResultInt32Void.err(), zoneOptions._toFfi(temp.arena));
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._ffi == result.union.err);
    }
    return GregorianZonedDateTimeFormatter._fromFfi(result.union.ok, []);
  }

  /// Formats a [`IsoDateTime`] and [`CustomTimeZone`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.format) for more information.
  String formatIsoDatetimeWithCustomTimeZone(IsoDateTime datetime, CustomTimeZone timeZone) {
    final write = _Write();
    _icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1(_ffi, datetime._ffi, timeZone._ffi, write._ffi);
    return write.finalize();
  }
}

@meta.ResourceIdentifier('icu4x_GregorianZonedDateTimeFormatter_destroy_mv1')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_GregorianZonedDateTimeFormatter_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_GregorianZonedDateTimeFormatter_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_mv1')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _ResultInt32Void, _ResultInt32Void)>(isLeaf: true, symbol: 'icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, _ResultInt32Void dateLength, _ResultInt32Void timeLength);

@meta.ResourceIdentifier('icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_mv1')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, _ResultInt32Void, _ResultInt32Void, _IsoTimeZoneOptionsFfi)>(isLeaf: true, symbol: 'icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_GregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback_mv1(ffi.Pointer<ffi.Opaque> provider, ffi.Pointer<ffi.Opaque> locale, _ResultInt32Void dateLength, _ResultInt32Void timeLength, _IsoTimeZoneOptionsFfi zoneOptions);

@meta.ResourceIdentifier('icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_GregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> datetime, ffi.Pointer<ffi.Opaque> timeZone, ffi.Pointer<ffi.Opaque> write);
