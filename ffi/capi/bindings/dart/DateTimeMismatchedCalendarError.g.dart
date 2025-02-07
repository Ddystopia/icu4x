// generated by diplomat-tool

part of 'lib.g.dart';

final class _DateTimeMismatchedCalendarErrorFfi extends ffi.Struct {
  @ffi.Int32()
  external int thisKind;
  external _ResultInt32Void dateKind;
}

/// See the [Rust documentation for `MismatchedCalendarError`](https://docs.rs/icu/latest/icu/datetime/struct.MismatchedCalendarError.html) for more information.
final class DateTimeMismatchedCalendarError {
  AnyCalendarKind thisKind;
  AnyCalendarKind? dateKind;

  DateTimeMismatchedCalendarError({required this.thisKind, this.dateKind});

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  DateTimeMismatchedCalendarError._fromFfi(_DateTimeMismatchedCalendarErrorFfi ffi) :
    thisKind = AnyCalendarKind.values[ffi.thisKind],
    dateKind = ffi.dateKind.isOk ? AnyCalendarKind.values[ffi.dateKind.union.ok] : null;

  // ignore: unused_element
  _DateTimeMismatchedCalendarErrorFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_DateTimeMismatchedCalendarErrorFfi>();
    struct.thisKind = thisKind.index;
    AnyCalendarKind? dateKind = this.dateKind;
    struct.dateKind = dateKind != null ? _ResultInt32Void.ok(dateKind.index) : _ResultInt32Void.err();
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is DateTimeMismatchedCalendarError &&
      other.thisKind == thisKind &&
      other.dateKind == dateKind;

  @override
  int get hashCode => Object.hashAll([
        thisKind,
        dateKind,
      ]);
}
