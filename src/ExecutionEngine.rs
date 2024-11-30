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
pub struct StructMlirExecutionEngine {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirExecutionEngine = StructMlirExecutionEngine;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirExecutionEngineCreate(op: MlirModule, optLevel: std::ffi::c_int, numPaths: std::ffi::c_int, sharedLibPaths: *const MlirStringRef, enableObjectDump: u8) -> MlirExecutionEngine;

  pub fn mlirExecutionEngineDestroy(jit: MlirExecutionEngine) -> ();

  pub fn mlirExecutionEngineIsNull(jit: MlirExecutionEngine) -> u8;

  pub fn mlirExecutionEngineInvokePacked(jit: MlirExecutionEngine, name: MlirStringRef, arguments: *mut *mut std::ffi::c_void) -> MlirLogicalResult;

  pub fn mlirExecutionEngineLookupPacked(jit: MlirExecutionEngine, name: MlirStringRef) -> *mut std::ffi::c_void;

  pub fn mlirExecutionEngineLookup(jit: MlirExecutionEngine, name: MlirStringRef) -> *mut std::ffi::c_void;

  pub fn mlirExecutionEngineRegisterSymbol(jit: MlirExecutionEngine, name: MlirStringRef, sym: *mut std::ffi::c_void) -> ();

  pub fn mlirExecutionEngineDumpToObjectFile(jit: MlirExecutionEngine, fileName: MlirStringRef) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirExecutionEngine> {
  pub unsafe fn mlirExecutionEngineCreate<T0_, T1_, T2_, T3_, T4_>(op_:  T0_, optLevel_:  T1_, numPaths_:  T2_, sharedLibPaths_:  T3_, enableObjectDump_:  T4_)-> Tret_
  where
     T0_: Into<MlirModule>,  T1_: Into<std::ffi::c_int>,  T2_: Into<std::ffi::c_int>,  T3_: Into<*const MlirStringRef>,  T4_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::mlirExecutionEngineCreate(Into::<MlirModule>::into(op_), Into::<std::ffi::c_int>::into(optLevel_), Into::<std::ffi::c_int>::into(numPaths_), Into::<*const MlirStringRef>::into(sharedLibPaths_), Into::<u8>::into(enableObjectDump_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirExecutionEngineDestroy<T0_>(jit_:  T0_)
  where
     T0_: Into<MlirExecutionEngine>
  {
    unsafe {
      crate::ExecutionEngine::mlirExecutionEngineDestroy(Into::<MlirExecutionEngine>::into(jit_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirExecutionEngineIsNull<T0_>(jit_:  T0_)-> Tret_
  where
     T0_: Into<MlirExecutionEngine>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::mlirExecutionEngineIsNull(Into::<MlirExecutionEngine>::into(jit_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirExecutionEngineInvokePacked<T0_, T1_, T2_>(jit_:  T0_, name_:  T1_, arguments_:  T2_)-> Tret_
  where
     T0_: Into<MlirExecutionEngine>,  T1_: Into<MlirStringRef>,  T2_: Into<*mut *mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::mlirExecutionEngineInvokePacked(Into::<MlirExecutionEngine>::into(jit_), Into::<MlirStringRef>::into(name_), Into::<*mut *mut std::ffi::c_void>::into(arguments_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn mlirExecutionEngineLookupPacked<T0_, T1_>(jit_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirExecutionEngine>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::mlirExecutionEngineLookupPacked(Into::<MlirExecutionEngine>::into(jit_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<*mut std::ffi::c_void> {
  pub unsafe fn mlirExecutionEngineLookup<T0_, T1_>(jit_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirExecutionEngine>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::ExecutionEngine::mlirExecutionEngineLookup(Into::<MlirExecutionEngine>::into(jit_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirExecutionEngineRegisterSymbol<T0_, T1_, T2_>(jit_:  T0_, name_:  T1_, sym_:  T2_)
  where
     T0_: Into<MlirExecutionEngine>,  T1_: Into<MlirStringRef>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::ExecutionEngine::mlirExecutionEngineRegisterSymbol(Into::<MlirExecutionEngine>::into(jit_), Into::<MlirStringRef>::into(name_), Into::<*mut std::ffi::c_void>::into(sym_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirExecutionEngineDumpToObjectFile<T0_, T1_>(jit_:  T0_, fileName_:  T1_)
  where
     T0_: Into<MlirExecutionEngine>,  T1_: Into<MlirStringRef>
  {
    unsafe {
      crate::ExecutionEngine::mlirExecutionEngineDumpToObjectFile(Into::<MlirExecutionEngine>::into(jit_), Into::<MlirStringRef>::into(fileName_))
    }
  }
}

