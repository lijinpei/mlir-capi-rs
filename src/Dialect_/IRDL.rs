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

  pub fn mlirGetDialectHandle__irdl__() -> MlirDialectHandle;

  pub fn mlirLoadIRDLDialects(module: MlirModule) -> MlirLogicalResult;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__irdl__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::IRDL::mlirGetDialectHandle__irdl__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirLoadIRDLDialects<T0_>(module_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::IRDL::mlirLoadIRDLDialects(Into::<MlirModule>::into(module_))
      }
    )
  }
}

