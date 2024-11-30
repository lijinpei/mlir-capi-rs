// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::IR::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirGetDialectHandle__quant__() -> MlirDialectHandle;

  pub fn mlirTypeIsAQuantizedType(r#type: MlirType) -> u8;

  pub fn mlirQuantizedTypeGetSignedFlag() -> std::ffi::c_uint;

  pub fn mlirQuantizedTypeGetDefaultMinimumForInteger(isSigned: u8, integralWidth: std::ffi::c_uint) -> std::ffi::c_long;

  pub fn mlirQuantizedTypeGetDefaultMaximumForInteger(isSigned: u8, integralWidth: std::ffi::c_uint) -> std::ffi::c_long;

  pub fn mlirQuantizedTypeGetExpressedType(r#type: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeGetFlags(r#type: MlirType) -> std::ffi::c_uint;

  pub fn mlirQuantizedTypeIsSigned(r#type: MlirType) -> u8;

  pub fn mlirQuantizedTypeGetStorageType(r#type: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeGetStorageTypeMin(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirQuantizedTypeGetStorageTypeMax(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirQuantizedTypeGetStorageTypeIntegralWidth(r#type: MlirType) -> std::ffi::c_uint;

  pub fn mlirQuantizedTypeIsCompatibleExpressedType(r#type: MlirType, candidate: MlirType) -> u8;

  pub fn mlirQuantizedTypeGetQuantizedElementType(r#type: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeCastFromStorageType(r#type: MlirType, candidate: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeCastToStorageType(r#type: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeCastFromExpressedType(r#type: MlirType, candidate: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeCastToExpressedType(r#type: MlirType) -> MlirType;

  pub fn mlirQuantizedTypeCastExpressedToStorageType(r#type: MlirType, candidate: MlirType) -> MlirType;

  pub fn mlirTypeIsAAnyQuantizedType(r#type: MlirType) -> u8;

  pub fn mlirAnyQuantizedTypeGet(flags: std::ffi::c_uint, storageType: MlirType, expressedType: MlirType, storageTypeMin: std::ffi::c_long, storageTypeMax: std::ffi::c_long) -> MlirType;

  pub fn mlirTypeIsAUniformQuantizedType(r#type: MlirType) -> u8;

  pub fn mlirUniformQuantizedTypeGet(flags: std::ffi::c_uint, storageType: MlirType, expressedType: MlirType, scale: f64, zeroPoint: std::ffi::c_long, storageTypeMin: std::ffi::c_long, storageTypeMax: std::ffi::c_long) -> MlirType;

  pub fn mlirUniformQuantizedTypeGetScale(r#type: MlirType) -> f64;

  pub fn mlirUniformQuantizedTypeGetZeroPoint(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirUniformQuantizedTypeIsFixedPoint(r#type: MlirType) -> u8;

  pub fn mlirTypeIsAUniformQuantizedPerAxisType(r#type: MlirType) -> u8;

  pub fn mlirUniformQuantizedPerAxisTypeGet(flags: std::ffi::c_uint, storageType: MlirType, expressedType: MlirType, nDims: std::ffi::c_long, scales: *mut f64, zeroPoints: *mut std::ffi::c_long, quantizedDimension: std::ffi::c_int, storageTypeMin: std::ffi::c_long, storageTypeMax: std::ffi::c_long) -> MlirType;

  pub fn mlirUniformQuantizedPerAxisTypeGetNumDims(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirUniformQuantizedPerAxisTypeGetScale(r#type: MlirType, pos: std::ffi::c_long) -> f64;

  pub fn mlirUniformQuantizedPerAxisTypeGetZeroPoint(r#type: MlirType, pos: std::ffi::c_long) -> std::ffi::c_long;

  pub fn mlirUniformQuantizedPerAxisTypeGetQuantizedDimension(r#type: MlirType) -> std::ffi::c_int;

  pub fn mlirUniformQuantizedPerAxisTypeIsFixedPoint(r#type: MlirType) -> u8;

  pub fn mlirTypeIsACalibratedQuantizedType(r#type: MlirType) -> u8;

  pub fn mlirCalibratedQuantizedTypeGet(expressedType: MlirType, min: f64, max: f64) -> MlirType;

  pub fn mlirCalibratedQuantizedTypeGetMin(r#type: MlirType) -> f64;

  pub fn mlirCalibratedQuantizedTypeGetMax(r#type: MlirType) -> f64;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__quant__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirGetDialectHandle__quant__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAQuantizedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirTypeIsAQuantizedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirQuantizedTypeGetSignedFlag()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetSignedFlag()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirQuantizedTypeGetDefaultMinimumForInteger<T0_, T1_>(isSigned_:  T0_, integralWidth_:  T1_)-> Tret_
  where
     T0_: Into<u8>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetDefaultMinimumForInteger(Into::<u8>::into(isSigned_), Into::<std::ffi::c_uint>::into(integralWidth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirQuantizedTypeGetDefaultMaximumForInteger<T0_, T1_>(isSigned_:  T0_, integralWidth_:  T1_)-> Tret_
  where
     T0_: Into<u8>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetDefaultMaximumForInteger(Into::<u8>::into(isSigned_), Into::<std::ffi::c_uint>::into(integralWidth_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeGetExpressedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetExpressedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirQuantizedTypeGetFlags<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetFlags(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirQuantizedTypeIsSigned<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeIsSigned(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeGetStorageType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetStorageType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirQuantizedTypeGetStorageTypeMin<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetStorageTypeMin(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirQuantizedTypeGetStorageTypeMax<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetStorageTypeMax(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirQuantizedTypeGetStorageTypeIntegralWidth<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetStorageTypeIntegralWidth(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirQuantizedTypeIsCompatibleExpressedType<T0_, T1_>(type_:  T0_, candidate_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeIsCompatibleExpressedType(Into::<MlirType>::into(type_), Into::<MlirType>::into(candidate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeGetQuantizedElementType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeGetQuantizedElementType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeCastFromStorageType<T0_, T1_>(type_:  T0_, candidate_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeCastFromStorageType(Into::<MlirType>::into(type_), Into::<MlirType>::into(candidate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeCastToStorageType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeCastToStorageType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeCastFromExpressedType<T0_, T1_>(type_:  T0_, candidate_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeCastFromExpressedType(Into::<MlirType>::into(type_), Into::<MlirType>::into(candidate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeCastToExpressedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeCastToExpressedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirQuantizedTypeCastExpressedToStorageType<T0_, T1_>(type_:  T0_, candidate_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirQuantizedTypeCastExpressedToStorageType(Into::<MlirType>::into(type_), Into::<MlirType>::into(candidate_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAAnyQuantizedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirTypeIsAAnyQuantizedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirAnyQuantizedTypeGet<T0_, T1_, T2_, T3_, T4_>(flags_:  T0_, storageType_:  T1_, expressedType_:  T2_, storageTypeMin_:  T3_, storageTypeMax_:  T4_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>,  T1_: Into<MlirType>,  T2_: Into<MlirType>,  T3_: Into<std::ffi::c_long>,  T4_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirAnyQuantizedTypeGet(Into::<std::ffi::c_uint>::into(flags_), Into::<MlirType>::into(storageType_), Into::<MlirType>::into(expressedType_), Into::<std::ffi::c_long>::into(storageTypeMin_), Into::<std::ffi::c_long>::into(storageTypeMax_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAUniformQuantizedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirTypeIsAUniformQuantizedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUniformQuantizedTypeGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_>(flags_:  T0_, storageType_:  T1_, expressedType_:  T2_, scale_:  T3_, zeroPoint_:  T4_, storageTypeMin_:  T5_, storageTypeMax_:  T6_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>,  T1_: Into<MlirType>,  T2_: Into<MlirType>,  T3_: Into<f64>,  T4_: Into<std::ffi::c_long>,  T5_: Into<std::ffi::c_long>,  T6_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedTypeGet(Into::<std::ffi::c_uint>::into(flags_), Into::<MlirType>::into(storageType_), Into::<MlirType>::into(expressedType_), Into::<f64>::into(scale_), Into::<std::ffi::c_long>::into(zeroPoint_), Into::<std::ffi::c_long>::into(storageTypeMin_), Into::<std::ffi::c_long>::into(storageTypeMax_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirUniformQuantizedTypeGetScale<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedTypeGetScale(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirUniformQuantizedTypeGetZeroPoint<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedTypeGetZeroPoint(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirUniformQuantizedTypeIsFixedPoint<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedTypeIsFixedPoint(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAUniformQuantizedPerAxisType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirTypeIsAUniformQuantizedPerAxisType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(flags_:  T0_, storageType_:  T1_, expressedType_:  T2_, nDims_:  T3_, scales_:  T4_, zeroPoints_:  T5_, quantizedDimension_:  T6_, storageTypeMin_:  T7_, storageTypeMax_:  T8_)-> Tret_
  where
     T0_: Into<std::ffi::c_uint>,  T1_: Into<MlirType>,  T2_: Into<MlirType>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*mut f64>,  T5_: Into<*mut std::ffi::c_long>,  T6_: Into<std::ffi::c_int>,  T7_: Into<std::ffi::c_long>,  T8_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeGet(Into::<std::ffi::c_uint>::into(flags_), Into::<MlirType>::into(storageType_), Into::<MlirType>::into(expressedType_), Into::<std::ffi::c_long>::into(nDims_), Into::<*mut f64>::into(scales_), Into::<*mut std::ffi::c_long>::into(zeroPoints_), Into::<std::ffi::c_int>::into(quantizedDimension_), Into::<std::ffi::c_long>::into(storageTypeMin_), Into::<std::ffi::c_long>::into(storageTypeMax_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeGetNumDims<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeGetNumDims(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeGetScale<T0_, T1_>(type_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeGetScale(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeGetZeroPoint<T0_, T1_>(type_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeGetZeroPoint(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeGetQuantizedDimension<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeGetQuantizedDimension(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirUniformQuantizedPerAxisTypeIsFixedPoint<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirUniformQuantizedPerAxisTypeIsFixedPoint(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsACalibratedQuantizedType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirTypeIsACalibratedQuantizedType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirCalibratedQuantizedTypeGet<T0_, T1_, T2_>(expressedType_:  T0_, min_:  T1_, max_:  T2_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<f64>,  T2_: Into<f64>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirCalibratedQuantizedTypeGet(Into::<MlirType>::into(expressedType_), Into::<f64>::into(min_), Into::<f64>::into(max_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirCalibratedQuantizedTypeGetMin<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirCalibratedQuantizedTypeGetMin(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<f64> {
  pub unsafe fn mlirCalibratedQuantizedTypeGetMax<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Quant::mlirCalibratedQuantizedTypeGetMax(Into::<MlirType>::into(type_))
      }
    )
  }
}

