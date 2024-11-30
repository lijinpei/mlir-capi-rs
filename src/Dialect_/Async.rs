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
use crate::Pass::*;
use crate::IR::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirGetDialectHandle__async__() -> MlirDialectHandle;

  pub fn mlirRegisterAsyncPasses() -> ();

  pub fn mlirCreateAsyncAsyncFuncToAsyncRuntime() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncFuncToAsyncRuntime() -> ();

  pub fn mlirCreateAsyncAsyncParallelFor() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncParallelFor() -> ();

  pub fn mlirCreateAsyncAsyncRuntimePolicyBasedRefCounting() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncRuntimePolicyBasedRefCounting() -> ();

  pub fn mlirCreateAsyncAsyncRuntimeRefCounting() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncRuntimeRefCounting() -> ();

  pub fn mlirCreateAsyncAsyncRuntimeRefCountingOpt() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncRuntimeRefCountingOpt() -> ();

  pub fn mlirCreateAsyncAsyncToAsyncRuntime() -> MlirPass;

  pub fn mlirRegisterAsyncAsyncToAsyncRuntime() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__async__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirGetDialectHandle__async__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncFuncToAsyncRuntime()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncFuncToAsyncRuntime()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncParallelFor()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncParallelFor()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncRuntimePolicyBasedRefCounting()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncRuntimePolicyBasedRefCounting()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncRuntimeRefCounting()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncRuntimeRefCounting()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncRuntimeRefCountingOpt()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncRuntimeRefCountingOpt()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateAsyncAsyncToAsyncRuntime()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Async::mlirCreateAsyncAsyncToAsyncRuntime()
      }
    )
  }
}

