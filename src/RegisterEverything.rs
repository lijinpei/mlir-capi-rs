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

  pub fn mlirRegisterAllDialects(registry: MlirDialectRegistry) -> ();

  pub fn mlirRegisterAllLLVMTranslations(context: MlirContext) -> ();

  pub fn mlirRegisterAllPasses() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn mlirRegisterAllDialects<T0_>(registry_:  T0_)
  where
     T0_: Into<MlirDialectRegistry>
  {
    unsafe {
      crate::RegisterEverything::mlirRegisterAllDialects(Into::<MlirDialectRegistry>::into(registry_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegisterAllLLVMTranslations<T0_>(context_:  T0_)
  where
     T0_: Into<MlirContext>
  {
    unsafe {
      crate::RegisterEverything::mlirRegisterAllLLVMTranslations(Into::<MlirContext>::into(context_))
    }
  }
}

