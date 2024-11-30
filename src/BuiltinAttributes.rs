// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::AffineMap::*;
use crate::AffineExpr::*;
use crate::IR::*;
use crate::Support::*;
use crate::IR::*;
use crate::IR::*;
use crate::IntegerSet::*;
use crate::AffineExpr::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirAttributeGetNull() -> MlirAttribute;

  pub fn mlirAttributeIsALocation(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsAAffineMap(attr: MlirAttribute) -> u8;

  pub fn mlirAffineMapAttrGet(map: MlirAffineMap) -> MlirAttribute;

  pub fn mlirAffineMapAttrGetValue(attr: MlirAttribute) -> MlirAffineMap;

  pub fn mlirAffineMapAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAArray(attr: MlirAttribute) -> u8;

  pub fn mlirArrayAttrGet(ctx: MlirContext, numElements: std::ffi::c_long, elements: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirArrayAttrGetNumElements(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirArrayAttrGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> MlirAttribute;

  pub fn mlirArrayAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsADictionary(attr: MlirAttribute) -> u8;

  pub fn mlirDictionaryAttrGet(ctx: MlirContext, numElements: std::ffi::c_long, elements: *const MlirNamedAttribute) -> MlirAttribute;

  pub fn mlirDictionaryAttrGetNumElements(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirDictionaryAttrGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> MlirNamedAttribute;

  pub fn mlirDictionaryAttrGetElementByName(attr: MlirAttribute, name: MlirStringRef) -> MlirAttribute;

  pub fn mlirDictionaryAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAFloat(attr: MlirAttribute) -> u8;

  pub fn mlirFloatAttrDoubleGet(ctx: MlirContext, r#type: MlirType, value: f64) -> MlirAttribute;

  pub fn mlirFloatAttrDoubleGetChecked(loc: MlirLocation, r#type: MlirType, value: f64) -> MlirAttribute;

  pub fn mlirFloatAttrGetValueDouble(attr: MlirAttribute) -> f64;

  pub fn mlirFloatAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAInteger(attr: MlirAttribute) -> u8;

  pub fn mlirIntegerAttrGet(r#type: MlirType, value: std::ffi::c_long) -> MlirAttribute;

  pub fn mlirIntegerAttrGetValueInt(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirIntegerAttrGetValueSInt(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirIntegerAttrGetValueUInt(attr: MlirAttribute) -> std::ffi::c_ulong;

  pub fn mlirIntegerAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsABool(attr: MlirAttribute) -> u8;

  pub fn mlirBoolAttrGet(ctx: MlirContext, value: std::ffi::c_int) -> MlirAttribute;

  pub fn mlirBoolAttrGetValue(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsAIntegerSet(attr: MlirAttribute) -> u8;

  pub fn mlirIntegerSetAttrGet(set: MlirIntegerSet) -> MlirAttribute;

  pub fn mlirIntegerSetAttrGetValue(attr: MlirAttribute) -> MlirIntegerSet;

  pub fn mlirIntegerSetAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAOpaque(attr: MlirAttribute) -> u8;

  pub fn mlirOpaqueAttrGet(ctx: MlirContext, dialectNamespace: MlirStringRef, dataLength: std::ffi::c_long, data: *const std::ffi::c_char, r#type: MlirType) -> MlirAttribute;

  pub fn mlirOpaqueAttrGetDialectNamespace(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirOpaqueAttrGetData(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirOpaqueAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAString(attr: MlirAttribute) -> u8;

  pub fn mlirStringAttrGet(ctx: MlirContext, str: MlirStringRef) -> MlirAttribute;

  pub fn mlirStringAttrTypedGet(r#type: MlirType, str: MlirStringRef) -> MlirAttribute;

  pub fn mlirStringAttrGetValue(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirStringAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsASymbolRef(attr: MlirAttribute) -> u8;

  pub fn mlirSymbolRefAttrGet(ctx: MlirContext, symbol: MlirStringRef, numReferences: std::ffi::c_long, references: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirSymbolRefAttrGetRootReference(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirSymbolRefAttrGetLeafReference(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirSymbolRefAttrGetNumNestedReferences(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirSymbolRefAttrGetNestedReference(attr: MlirAttribute, pos: std::ffi::c_long) -> MlirAttribute;

  pub fn mlirSymbolRefAttrGetTypeID() -> MlirTypeID;

  pub fn mlirDisctinctAttrCreate(referencedAttr: MlirAttribute) -> MlirAttribute;

  pub fn mlirAttributeIsAFlatSymbolRef(attr: MlirAttribute) -> u8;

  pub fn mlirFlatSymbolRefAttrGet(ctx: MlirContext, symbol: MlirStringRef) -> MlirAttribute;

  pub fn mlirFlatSymbolRefAttrGetValue(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirAttributeIsAType(attr: MlirAttribute) -> u8;

  pub fn mlirTypeAttrGet(r#type: MlirType) -> MlirAttribute;

  pub fn mlirTypeAttrGetValue(attr: MlirAttribute) -> MlirType;

  pub fn mlirTypeAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAUnit(attr: MlirAttribute) -> u8;

  pub fn mlirUnitAttrGet(ctx: MlirContext) -> MlirAttribute;

  pub fn mlirUnitAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAElements(attr: MlirAttribute) -> u8;

  pub fn mlirElementsAttrGetValue(attr: MlirAttribute, rank: std::ffi::c_long, idxs: *mut std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirElementsAttrIsValidIndex(attr: MlirAttribute, rank: std::ffi::c_long, idxs: *mut std::ffi::c_ulong) -> u8;

  pub fn mlirElementsAttrGetNumElements(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirDenseArrayAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsADenseBoolArray(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseI8Array(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseI16Array(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseI32Array(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseI64Array(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseF32Array(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseF64Array(attr: MlirAttribute) -> u8;

  pub fn mlirDenseBoolArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirDenseI8ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const std::ffi::c_schar) -> MlirAttribute;

  pub fn mlirDenseI16ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const std::ffi::c_short) -> MlirAttribute;

  pub fn mlirDenseI32ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirDenseI64ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const std::ffi::c_long) -> MlirAttribute;

  pub fn mlirDenseF32ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const f32) -> MlirAttribute;

  pub fn mlirDenseF64ArrayGet(ctx: MlirContext, size: std::ffi::c_long, values: *const f64) -> MlirAttribute;

  pub fn mlirDenseArrayGetNumElements(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirDenseBoolArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> u8;

  pub fn mlirDenseI8ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_schar;

  pub fn mlirDenseI16ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_short;

  pub fn mlirDenseI32ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_int;

  pub fn mlirDenseI64ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirDenseF32ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> f32;

  pub fn mlirDenseF64ArrayGetElement(attr: MlirAttribute, pos: std::ffi::c_long) -> f64;

  pub fn mlirAttributeIsADenseElements(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseIntElements(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeIsADenseFPElements(attr: MlirAttribute) -> u8;

  pub fn mlirDenseIntOrFPElementsAttrGetTypeID() -> MlirTypeID;

  pub fn mlirDenseElementsAttrGet(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirDenseElementsAttrRawBufferGet(shapedType: MlirType, rawBufferSize: std::ffi::c_ulong, rawBuffer: *const std::ffi::c_void) -> MlirAttribute;

  pub fn mlirDenseElementsAttrSplatGet(shapedType: MlirType, element: MlirAttribute) -> MlirAttribute;

  pub fn mlirDenseElementsAttrBoolSplatGet(shapedType: MlirType, element: u8) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt8SplatGet(shapedType: MlirType, element: std::ffi::c_uchar) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt8SplatGet(shapedType: MlirType, element: std::ffi::c_schar) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt32SplatGet(shapedType: MlirType, element: std::ffi::c_uint) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt32SplatGet(shapedType: MlirType, element: std::ffi::c_int) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt64SplatGet(shapedType: MlirType, element: std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt64SplatGet(shapedType: MlirType, element: std::ffi::c_long) -> MlirAttribute;

  pub fn mlirDenseElementsAttrFloatSplatGet(shapedType: MlirType, element: f32) -> MlirAttribute;

  pub fn mlirDenseElementsAttrDoubleSplatGet(shapedType: MlirType, element: f64) -> MlirAttribute;

  pub fn mlirDenseElementsAttrBoolGet(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt8Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_uchar) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt8Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_schar) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt16Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_ushort) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt16Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_short) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt32Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_uint) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt32Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirDenseElementsAttrUInt64Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirDenseElementsAttrInt64Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_long) -> MlirAttribute;

  pub fn mlirDenseElementsAttrFloatGet(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const f32) -> MlirAttribute;

  pub fn mlirDenseElementsAttrDoubleGet(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const f64) -> MlirAttribute;

  pub fn mlirDenseElementsAttrBFloat16Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_ushort) -> MlirAttribute;

  pub fn mlirDenseElementsAttrFloat16Get(shapedType: MlirType, numElements: std::ffi::c_long, elements: *const std::ffi::c_ushort) -> MlirAttribute;

  pub fn mlirDenseElementsAttrStringGet(shapedType: MlirType, numElements: std::ffi::c_long, strs: *mut MlirStringRef) -> MlirAttribute;

  pub fn mlirDenseElementsAttrReshapeGet(attr: MlirAttribute, shapedType: MlirType) -> MlirAttribute;

  pub fn mlirDenseElementsAttrIsSplat(attr: MlirAttribute) -> u8;

  pub fn mlirDenseElementsAttrGetSplatValue(attr: MlirAttribute) -> MlirAttribute;

  pub fn mlirDenseElementsAttrGetBoolSplatValue(attr: MlirAttribute) -> std::ffi::c_int;

  pub fn mlirDenseElementsAttrGetInt8SplatValue(attr: MlirAttribute) -> std::ffi::c_schar;

  pub fn mlirDenseElementsAttrGetUInt8SplatValue(attr: MlirAttribute) -> std::ffi::c_uchar;

  pub fn mlirDenseElementsAttrGetInt32SplatValue(attr: MlirAttribute) -> std::ffi::c_int;

  pub fn mlirDenseElementsAttrGetUInt32SplatValue(attr: MlirAttribute) -> std::ffi::c_uint;

  pub fn mlirDenseElementsAttrGetInt64SplatValue(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirDenseElementsAttrGetUInt64SplatValue(attr: MlirAttribute) -> std::ffi::c_ulong;

  pub fn mlirDenseElementsAttrGetFloatSplatValue(attr: MlirAttribute) -> f32;

  pub fn mlirDenseElementsAttrGetDoubleSplatValue(attr: MlirAttribute) -> f64;

  pub fn mlirDenseElementsAttrGetStringSplatValue(attr: MlirAttribute) -> MlirStringRef;

  pub fn mlirDenseElementsAttrGetBoolValue(attr: MlirAttribute, pos: std::ffi::c_long) -> u8;

  pub fn mlirDenseElementsAttrGetInt8Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_schar;

  pub fn mlirDenseElementsAttrGetUInt8Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_uchar;

  pub fn mlirDenseElementsAttrGetInt16Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_short;

  pub fn mlirDenseElementsAttrGetUInt16Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_ushort;

  pub fn mlirDenseElementsAttrGetInt32Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_int;

  pub fn mlirDenseElementsAttrGetUInt32Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_uint;

  pub fn mlirDenseElementsAttrGetInt64Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirDenseElementsAttrGetUInt64Value(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_ulong;

  pub fn mlirDenseElementsAttrGetFloatValue(attr: MlirAttribute, pos: std::ffi::c_long) -> f32;

  pub fn mlirDenseElementsAttrGetDoubleValue(attr: MlirAttribute, pos: std::ffi::c_long) -> f64;

  pub fn mlirDenseElementsAttrGetStringValue(attr: MlirAttribute, pos: std::ffi::c_long) -> MlirStringRef;

  pub fn mlirDenseElementsAttrGetRawData(attr: MlirAttribute) -> *const std::ffi::c_void;

  pub fn mlirAttributeIsADenseResourceElements(attr: MlirAttribute) -> u8;

  pub fn mlirUnmanagedDenseResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, data: *mut std::ffi::c_void, dataLength: std::ffi::c_ulong, dataAlignment: std::ffi::c_ulong, dataIsMutable: u8, deleter: *mut extern fn (*mut std::ffi::c_void, *const std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_ulong) -> (), userData: *mut std::ffi::c_void) -> MlirAttribute;

  pub fn mlirUnmanagedDenseBoolResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirUnmanagedDenseUInt8ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_uchar) -> MlirAttribute;

  pub fn mlirUnmanagedDenseInt8ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_schar) -> MlirAttribute;

  pub fn mlirUnmanagedDenseUInt16ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_ushort) -> MlirAttribute;

  pub fn mlirUnmanagedDenseInt16ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_short) -> MlirAttribute;

  pub fn mlirUnmanagedDenseUInt32ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_uint) -> MlirAttribute;

  pub fn mlirUnmanagedDenseInt32ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_int) -> MlirAttribute;

  pub fn mlirUnmanagedDenseUInt64ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirUnmanagedDenseInt64ResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const std::ffi::c_long) -> MlirAttribute;

  pub fn mlirUnmanagedDenseFloatResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const f32) -> MlirAttribute;

  pub fn mlirUnmanagedDenseDoubleResourceElementsAttrGet(shapedType: MlirType, name: MlirStringRef, numElements: std::ffi::c_long, elements: *const f64) -> MlirAttribute;

  pub fn mlirDenseBoolResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> u8;

  pub fn mlirDenseInt8ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_schar;

  pub fn mlirDenseUInt8ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_uchar;

  pub fn mlirDenseInt16ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_short;

  pub fn mlirDenseUInt16ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_ushort;

  pub fn mlirDenseInt32ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_int;

  pub fn mlirDenseUInt32ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_uint;

  pub fn mlirDenseInt64ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirDenseUInt64ResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_ulong;

  pub fn mlirDenseFloatResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> f32;

  pub fn mlirDenseDoubleResourceElementsAttrGetValue(attr: MlirAttribute, pos: std::ffi::c_long) -> f64;

  pub fn mlirAttributeIsASparseElements(attr: MlirAttribute) -> u8;

  pub fn mlirSparseElementsAttribute(shapedType: MlirType, denseIndices: MlirAttribute, denseValues: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseElementsAttrGetIndices(attr: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseElementsAttrGetValues(attr: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseElementsAttrGetTypeID() -> MlirTypeID;

  pub fn mlirAttributeIsAStridedLayout(attr: MlirAttribute) -> u8;

  pub fn mlirStridedLayoutAttrGet(ctx: MlirContext, offset: std::ffi::c_long, numStrides: std::ffi::c_long, strides: *const std::ffi::c_long) -> MlirAttribute;

  pub fn mlirStridedLayoutAttrGetOffset(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirStridedLayoutAttrGetNumStrides(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirStridedLayoutAttrGetStride(attr: MlirAttribute, pos: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirStridedLayoutAttrGetTypeID() -> MlirTypeID;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirAttributeGetNull()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeGetNull()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsALocation<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsALocation(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAAffineMap<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAAffineMap(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirAffineMapAttrGet<T0_>(map_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAffineMapAttrGet(Into::<MlirAffineMap>::into(map_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAffineMapAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirAffineMapAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAffineMapAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAArray<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAArray(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirArrayAttrGet<T0_, T1_, T2_>(ctx_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirArrayAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const MlirAttribute>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirArrayAttrGetNumElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirArrayAttrGetNumElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirArrayAttrGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirArrayAttrGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirArrayAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirArrayAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADictionary<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADictionary(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDictionaryAttrGet<T0_, T1_, T2_>(ctx_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirNamedAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDictionaryAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const MlirNamedAttribute>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDictionaryAttrGetNumElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDictionaryAttrGetNumElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirNamedAttribute> {
  pub unsafe fn mlirDictionaryAttrGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDictionaryAttrGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDictionaryAttrGetElementByName<T0_, T1_>(attr_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDictionaryAttrGetElementByName(Into::<MlirAttribute>::into(attr_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirDictionaryAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDictionaryAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAFloat<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAFloat(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirFloatAttrDoubleGet<T0_, T1_, T2_>(ctx_:  T0_, type_:  T1_, value_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirType>,  T2_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFloatAttrDoubleGet(Into::<MlirContext>::into(ctx_), Into::<MlirType>::into(type_), Into::<f64>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirFloatAttrDoubleGetChecked<T0_, T1_, T2_>(loc_:  T0_, type_:  T1_, value_:  T2_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirType>,  T2_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFloatAttrDoubleGetChecked(Into::<MlirLocation>::into(loc_), Into::<MlirType>::into(type_), Into::<f64>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirFloatAttrGetValueDouble<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFloatAttrGetValueDouble(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloatAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFloatAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAInteger<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAInteger(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirIntegerAttrGet<T0_, T1_>(type_:  T0_, value_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerAttrGet(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerAttrGetValueInt<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerAttrGetValueInt(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerAttrGetValueSInt<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerAttrGetValueSInt(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirIntegerAttrGetValueUInt<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerAttrGetValueUInt(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirIntegerAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsABool<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsABool(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirBoolAttrGet<T0_, T1_>(ctx_:  T0_, value_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirBoolAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_int>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirBoolAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirBoolAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAIntegerSet<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAIntegerSet(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirIntegerSetAttrGet<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerSetAttrGet(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIntegerSet> {
  pub unsafe fn mlirIntegerSetAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerSetAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirIntegerSetAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirIntegerSetAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAOpaque<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAOpaque(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirOpaqueAttrGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, dialectNamespace_:  T1_, dataLength_:  T2_, data_:  T3_, type_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_char>,  T4_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirOpaqueAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(dialectNamespace_), Into::<std::ffi::c_long>::into(dataLength_), Into::<*const std::ffi::c_char>::into(data_), Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirOpaqueAttrGetDialectNamespace<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirOpaqueAttrGetDialectNamespace(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirOpaqueAttrGetData<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirOpaqueAttrGetData(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirOpaqueAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirOpaqueAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAString<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAString(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirStringAttrGet<T0_, T1_>(ctx_:  T0_, str_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStringAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(str_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirStringAttrTypedGet<T0_, T1_>(type_:  T0_, str_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStringAttrTypedGet(Into::<MlirType>::into(type_), Into::<MlirStringRef>::into(str_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirStringAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStringAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirStringAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStringAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsASymbolRef<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsASymbolRef(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSymbolRefAttrGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, symbol_:  T1_, numReferences_:  T2_, references_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(symbol_), Into::<std::ffi::c_long>::into(numReferences_), Into::<*const MlirAttribute>::into(references_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirSymbolRefAttrGetRootReference<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGetRootReference(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirSymbolRefAttrGetLeafReference<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGetLeafReference(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirSymbolRefAttrGetNumNestedReferences<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGetNumNestedReferences(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSymbolRefAttrGetNestedReference<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGetNestedReference(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirSymbolRefAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSymbolRefAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDisctinctAttrCreate<T0_>(referencedAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDisctinctAttrCreate(Into::<MlirAttribute>::into(referencedAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAFlatSymbolRef<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAFlatSymbolRef(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirFlatSymbolRefAttrGet<T0_, T1_>(ctx_:  T0_, symbol_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFlatSymbolRefAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(symbol_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirFlatSymbolRefAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirFlatSymbolRefAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAType<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAType(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirTypeAttrGet<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirTypeAttrGet(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTypeAttrGetValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirTypeAttrGetValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTypeAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirTypeAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAUnit<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAUnit(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnitAttrGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnitAttrGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirUnitAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnitAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirElementsAttrGetValue<T0_, T1_, T2_>(attr_:  T0_, rank_:  T1_, idxs_:  T2_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(rank_), Into::<*mut std::ffi::c_ulong>::into(idxs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirElementsAttrIsValidIndex<T0_, T1_, T2_>(attr_:  T0_, rank_:  T1_, idxs_:  T2_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirElementsAttrIsValidIndex(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(rank_), Into::<*mut std::ffi::c_ulong>::into(idxs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirElementsAttrGetNumElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirElementsAttrGetNumElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirDenseArrayAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseArrayAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseBoolArray<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseBoolArray(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseI8Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseI8Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseI16Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseI16Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseI32Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseI32Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseI64Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseI64Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseF32Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseF32Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseF64Array<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseF64Array(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseBoolArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseBoolArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const std::ffi::c_int>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseI8ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_schar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI8ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const std::ffi::c_schar>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseI16ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_short>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI16ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const std::ffi::c_short>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseI32ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI32ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const std::ffi::c_int>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseI64ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI64ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const std::ffi::c_long>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseF32ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const f32>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseF32ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const f32>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseF64ArrayGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, values_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseF64ArrayGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*const f64>::into(values_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDenseArrayGetNumElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseArrayGetNumElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDenseBoolArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseBoolArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_schar> {
  pub unsafe fn mlirDenseI8ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI8ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_short> {
  pub unsafe fn mlirDenseI16ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI16ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirDenseI32ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI32ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDenseI64ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseI64ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f32> {
  pub unsafe fn mlirDenseF32ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseF32ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirDenseF64ArrayGetElement<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseF64ArrayGetElement(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseIntElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseIntElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseFPElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseFPElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirDenseIntOrFPElementsAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseIntOrFPElementsAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrGet<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const MlirAttribute>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrRawBufferGet<T0_, T1_, T2_>(shapedType_:  T0_, rawBufferSize_:  T1_, rawBuffer_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_ulong>,  T2_: Into<*const std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrRawBufferGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_ulong>::into(rawBufferSize_), Into::<*const std::ffi::c_void>::into(rawBuffer_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrSplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrSplatGet(Into::<MlirType>::into(shapedType_), Into::<MlirAttribute>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrBoolSplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrBoolSplatGet(Into::<MlirType>::into(shapedType_), Into::<u8>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt8SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_uchar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt8SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_uchar>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt8SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_schar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt8SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_schar>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt32SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt32SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_uint>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt32SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt32SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_int>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt64SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt64SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_ulong>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt64SplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt64SplatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrFloatSplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<f32>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrFloatSplatGet(Into::<MlirType>::into(shapedType_), Into::<f32>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrDoubleSplatGet<T0_, T1_>(shapedType_:  T0_, element_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrDoubleSplatGet(Into::<MlirType>::into(shapedType_), Into::<f64>::into(element_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrBoolGet<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrBoolGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_int>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt8Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_uchar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt8Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_uchar>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt8Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_schar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt8Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_schar>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt16Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_ushort>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt16Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ushort>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt16Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_short>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt16Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_short>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt32Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt32Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_uint>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt32Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt32Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_int>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrUInt64Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrUInt64Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ulong>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrInt64Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrInt64Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_long>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrFloatGet<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const f32>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrFloatGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const f32>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrDoubleGet<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrDoubleGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const f64>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrBFloat16Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_ushort>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrBFloat16Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ushort>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrFloat16Get<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_ushort>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrFloat16Get(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ushort>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrStringGet<T0_, T1_, T2_>(shapedType_:  T0_, numElements_:  T1_, strs_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrStringGet(Into::<MlirType>::into(shapedType_), Into::<std::ffi::c_long>::into(numElements_), Into::<*mut MlirStringRef>::into(strs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrReshapeGet<T0_, T1_>(attr_:  T0_, shapedType_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrReshapeGet(Into::<MlirAttribute>::into(attr_), Into::<MlirType>::into(shapedType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDenseElementsAttrIsSplat<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrIsSplat(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirDenseElementsAttrGetSplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetSplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirDenseElementsAttrGetBoolSplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetBoolSplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_schar> {
  pub unsafe fn mlirDenseElementsAttrGetInt8SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt8SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uchar> {
  pub unsafe fn mlirDenseElementsAttrGetUInt8SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt8SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirDenseElementsAttrGetInt32SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt32SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirDenseElementsAttrGetUInt32SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt32SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDenseElementsAttrGetInt64SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt64SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirDenseElementsAttrGetUInt64SplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt64SplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f32> {
  pub unsafe fn mlirDenseElementsAttrGetFloatSplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetFloatSplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirDenseElementsAttrGetDoubleSplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetDoubleSplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirDenseElementsAttrGetStringSplatValue<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetStringSplatValue(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDenseElementsAttrGetBoolValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetBoolValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_schar> {
  pub unsafe fn mlirDenseElementsAttrGetInt8Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt8Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uchar> {
  pub unsafe fn mlirDenseElementsAttrGetUInt8Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt8Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_short> {
  pub unsafe fn mlirDenseElementsAttrGetInt16Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt16Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ushort> {
  pub unsafe fn mlirDenseElementsAttrGetUInt16Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt16Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirDenseElementsAttrGetInt32Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt32Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirDenseElementsAttrGetUInt32Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt32Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDenseElementsAttrGetInt64Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetInt64Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirDenseElementsAttrGetUInt64Value<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetUInt64Value(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f32> {
  pub unsafe fn mlirDenseElementsAttrGetFloatValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetFloatValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirDenseElementsAttrGetDoubleValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetDoubleValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirDenseElementsAttrGetStringValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetStringValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*const std::ffi::c_void> {
  pub unsafe fn mlirDenseElementsAttrGetRawData<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseElementsAttrGetRawData(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsADenseResourceElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsADenseResourceElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseResourceElementsAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(shapedType_:  T0_, name_:  T1_, data_:  T2_, dataLength_:  T3_, dataAlignment_:  T4_, dataIsMutable_:  T5_, deleter_:  T6_, userData_:  T7_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<*mut std::ffi::c_void>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_ulong>,  T5_: Into<u8>,  T6_: Into<*mut extern fn (*mut std::ffi::c_void, *const std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_ulong) -> ()>,  T7_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<*mut std::ffi::c_void>::into(data_), Into::<std::ffi::c_ulong>::into(dataLength_), Into::<std::ffi::c_ulong>::into(dataAlignment_), Into::<u8>::into(dataIsMutable_), Into::<*mut extern fn (*mut std::ffi::c_void, *const std::ffi::c_void, std::ffi::c_ulong, std::ffi::c_ulong) -> ()>::into(deleter_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseBoolResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseBoolResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_int>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseUInt8ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_uchar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseUInt8ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_uchar>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseInt8ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_schar>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseInt8ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_schar>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseUInt16ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_ushort>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseUInt16ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ushort>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseInt16ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_short>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseInt16ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_short>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseUInt32ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseUInt32ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_uint>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseInt32ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseInt32ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_int>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseUInt64ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseUInt64ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_ulong>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseInt64ResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseInt64ResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const std::ffi::c_long>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseFloatResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const f32>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseFloatResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const f32>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnmanagedDenseDoubleResourceElementsAttrGet<T0_, T1_, T2_, T3_>(shapedType_:  T0_, name_:  T1_, numElements_:  T2_, elements_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirUnmanagedDenseDoubleResourceElementsAttrGet(Into::<MlirType>::into(shapedType_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const f64>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDenseBoolResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseBoolResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_schar> {
  pub unsafe fn mlirDenseInt8ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseInt8ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uchar> {
  pub unsafe fn mlirDenseUInt8ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseUInt8ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_short> {
  pub unsafe fn mlirDenseInt16ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseInt16ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ushort> {
  pub unsafe fn mlirDenseUInt16ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseUInt16ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirDenseInt32ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseInt32ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirDenseUInt32ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseUInt32ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDenseInt64ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseInt64ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirDenseUInt64ResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseUInt64ResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f32> {
  pub unsafe fn mlirDenseFloatResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseFloatResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirDenseDoubleResourceElementsAttrGetValue<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirDenseDoubleResourceElementsAttrGetValue(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsASparseElements<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsASparseElements(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseElementsAttribute<T0_, T1_, T2_>(shapedType_:  T0_, denseIndices_:  T1_, denseValues_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSparseElementsAttribute(Into::<MlirType>::into(shapedType_), Into::<MlirAttribute>::into(denseIndices_), Into::<MlirAttribute>::into(denseValues_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseElementsAttrGetIndices<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSparseElementsAttrGetIndices(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseElementsAttrGetValues<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSparseElementsAttrGetValues(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirSparseElementsAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirSparseElementsAttrGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAStridedLayout<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirAttributeIsAStridedLayout(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirStridedLayoutAttrGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, offset_:  T1_, numStrides_:  T2_, strides_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStridedLayoutAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(offset_), Into::<std::ffi::c_long>::into(numStrides_), Into::<*const std::ffi::c_long>::into(strides_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirStridedLayoutAttrGetOffset<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStridedLayoutAttrGetOffset(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirStridedLayoutAttrGetNumStrides<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStridedLayoutAttrGetNumStrides(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirStridedLayoutAttrGetStride<T0_, T1_>(attr_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStridedLayoutAttrGetStride(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirStridedLayoutAttrGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinAttributes::mlirStridedLayoutAttrGetTypeID()
      }
    )
  }
}

