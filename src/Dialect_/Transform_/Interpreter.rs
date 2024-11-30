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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirTransformOptions {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirTransformOptions = StructMlirTransformOptions;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirTransformOptionsCreate() -> MlirTransformOptions;

  pub fn mlirTransformOptionsEnableExpensiveChecks(transformOptions: MlirTransformOptions, enable: u8) -> ();

  pub fn mlirTransformOptionsGetExpensiveChecksEnabled(transformOptions: MlirTransformOptions) -> u8;

  pub fn mlirTransformOptionsEnforceSingleTopLevelTransformOp(transformOptions: MlirTransformOptions, enable: u8) -> ();

  pub fn mlirTransformOptionsGetEnforceSingleTopLevelTransformOp(transformOptions: MlirTransformOptions) -> u8;

  pub fn mlirTransformOptionsDestroy(transformOptions: MlirTransformOptions) -> ();

  pub fn mlirTransformApplyNamedSequence(payload: MlirOperation, transformRoot: MlirOperation, transformModule: MlirOperation, transformOptions: MlirTransformOptions) -> MlirLogicalResult;

  pub fn mlirMergeSymbolsIntoFromClone(target: MlirOperation, other: MlirOperation) -> MlirLogicalResult;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTransformOptions> {
  pub unsafe fn mlirTransformOptionsCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTransformOptionsEnableExpensiveChecks<T0_, T1_>(transformOptions_:  T0_, enable_:  T1_)
  where
     T0_: Into<MlirTransformOptions>,  T1_: Into<u8>
  {
    unsafe {
      crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsEnableExpensiveChecks(Into::<MlirTransformOptions>::into(transformOptions_), Into::<u8>::into(enable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTransformOptionsGetExpensiveChecksEnabled<T0_>(transformOptions_:  T0_)-> Tret_
  where
     T0_: Into<MlirTransformOptions>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsGetExpensiveChecksEnabled(Into::<MlirTransformOptions>::into(transformOptions_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTransformOptionsEnforceSingleTopLevelTransformOp<T0_, T1_>(transformOptions_:  T0_, enable_:  T1_)
  where
     T0_: Into<MlirTransformOptions>,  T1_: Into<u8>
  {
    unsafe {
      crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsEnforceSingleTopLevelTransformOp(Into::<MlirTransformOptions>::into(transformOptions_), Into::<u8>::into(enable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTransformOptionsGetEnforceSingleTopLevelTransformOp<T0_>(transformOptions_:  T0_)-> Tret_
  where
     T0_: Into<MlirTransformOptions>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsGetEnforceSingleTopLevelTransformOp(Into::<MlirTransformOptions>::into(transformOptions_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTransformOptionsDestroy<T0_>(transformOptions_:  T0_)
  where
     T0_: Into<MlirTransformOptions>
  {
    unsafe {
      crate::Dialect_::Transform_::Interpreter::mlirTransformOptionsDestroy(Into::<MlirTransformOptions>::into(transformOptions_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirTransformApplyNamedSequence<T0_, T1_, T2_, T3_>(payload_:  T0_, transformRoot_:  T1_, transformModule_:  T2_, transformOptions_:  T3_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>,  T3_: Into<MlirTransformOptions>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform_::Interpreter::mlirTransformApplyNamedSequence(Into::<MlirOperation>::into(payload_), Into::<MlirOperation>::into(transformRoot_), Into::<MlirOperation>::into(transformModule_), Into::<MlirTransformOptions>::into(transformOptions_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirMergeSymbolsIntoFromClone<T0_, T1_>(target_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform_::Interpreter::mlirMergeSymbolsIntoFromClone(Into::<MlirOperation>::into(target_), Into::<MlirOperation>::into(other_))
      }
    )
  }
}

