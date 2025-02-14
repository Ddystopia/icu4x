// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
///
/// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/time/struct.Time.html) for more information.
final class Time implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  Time._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_Time_destroy_mv1));

  /// Creates a new [`Time`] given field values
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/time/struct.Time.html#method.try_new) for more information.
  ///
  /// Throws [CalendarError] on failure.
  factory Time(int hour, int minute, int second, int subsecond) {
    final result = _icu4x_Time_create_mv1(hour, minute, second, subsecond);
    if (!result.isOk) {
      throw CalendarError.values[result.union.err];
    }
    return Time._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`Time`] from an IXDTF string.
  ///
  /// See the [Rust documentation for `try_from_str`](https://docs.rs/icu/latest/icu/time/struct.Time.html#method.try_from_str) for more information.
  ///
  /// Throws [CalendarParseError] on failure.
  factory Time.fromString(String v) {
    final temp = _FinalizedArena();
    final result = _icu4x_Time_from_string_mv1(v._utf8AllocIn(temp.arena));
    if (!result.isOk) {
      throw CalendarParseError.values[result.union.err];
    }
    return Time._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`Time`] representing midnight (00:00.000).
  ///
  /// See the [Rust documentation for `midnight`](https://docs.rs/icu/latest/icu/time/struct.Time.html#method.midnight) for more information.
  ///
  /// Throws [CalendarError] on failure.
  factory Time.midnight() {
    final result = _icu4x_Time_midnight_mv1();
    if (!result.isOk) {
      throw CalendarError.values[result.union.err];
    }
    return Time._fromFfi(result.union.ok, []);
  }

  /// Returns the hour in this time
  ///
  /// See the [Rust documentation for `hour`](https://docs.rs/icu/latest/icu/time/struct.Time.html#structfield.hour) for more information.
  int get hour {
    final result = _icu4x_Time_hour_mv1(_ffi);
    return result;
  }

  /// Returns the minute in this time
  ///
  /// See the [Rust documentation for `minute`](https://docs.rs/icu/latest/icu/time/struct.Time.html#structfield.minute) for more information.
  int get minute {
    final result = _icu4x_Time_minute_mv1(_ffi);
    return result;
  }

  /// Returns the second in this time
  ///
  /// See the [Rust documentation for `second`](https://docs.rs/icu/latest/icu/time/struct.Time.html#structfield.second) for more information.
  int get second {
    final result = _icu4x_Time_second_mv1(_ffi);
    return result;
  }

  /// Returns the subsecond in this time as nanoseconds
  ///
  /// See the [Rust documentation for `subsecond`](https://docs.rs/icu/latest/icu/time/struct.Time.html#structfield.subsecond) for more information.
  int get subsecond {
    final result = _icu4x_Time_subsecond_mv1(_ffi);
    return result;
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_Time_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_Time_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Uint8, ffi.Uint8, ffi.Uint8, ffi.Uint32)>(isLeaf: true, symbol: 'icu4x_Time_create_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_Time_create_mv1(int hour, int minute, int second, int subsecond);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(_SliceUtf8)>(isLeaf: true, symbol: 'icu4x_Time_from_string_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_Time_from_string_mv1(_SliceUtf8 v);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'icu4x_Time_midnight_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_Time_midnight_mv1();

@meta.RecordUse()
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_Time_hour_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_Time_hour_mv1(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_Time_minute_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_Time_minute_mv1(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_Time_second_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_Time_second_mv1(ffi.Pointer<ffi.Opaque> self);

@meta.RecordUse()
@ffi.Native<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_Time_subsecond_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_Time_subsecond_mv1(ffi.Pointer<ffi.Opaque> self);
