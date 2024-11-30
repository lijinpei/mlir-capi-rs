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

  pub fn mlirGetDialectHandle__func__() -> MlirDialectHandle;

  pub fn mlirFuncSetArgAttr(op: MlirOperation, pos: std::ffi::c_long, name: MlirStringRef, attr: MlirAttribute) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__func__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Func::mlirGetDialectHandle__func__()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirFuncSetArgAttr<T0_, T1_, T2_, T3_>(op_:  T0_, pos_:  T1_, name_:  T2_, attr_:  T3_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirStringRef>,  T3_: Into<MlirAttribute>
  {
    unsafe {
      crate::Dialect_::Func::mlirFuncSetArgAttr(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirStringRef>::into(name_), Into::<MlirAttribute>::into(attr_))
    }
  }
}

