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
pub struct StructMlirDiagnostic {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirDiagnostic = StructMlirDiagnostic;

pub const MlirDiagnosticError: std::ffi::c_uint = 0;
pub const MlirDiagnosticWarning: std::ffi::c_uint = 1;
pub const MlirDiagnosticNote: std::ffi::c_uint = 2;
pub const MlirDiagnosticRemark: std::ffi::c_uint = 3;
pub type EnumMlirDiagnosticSeverity = std::ffi::c_uint;
pub type MlirDiagnosticSeverity = EnumMlirDiagnosticSeverity;
pub type MlirDiagnosticHandlerID = std::ffi::c_ulong;
pub type MlirDiagnosticHandler = *mut extern fn (MlirDiagnostic, *mut std::ffi::c_void) -> MlirLogicalResult;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirDiagnosticPrint(diagnostic: MlirDiagnostic, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirDiagnosticGetLocation(diagnostic: MlirDiagnostic) -> MlirLocation;

  pub fn mlirDiagnosticGetSeverity(diagnostic: MlirDiagnostic) -> EnumMlirDiagnosticSeverity;

  pub fn mlirDiagnosticGetNumNotes(diagnostic: MlirDiagnostic) -> std::ffi::c_long;

  pub fn mlirDiagnosticGetNote(diagnostic: MlirDiagnostic, pos: std::ffi::c_long) -> MlirDiagnostic;

  pub fn mlirContextAttachDiagnosticHandler(context: MlirContext, handler: MlirDiagnosticHandler, userData: *mut std::ffi::c_void, deleteUserData: *mut extern fn (*mut std::ffi::c_void) -> ()) -> std::ffi::c_ulong;

  pub fn mlirContextDetachDiagnosticHandler(context: MlirContext, id: std::ffi::c_ulong) -> ();

  pub fn mlirEmitError(location: MlirLocation, message: *const std::ffi::c_char) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn mlirDiagnosticPrint<T0_, T1_, T2_>(diagnostic_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirDiagnostic>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Diagnostics::mlirDiagnosticPrint(Into::<MlirDiagnostic>::into(diagnostic_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirDiagnosticGetLocation<T0_>(diagnostic_:  T0_)-> Tret_
  where
     T0_: Into<MlirDiagnostic>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Diagnostics::mlirDiagnosticGetLocation(Into::<MlirDiagnostic>::into(diagnostic_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<EnumMlirDiagnosticSeverity> {
  pub unsafe fn mlirDiagnosticGetSeverity<T0_>(diagnostic_:  T0_)-> Tret_
  where
     T0_: Into<MlirDiagnostic>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Diagnostics::mlirDiagnosticGetSeverity(Into::<MlirDiagnostic>::into(diagnostic_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirDiagnosticGetNumNotes<T0_>(diagnostic_:  T0_)-> Tret_
  where
     T0_: Into<MlirDiagnostic>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Diagnostics::mlirDiagnosticGetNumNotes(Into::<MlirDiagnostic>::into(diagnostic_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDiagnostic> {
  pub unsafe fn mlirDiagnosticGetNote<T0_, T1_>(diagnostic_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirDiagnostic>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Diagnostics::mlirDiagnosticGetNote(Into::<MlirDiagnostic>::into(diagnostic_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirContextAttachDiagnosticHandler<T0_, T1_, T2_, T3_>(context_:  T0_, handler_:  T1_, userData_:  T2_, deleteUserData_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirDiagnosticHandler>,  T2_: Into<*mut std::ffi::c_void>,  T3_: Into<*mut extern fn (*mut std::ffi::c_void) -> ()>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Diagnostics::mlirContextAttachDiagnosticHandler(Into::<MlirContext>::into(context_), Into::<MlirDiagnosticHandler>::into(handler_), Into::<*mut std::ffi::c_void>::into(userData_), Into::<*mut extern fn (*mut std::ffi::c_void) -> ()>::into(deleteUserData_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextDetachDiagnosticHandler<T0_, T1_>(context_:  T0_, id_:  T1_)
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_ulong>
  {
    unsafe {
      crate::Diagnostics::mlirContextDetachDiagnosticHandler(Into::<MlirContext>::into(context_), Into::<std::ffi::c_ulong>::into(id_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirEmitError<T0_, T1_>(location_:  T0_, message_:  T1_)
  where
     T0_: Into<MlirLocation>,  T1_: Into<*const std::ffi::c_char>
  {
    unsafe {
      crate::Diagnostics::mlirEmitError(Into::<MlirLocation>::into(location_), Into::<*const std::ffi::c_char>::into(message_))
    }
  }
}

