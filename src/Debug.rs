// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirEnableGlobalDebug(enable: u8) -> ();

  pub fn mlirIsGlobalDebugEnabled() -> u8;

  pub fn mlirSetGlobalDebugType(r#type: *const std::ffi::c_char) -> ();

  pub fn mlirSetGlobalDebugTypes(types: *mut *const std::ffi::c_char, n: std::ffi::c_long) -> ();

  pub fn mlirIsCurrentDebugType(r#type: *const std::ffi::c_char) -> u8;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn mlirEnableGlobalDebug<T0_>(enable_:  T0_)
  where
     T0_: Into<u8>
  {
    unsafe {
      crate::Debug::mlirEnableGlobalDebug(Into::<u8>::into(enable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIsGlobalDebugEnabled()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Debug::mlirIsGlobalDebugEnabled()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirSetGlobalDebugType<T0_>(type_:  T0_)
  where
     T0_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Debug::mlirSetGlobalDebugType(Into::<*const std::ffi::c_char>::into(type_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirSetGlobalDebugTypes<T0_, T1_>(types_:  T0_, n_:  T1_)
  where
     T0_: Into<*mut *const std::ffi::c_char>,  T1_: Into<std::ffi::c_long>
  {
    unsafe {
      crate::Debug::mlirSetGlobalDebugTypes(Into::<*mut *const std::ffi::c_char>::into(types_), Into::<std::ffi::c_long>::into(n_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIsCurrentDebugType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Debug::mlirIsCurrentDebugType(Into::<*const std::ffi::c_char>::into(type_))
      }
    )
  }
}

