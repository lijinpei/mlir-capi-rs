// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::IR::*;
use crate::Support::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirGetDialectHandle__nvgpu__() -> MlirDialectHandle;

  pub fn mlirTypeIsANVGPUTensorMapDescriptorType(r#type: MlirType) -> u8;

  pub fn mlirNVGPUTensorMapDescriptorTypeGet(ctx: MlirContext, tensorMemrefType: MlirType, swizzle: std::ffi::c_int, l2promo: std::ffi::c_int, oobFill: std::ffi::c_int, interleave: std::ffi::c_int) -> MlirType;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__nvgpu__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::NVGPU::mlirGetDialectHandle__nvgpu__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsANVGPUTensorMapDescriptorType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::NVGPU::mlirTypeIsANVGPUTensorMapDescriptorType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirNVGPUTensorMapDescriptorTypeGet<T0_, T1_, T2_, T3_, T4_, T5_>(ctx_:  T0_, tensorMemrefType_:  T1_, swizzle_:  T2_, l2promo_:  T3_, oobFill_:  T4_, interleave_:  T5_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirType>,  T2_: Into<std::ffi::c_int>,  T3_: Into<std::ffi::c_int>,  T4_: Into<std::ffi::c_int>,  T5_: Into<std::ffi::c_int>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::NVGPU::mlirNVGPUTensorMapDescriptorTypeGet(Into::<MlirContext>::into(ctx_), Into::<MlirType>::into(tensorMemrefType_), Into::<std::ffi::c_int>::into(swizzle_), Into::<std::ffi::c_int>::into(l2promo_), Into::<std::ffi::c_int>::into(oobFill_), Into::<std::ffi::c_int>::into(interleave_))
      }
    )
  }
}

