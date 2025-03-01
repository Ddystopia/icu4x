// generated by diplomat-tool

part of 'lib.g.dart';

/// See the [Rust documentation for `ZonedDateTimeParser`](https://docs.rs/icu/latest/icu/time/struct.ZonedDateTimeParser.html) for more information.
final class ZonedDateTimeParser implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  ZonedDateTimeParser._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_ZonedDateTimeParser_destroy_mv1));

  /// Construct a new [`ZonedDateTimeParser`] instance using compiled data.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/time/struct.ZonedDateTimeParser.html#method.new) for more information.
  factory ZonedDateTimeParser() {
    final result = _icu4x_ZonedDateTimeParser_create_mv1();
    return ZonedDateTimeParser._fromFfi(result, []);
  }

  /// Construct a new [`ZonedDateTimeParser`] instance using a particular data source.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/time/struct.ZonedDateTimeParser.html#method.new) for more information.
  ///
  /// Throws [DataError] on failure.
  factory ZonedDateTimeParser.withProvider(DataProvider provider) {
    final result = _icu4x_ZonedDateTimeParser_create_with_provider_mv1(provider._ffi);
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return ZonedDateTimeParser._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`ZonedIsoDateTime`] from an IXDTF string.
  ///
  /// See the [Rust documentation for `try_from_str`](https://docs.rs/icu/latest/icu/time/struct.ZonedDateTime.html#method.try_from_str) for more information.
  ///
  /// Throws [CalendarParseError] on failure.
  ZonedIsoDateTime tryIsoFromStr(String v) {
    final temp = _FinalizedArena();
    final result = _icu4x_ZonedDateTimeParser_try_iso_from_str_mv1(_ffi, v._utf8AllocIn(temp.arena));
    if (!result.isOk) {
      throw CalendarParseError.values[result.union.err];
    }
    return ZonedIsoDateTime._fromFfi(result.union.ok);
  }

  /// Creates a new [`ZonedDateTime`] from an IXDTF string.
  ///
  /// See the [Rust documentation for `try_from_str`](https://docs.rs/icu/latest/icu/time/struct.DateTime.html#method.try_from_str) for more information.
  ///
  /// Throws [CalendarParseError] on failure.
  ZonedDateTime tryFromStr(String v, Calendar calendar) {
    final temp = _FinalizedArena();
    final result = _icu4x_ZonedDateTimeParser_try_from_str_mv1(_ffi, v._utf8AllocIn(temp.arena), calendar._ffi);
    if (!result.isOk) {
      throw CalendarParseError.values[result.union.err];
    }
    return ZonedDateTime._fromFfi(result.union.ok);
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeParser_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_ZonedDateTimeParser_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeParser_create_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_ZonedDateTimeParser_create_mv1();

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeParser_create_with_provider_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_ZonedDateTimeParser_create_with_provider_mv1(ffi.Pointer<ffi.Opaque> provider);

@meta.RecordUse()
@ffi.Native<_ResultZonedIsoDateTimeFfiInt32 Function(ffi.Pointer<ffi.Opaque>, _SliceUtf8)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeParser_try_iso_from_str_mv1')
// ignore: non_constant_identifier_names
external _ResultZonedIsoDateTimeFfiInt32 _icu4x_ZonedDateTimeParser_try_iso_from_str_mv1(ffi.Pointer<ffi.Opaque> self, _SliceUtf8 v);

@meta.RecordUse()
@ffi.Native<_ResultZonedDateTimeFfiInt32 Function(ffi.Pointer<ffi.Opaque>, _SliceUtf8, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_ZonedDateTimeParser_try_from_str_mv1')
// ignore: non_constant_identifier_names
external _ResultZonedDateTimeFfiInt32 _icu4x_ZonedDateTimeParser_try_from_str_mv1(ffi.Pointer<ffi.Opaque> self, _SliceUtf8 v, ffi.Pointer<ffi.Opaque> calendar);
