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

#[link(name = "MLIR-C")]
extern {

  pub fn mlirIntegerTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAInteger(r#type: MlirType) -> u8;

  pub fn mlirIntegerTypeGet(ctx: MlirContext, bitwidth: std::ffi::c_uint) -> MlirType;

  pub fn mlirIntegerTypeSignedGet(ctx: MlirContext, bitwidth: std::ffi::c_uint) -> MlirType;

  pub fn mlirIntegerTypeUnsignedGet(ctx: MlirContext, bitwidth: std::ffi::c_uint) -> MlirType;

  pub fn mlirIntegerTypeGetWidth(r#type: MlirType) -> std::ffi::c_uint;

  pub fn mlirIntegerTypeIsSignless(r#type: MlirType) -> u8;

  pub fn mlirIntegerTypeIsSigned(r#type: MlirType) -> u8;

  pub fn mlirIntegerTypeIsUnsigned(r#type: MlirType) -> u8;

  pub fn mlirIndexTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAIndex(r#type: MlirType) -> u8;

  pub fn mlirIndexTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsAFloat(r#type: MlirType) -> u8;

  pub fn mlirFloatTypeGetWidth(r#type: MlirType) -> std::ffi::c_uint;

  pub fn mlirFloat4E2M1FNTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat4E2M1FN(r#type: MlirType) -> u8;

  pub fn mlirFloat4E2M1FNTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat6E2M3FNTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat6E2M3FN(r#type: MlirType) -> u8;

  pub fn mlirFloat6E2M3FNTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat6E3M2FNTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat6E3M2FN(r#type: MlirType) -> u8;

  pub fn mlirFloat6E3M2FNTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E5M2TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E5M2(r#type: MlirType) -> u8;

  pub fn mlirFloat8E5M2TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E4M3TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E4M3(r#type: MlirType) -> u8;

  pub fn mlirFloat8E4M3TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E4M3FNTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E4M3FN(r#type: MlirType) -> u8;

  pub fn mlirFloat8E4M3FNTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E5M2FNUZTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E5M2FNUZ(r#type: MlirType) -> u8;

  pub fn mlirFloat8E5M2FNUZTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E4M3FNUZTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E4M3FNUZ(r#type: MlirType) -> u8;

  pub fn mlirFloat8E4M3FNUZTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E4M3B11FNUZTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E4M3B11FNUZ(r#type: MlirType) -> u8;

  pub fn mlirFloat8E4M3B11FNUZTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E3M4TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E3M4(r#type: MlirType) -> u8;

  pub fn mlirFloat8E3M4TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat8E8M0FNUTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFloat8E8M0FNU(r#type: MlirType) -> u8;

  pub fn mlirFloat8E8M0FNUTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirBFloat16TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsABF16(r#type: MlirType) -> u8;

  pub fn mlirBF16TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat16TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAF16(r#type: MlirType) -> u8;

  pub fn mlirF16TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat32TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAF32(r#type: MlirType) -> u8;

  pub fn mlirF32TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloat64TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAF64(r#type: MlirType) -> u8;

  pub fn mlirF64TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirFloatTF32TypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsATF32(r#type: MlirType) -> u8;

  pub fn mlirTF32TypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirNoneTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsANone(r#type: MlirType) -> u8;

  pub fn mlirNoneTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirComplexTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAComplex(r#type: MlirType) -> u8;

  pub fn mlirComplexTypeGet(elementType: MlirType) -> MlirType;

  pub fn mlirComplexTypeGetElementType(r#type: MlirType) -> MlirType;

  pub fn mlirTypeIsAShaped(r#type: MlirType) -> u8;

  pub fn mlirShapedTypeGetElementType(r#type: MlirType) -> MlirType;

  pub fn mlirShapedTypeHasRank(r#type: MlirType) -> u8;

  pub fn mlirShapedTypeGetRank(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirShapedTypeHasStaticShape(r#type: MlirType) -> u8;

  pub fn mlirShapedTypeIsDynamicDim(r#type: MlirType, dim: std::ffi::c_long) -> u8;

  pub fn mlirShapedTypeGetDimSize(r#type: MlirType, dim: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirShapedTypeIsDynamicSize(size: std::ffi::c_long) -> u8;

  pub fn mlirShapedTypeGetDynamicSize() -> std::ffi::c_long;

  pub fn mlirShapedTypeIsDynamicStrideOrOffset(val: std::ffi::c_long) -> u8;

  pub fn mlirShapedTypeGetDynamicStrideOrOffset() -> std::ffi::c_long;

  pub fn mlirVectorTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAVector(r#type: MlirType) -> u8;

  pub fn mlirVectorTypeGet(rank: std::ffi::c_long, shape: *const std::ffi::c_long, elementType: MlirType) -> MlirType;

  pub fn mlirVectorTypeGetChecked(loc: MlirLocation, rank: std::ffi::c_long, shape: *const std::ffi::c_long, elementType: MlirType) -> MlirType;

  pub fn mlirVectorTypeGetScalable(rank: std::ffi::c_long, shape: *const std::ffi::c_long, scalable: *const u8, elementType: MlirType) -> MlirType;

  pub fn mlirVectorTypeGetScalableChecked(loc: MlirLocation, rank: std::ffi::c_long, shape: *const std::ffi::c_long, scalable: *const u8, elementType: MlirType) -> MlirType;

  pub fn mlirVectorTypeIsScalable(r#type: MlirType) -> u8;

  pub fn mlirVectorTypeIsDimScalable(r#type: MlirType, dim: std::ffi::c_long) -> u8;

  pub fn mlirTypeIsATensor(r#type: MlirType) -> u8;

  pub fn mlirRankedTensorTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsARankedTensor(r#type: MlirType) -> u8;

  pub fn mlirUnrankedTensorTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAUnrankedTensor(r#type: MlirType) -> u8;

  pub fn mlirRankedTensorTypeGet(rank: std::ffi::c_long, shape: *const std::ffi::c_long, elementType: MlirType, encoding: MlirAttribute) -> MlirType;

  pub fn mlirRankedTensorTypeGetChecked(loc: MlirLocation, rank: std::ffi::c_long, shape: *const std::ffi::c_long, elementType: MlirType, encoding: MlirAttribute) -> MlirType;

  pub fn mlirRankedTensorTypeGetEncoding(r#type: MlirType) -> MlirAttribute;

  pub fn mlirUnrankedTensorTypeGet(elementType: MlirType) -> MlirType;

  pub fn mlirUnrankedTensorTypeGetChecked(loc: MlirLocation, elementType: MlirType) -> MlirType;

  pub fn mlirMemRefTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAMemRef(r#type: MlirType) -> u8;

  pub fn mlirUnrankedMemRefTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAUnrankedMemRef(r#type: MlirType) -> u8;

  pub fn mlirMemRefTypeGet(elementType: MlirType, rank: std::ffi::c_long, shape: *const std::ffi::c_long, layout: MlirAttribute, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirMemRefTypeGetChecked(loc: MlirLocation, elementType: MlirType, rank: std::ffi::c_long, shape: *const std::ffi::c_long, layout: MlirAttribute, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirMemRefTypeContiguousGet(elementType: MlirType, rank: std::ffi::c_long, shape: *const std::ffi::c_long, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirMemRefTypeContiguousGetChecked(loc: MlirLocation, elementType: MlirType, rank: std::ffi::c_long, shape: *const std::ffi::c_long, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirUnrankedMemRefTypeGet(elementType: MlirType, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirUnrankedMemRefTypeGetChecked(loc: MlirLocation, elementType: MlirType, memorySpace: MlirAttribute) -> MlirType;

  pub fn mlirMemRefTypeGetLayout(r#type: MlirType) -> MlirAttribute;

  pub fn mlirMemRefTypeGetAffineMap(r#type: MlirType) -> MlirAffineMap;

  pub fn mlirMemRefTypeGetMemorySpace(r#type: MlirType) -> MlirAttribute;

  pub fn mlirMemRefTypeGetStridesAndOffset(r#type: MlirType, strides: *mut std::ffi::c_long, offset: *mut std::ffi::c_long) -> MlirLogicalResult;

  pub fn mlirUnrankedMemrefGetMemorySpace(r#type: MlirType) -> MlirAttribute;

  pub fn mlirTupleTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsATuple(r#type: MlirType) -> u8;

  pub fn mlirTupleTypeGet(ctx: MlirContext, numElements: std::ffi::c_long, elements: *const MlirType) -> MlirType;

  pub fn mlirTupleTypeGetNumTypes(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirTupleTypeGetType(r#type: MlirType, pos: std::ffi::c_long) -> MlirType;

  pub fn mlirFunctionTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAFunction(r#type: MlirType) -> u8;

  pub fn mlirFunctionTypeGet(ctx: MlirContext, numInputs: std::ffi::c_long, inputs: *const MlirType, numResults: std::ffi::c_long, results: *const MlirType) -> MlirType;

  pub fn mlirFunctionTypeGetNumInputs(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirFunctionTypeGetNumResults(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirFunctionTypeGetInput(r#type: MlirType, pos: std::ffi::c_long) -> MlirType;

  pub fn mlirFunctionTypeGetResult(r#type: MlirType, pos: std::ffi::c_long) -> MlirType;

  pub fn mlirOpaqueTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTypeIsAOpaque(r#type: MlirType) -> u8;

  pub fn mlirOpaqueTypeGet(ctx: MlirContext, dialectNamespace: MlirStringRef, typeData: MlirStringRef) -> MlirType;

  pub fn mlirOpaqueTypeGetDialectNamespace(r#type: MlirType) -> MlirStringRef;

  pub fn mlirOpaqueTypeGetData(r#type: MlirType) -> MlirStringRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirIntegerTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAInteger<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAInteger(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirIntegerTypeGet<T0_, T1_>(ctx_:  T0_, bitwidth_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(bitwidth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirIntegerTypeSignedGet<T0_, T1_>(ctx_:  T0_, bitwidth_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeSignedGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(bitwidth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirIntegerTypeUnsignedGet<T0_, T1_>(ctx_:  T0_, bitwidth_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeUnsignedGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(bitwidth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirIntegerTypeGetWidth<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeGetWidth(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerTypeIsSignless<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeIsSignless(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerTypeIsSigned<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeIsSigned(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerTypeIsUnsigned<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIntegerTypeIsUnsigned(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirIndexTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIndexTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAIndex<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAIndex(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirIndexTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirIndexTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirFloatTypeGetWidth<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloatTypeGetWidth(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat4E2M1FNTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat4E2M1FNTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat4E2M1FN<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat4E2M1FN(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat4E2M1FNTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat4E2M1FNTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat6E2M3FNTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat6E2M3FNTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat6E2M3FN<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat6E2M3FN(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat6E2M3FNTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat6E2M3FNTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat6E3M2FNTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat6E3M2FNTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat6E3M2FN<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat6E3M2FN(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat6E3M2FNTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat6E3M2FNTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E5M2TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E5M2TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E5M2<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E5M2(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E5M2TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E5M2TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E4M3TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E4M3<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E4M3(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E4M3TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E4M3FNTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3FNTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E4M3FN<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E4M3FN(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E4M3FNTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3FNTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E5M2FNUZTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E5M2FNUZTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E5M2FNUZ<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E5M2FNUZ(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E5M2FNUZTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E5M2FNUZTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E4M3FNUZTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3FNUZTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E4M3FNUZ<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E4M3FNUZ(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E4M3FNUZTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3FNUZTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E4M3B11FNUZTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3B11FNUZTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E4M3B11FNUZ<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E4M3B11FNUZ(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E4M3B11FNUZTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E4M3B11FNUZTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E3M4TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E3M4TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E3M4<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E3M4(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E3M4TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E3M4TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat8E8M0FNUTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E8M0FNUTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFloat8E8M0FNU<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFloat8E8M0FNU(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFloat8E8M0FNUTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat8E8M0FNUTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirBFloat16TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirBFloat16TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsABF16<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsABF16(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirBF16TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirBF16TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat16TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat16TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAF16<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAF16(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirF16TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirF16TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat32TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat32TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAF32<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAF32(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirF32TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirF32TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloat64TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloat64TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAF64<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAF64(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirF64TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirF64TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFloatTF32TypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFloatTF32TypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATF32<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsATF32(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTF32TypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTF32TypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirNoneTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirNoneTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsANone<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsANone(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirNoneTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirNoneTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirComplexTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirComplexTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAComplex<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAComplex(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirComplexTypeGet<T0_>(elementType_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirComplexTypeGet(Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirComplexTypeGetElementType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirComplexTypeGetElementType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAShaped<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAShaped(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirShapedTypeGetElementType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeGetElementType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirShapedTypeHasRank<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeHasRank(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirShapedTypeGetRank<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeGetRank(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirShapedTypeHasStaticShape<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeHasStaticShape(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirShapedTypeIsDynamicDim<T0_, T1_>(type_:  T0_, dim_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeIsDynamicDim(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(dim_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirShapedTypeGetDimSize<T0_, T1_>(type_:  T0_, dim_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeGetDimSize(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(dim_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirShapedTypeIsDynamicSize<T0_>(size_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeIsDynamicSize(Into::<std::ffi::c_long>::into(size_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirShapedTypeGetDynamicSize()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeGetDynamicSize()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirShapedTypeIsDynamicStrideOrOffset<T0_>(val_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeIsDynamicStrideOrOffset(Into::<std::ffi::c_long>::into(val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirShapedTypeGetDynamicStrideOrOffset()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirShapedTypeGetDynamicStrideOrOffset()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirVectorTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAVector<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAVector(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirVectorTypeGet<T0_, T1_, T2_>(rank_:  T0_, shape_:  T1_, elementType_:  T2_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>,  T1_: Into<*const std::ffi::c_long>,  T2_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeGet(Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirVectorTypeGetChecked<T0_, T1_, T2_, T3_>(loc_:  T0_, rank_:  T1_, shape_:  T2_, elementType_:  T3_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>,  T3_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeGetChecked(Into::<MlirLocation>::into(loc_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirVectorTypeGetScalable<T0_, T1_, T2_, T3_>(rank_:  T0_, shape_:  T1_, scalable_:  T2_, elementType_:  T3_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>,  T1_: Into<*const std::ffi::c_long>,  T2_: Into<*const u8>,  T3_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeGetScalable(Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<*const u8>::into(scalable_), Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirVectorTypeGetScalableChecked<T0_, T1_, T2_, T3_, T4_>(loc_:  T0_, rank_:  T1_, shape_:  T2_, scalable_:  T3_, elementType_:  T4_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>,  T3_: Into<*const u8>,  T4_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeGetScalableChecked(Into::<MlirLocation>::into(loc_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<*const u8>::into(scalable_), Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirVectorTypeIsScalable<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeIsScalable(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirVectorTypeIsDimScalable<T0_, T1_>(type_:  T0_, dim_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirVectorTypeIsDimScalable(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(dim_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATensor<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsATensor(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirRankedTensorTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirRankedTensorTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsARankedTensor<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsARankedTensor(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirUnrankedTensorTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedTensorTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAUnrankedTensor<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAUnrankedTensor(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirRankedTensorTypeGet<T0_, T1_, T2_, T3_>(rank_:  T0_, shape_:  T1_, elementType_:  T2_, encoding_:  T3_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>,  T1_: Into<*const std::ffi::c_long>,  T2_: Into<MlirType>,  T3_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirRankedTensorTypeGet(Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirType>::into(elementType_), Into::<MlirAttribute>::into(encoding_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirRankedTensorTypeGetChecked<T0_, T1_, T2_, T3_, T4_>(loc_:  T0_, rank_:  T1_, shape_:  T2_, elementType_:  T3_, encoding_:  T4_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>,  T3_: Into<MlirType>,  T4_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirRankedTensorTypeGetChecked(Into::<MlirLocation>::into(loc_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirType>::into(elementType_), Into::<MlirAttribute>::into(encoding_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirRankedTensorTypeGetEncoding<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirRankedTensorTypeGetEncoding(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUnrankedTensorTypeGet<T0_>(elementType_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedTensorTypeGet(Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUnrankedTensorTypeGetChecked<T0_, T1_>(loc_:  T0_, elementType_:  T1_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedTensorTypeGetChecked(Into::<MlirLocation>::into(loc_), Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirMemRefTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAMemRef<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAMemRef(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirUnrankedMemRefTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedMemRefTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAUnrankedMemRef<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAUnrankedMemRef(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirMemRefTypeGet<T0_, T1_, T2_, T3_, T4_>(elementType_:  T0_, rank_:  T1_, shape_:  T2_, layout_:  T3_, memorySpace_:  T4_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>,  T3_: Into<MlirAttribute>,  T4_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGet(Into::<MlirType>::into(elementType_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirAttribute>::into(layout_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirMemRefTypeGetChecked<T0_, T1_, T2_, T3_, T4_, T5_>(loc_:  T0_, elementType_:  T1_, rank_:  T2_, shape_:  T3_, layout_:  T4_, memorySpace_:  T5_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirType>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_long>,  T4_: Into<MlirAttribute>,  T5_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetChecked(Into::<MlirLocation>::into(loc_), Into::<MlirType>::into(elementType_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirAttribute>::into(layout_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirMemRefTypeContiguousGet<T0_, T1_, T2_, T3_>(elementType_:  T0_, rank_:  T1_, shape_:  T2_, memorySpace_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_long>,  T3_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeContiguousGet(Into::<MlirType>::into(elementType_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirMemRefTypeContiguousGetChecked<T0_, T1_, T2_, T3_, T4_>(loc_:  T0_, elementType_:  T1_, rank_:  T2_, shape_:  T3_, memorySpace_:  T4_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirType>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_long>,  T4_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeContiguousGetChecked(Into::<MlirLocation>::into(loc_), Into::<MlirType>::into(elementType_), Into::<std::ffi::c_long>::into(rank_), Into::<*const std::ffi::c_long>::into(shape_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUnrankedMemRefTypeGet<T0_, T1_>(elementType_:  T0_, memorySpace_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedMemRefTypeGet(Into::<MlirType>::into(elementType_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUnrankedMemRefTypeGetChecked<T0_, T1_, T2_>(loc_:  T0_, elementType_:  T1_, memorySpace_:  T2_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirType>,  T2_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedMemRefTypeGetChecked(Into::<MlirLocation>::into(loc_), Into::<MlirType>::into(elementType_), Into::<MlirAttribute>::into(memorySpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirMemRefTypeGetLayout<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetLayout(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirMemRefTypeGetAffineMap<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetAffineMap(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirMemRefTypeGetMemorySpace<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetMemorySpace(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirMemRefTypeGetStridesAndOffset<T0_, T1_, T2_>(type_:  T0_, strides_:  T1_, offset_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<*mut std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirMemRefTypeGetStridesAndOffset(Into::<MlirType>::into(type_), Into::<*mut std::ffi::c_long>::into(strides_), Into::<*mut std::ffi::c_long>::into(offset_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirUnrankedMemrefGetMemorySpace<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirUnrankedMemrefGetMemorySpace(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTupleTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTupleTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATuple<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsATuple(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTupleTypeGet<T0_, T1_, T2_>(ctx_:  T0_, numElements_:  T1_, elements_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTupleTypeGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(numElements_), Into::<*const MlirType>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirTupleTypeGetNumTypes<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTupleTypeGetNumTypes(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTupleTypeGetType<T0_, T1_>(type_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTupleTypeGetType(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirFunctionTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAFunction<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAFunction(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFunctionTypeGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, numInputs_:  T1_, inputs_:  T2_, numResults_:  T3_, results_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*const MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(numInputs_), Into::<*const MlirType>::into(inputs_), Into::<std::ffi::c_long>::into(numResults_), Into::<*const MlirType>::into(results_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirFunctionTypeGetNumInputs<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGetNumInputs(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirFunctionTypeGetNumResults<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGetNumResults(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFunctionTypeGetInput<T0_, T1_>(type_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGetInput(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirFunctionTypeGetResult<T0_, T1_>(type_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirFunctionTypeGetResult(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirOpaqueTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirOpaqueTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAOpaque<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirTypeIsAOpaque(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirOpaqueTypeGet<T0_, T1_, T2_>(ctx_:  T0_, dialectNamespace_:  T1_, typeData_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirOpaqueTypeGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(dialectNamespace_), Into::<MlirStringRef>::into(typeData_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirOpaqueTypeGetDialectNamespace<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirOpaqueTypeGetDialectNamespace(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirOpaqueTypeGetData<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::BuiltinTypes::mlirOpaqueTypeGetData(Into::<MlirType>::into(type_))
      }
    )
  }
}

