// generated by diplomat-tool

part of 'lib.g.dart';

/// An ICU4X Units Converter Factory object, capable of creating converters a [`UnitsConverter`]
/// for converting between two [`MeasureUnit`]s.
///
/// Also, it can parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
///
/// See the [Rust documentation for `ConverterFactory`](https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html) for more information.
final class UnitsConverterFactory implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  UnitsConverterFactory._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_icu4x_UnitsConverterFactory_destroy_mv1));

  /// Construct a new [`UnitsConverterFactory`] instance.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.new) for more information.
  ///
  /// Throws [DataError] on failure.
  factory UnitsConverterFactory(DataProvider provider) {
    final result = _icu4x_UnitsConverterFactory_create_mv1(provider._ffi);
    if (!result.isOk) {
      throw DataError.values[result.union.err];
    }
    return UnitsConverterFactory._fromFfi(result.union.ok, []);
  }

  /// Creates a new [`UnitsConverter`] from the input and output [`MeasureUnit`]s.
  /// Returns nothing if the conversion between the two units is not possible.
  /// For example, conversion between `meter` and `second` is not possible.
  ///
  /// See the [Rust documentation for `converter`](https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.converter) for more information.
  UnitsConverter? converter(MeasureUnit from, MeasureUnit to) {
    final result = _icu4x_UnitsConverterFactory_converter_mv1(_ffi, from._ffi, to._ffi);
    return result.address == 0 ? null : UnitsConverter._fromFfi(result, []);
  }

  /// Creates a parser to parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
  ///
  /// See the [Rust documentation for `parser`](https://docs.rs/icu/latest/icu/experimental/units/converter_factory/struct.ConverterFactory.html#method.parser) for more information.
  MeasureUnitParser parser() {
    // This lifetime edge depends on lifetimes: 'a
    core.List<Object> aEdges = [this];
    final result = _icu4x_UnitsConverterFactory_parser_mv1(_ffi);
    return MeasureUnitParser._fromFfi(result, [], aEdges);
  }
}

@meta.RecordUse()
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'icu4x_UnitsConverterFactory_destroy_mv1')
// ignore: non_constant_identifier_names
external void _icu4x_UnitsConverterFactory_destroy_mv1(ffi.Pointer<ffi.Void> self);

@meta.RecordUse()
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_UnitsConverterFactory_create_mv1')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _icu4x_UnitsConverterFactory_create_mv1(ffi.Pointer<ffi.Opaque> provider);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_UnitsConverterFactory_converter_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_UnitsConverterFactory_converter_mv1(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> from, ffi.Pointer<ffi.Opaque> to);

@meta.RecordUse()
@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'icu4x_UnitsConverterFactory_parser_mv1')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _icu4x_UnitsConverterFactory_parser_mv1(ffi.Pointer<ffi.Opaque> self);
