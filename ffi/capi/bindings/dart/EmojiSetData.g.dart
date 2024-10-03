// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `EmojiSetData`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetData.html) for more information.
///
/// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetData.html#method.new) for more information.
///
/// See the [Rust documentation for `EmojiSetDataBorrowed`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetDataBorrowed.html) for more information.
final class EmojiSetData implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  EmojiSetData._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_EmojiSetData_destroy_mv1));

  /// Checks whether the string is in the set.
  ///
  /// See the [Rust documentation for `contains_str`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetDataBorrowed.html#method.contains_str) for more information.
  bool containsStr(String s) {
    final temp = _FinalizedArena();
    final result = _icu4x_EmojiSetData_contains_str_mv1(_ffi, s._utf8AllocIn(temp.arena));
    return result;
  }

  /// Checks whether the code point is in the set.
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/struct.EmojiSetDataBorrowed.html#method.contains) for more information.
  bool contains(Rune cp) {
    final result = _icu4x_EmojiSetData_contains_mv1(_ffi, cp);
    return result;
  }

  /// See the [Rust documentation for `BasicEmoji`](https://docs.rs/icu/latest/icu/properties/props/struct.BasicEmoji.html) for more information.
  ///
  /// Throws [DataError] on failure.
  factory EmojiSetData.basic(DataProvider provider) {
    final result = _icu4x_EmojiSetData_load_basic_mv1(provider._ffi);
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return EmojiSetData._fromFfi(result.union.ok, []);
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_EmojiSetData_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_EmojiSetData_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, _SliceUtf8)>(isLeaf: true, symbol: 'icu4x_EmojiSetData_contains_str_mv1')
// ignore: non_constant_identifier_names
external bool _icu4x_EmojiSetData_contains_str_mv1(ffi.Pointer<ffi.Opaque> self, _SliceUtf8 s);

@meta.RecordUse()
@ffi.Native<ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32)>(isLeaf: true, symbol: 'icu4x_EmojiSetData_contains_mv1')
// ignore: non_constant_identifier_names
external bool _icu4x_EmojiSetData_contains_mv1(ffi.Pointer<ffi.Opaque> self, Rune cp);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_EmojiSetData_load_basic_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_EmojiSetData_load_basic_mv1(ffi.Pointer<ffi.Opaque> provider);