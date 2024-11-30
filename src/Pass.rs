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
pub struct StructMlirPass {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirPass = StructMlirPass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirExternalPass {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirExternalPass = StructMlirExternalPass;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirPassManager {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirPassManager = StructMlirPassManager;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirOpPassManager {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirOpPassManager = StructMlirOpPassManager;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirExternalPassCallbacks {
  pub construct: *mut extern fn (*mut std::ffi::c_void) -> (),
  pub destruct: *mut extern fn (*mut std::ffi::c_void) -> (),
  pub initialize: *mut extern fn (MlirContext, *mut std::ffi::c_void) -> MlirLogicalResult,
  pub clone: *mut extern fn (*mut std::ffi::c_void) -> *mut std::ffi::c_void,
  pub run: *mut extern fn (MlirOperation, MlirExternalPass, *mut std::ffi::c_void) -> (),
}
pub type MlirExternalPassCallbacks = StructMlirExternalPassCallbacks;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirPassManagerCreate(ctx: MlirContext) -> MlirPassManager;

  pub fn mlirPassManagerCreateOnOperation(ctx: MlirContext, anchorOp: MlirStringRef) -> MlirPassManager;

  pub fn mlirPassManagerDestroy(passManager: MlirPassManager) -> ();

  pub fn mlirPassManagerIsNull(passManager: MlirPassManager) -> u8;

  pub fn mlirPassManagerGetAsOpPassManager(passManager: MlirPassManager) -> MlirOpPassManager;

  pub fn mlirPassManagerRunOnOp(passManager: MlirPassManager, op: MlirOperation) -> MlirLogicalResult;

  pub fn mlirPassManagerEnableIRPrinting(passManager: MlirPassManager, printBeforeAll: u8, printAfterAll: u8, printModuleScope: u8, printAfterOnlyOnChange: u8, printAfterOnlyOnFailure: u8, flags: MlirOpPrintingFlags, treePrintingPath: MlirStringRef) -> ();

  pub fn mlirPassManagerEnableVerifier(passManager: MlirPassManager, enable: u8) -> ();

  pub fn mlirPassManagerGetNestedUnder(passManager: MlirPassManager, operationName: MlirStringRef) -> MlirOpPassManager;

  pub fn mlirOpPassManagerGetNestedUnder(passManager: MlirOpPassManager, operationName: MlirStringRef) -> MlirOpPassManager;

  pub fn mlirPassManagerAddOwnedPass(passManager: MlirPassManager, pass: MlirPass) -> ();

  pub fn mlirOpPassManagerAddOwnedPass(passManager: MlirOpPassManager, pass: MlirPass) -> ();

  pub fn mlirOpPassManagerAddPipeline(passManager: MlirOpPassManager, pipelineElements: MlirStringRef, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> MlirLogicalResult;

  pub fn mlirPrintPassPipeline(passManager: MlirOpPassManager, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirParsePassPipeline(passManager: MlirOpPassManager, pipeline: MlirStringRef, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> MlirLogicalResult;

  pub fn mlirCreateExternalPass(passID: MlirTypeID, name: MlirStringRef, argument: MlirStringRef, description: MlirStringRef, opName: MlirStringRef, nDependentDialects: std::ffi::c_long, dependentDialects: *mut MlirDialectHandle, callbacks: MlirExternalPassCallbacks, userData: *mut std::ffi::c_void) -> MlirPass;

  pub fn mlirExternalPassSignalFailure(pass: MlirExternalPass) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPassManager> {
  pub unsafe fn mlirPassManagerCreate<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerCreate(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPassManager> {
  pub unsafe fn mlirPassManagerCreateOnOperation<T0_, T1_>(ctx_:  T0_, anchorOp_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerCreateOnOperation(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(anchorOp_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPassManagerDestroy<T0_>(passManager_:  T0_)
  where
     T0_: Into<MlirPassManager>
  {
    unsafe {
      crate::Pass::mlirPassManagerDestroy(Into::<MlirPassManager>::into(passManager_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirPassManagerIsNull<T0_>(passManager_:  T0_)-> Tret_
  where
     T0_: Into<MlirPassManager>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerIsNull(Into::<MlirPassManager>::into(passManager_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpPassManager> {
  pub unsafe fn mlirPassManagerGetAsOpPassManager<T0_>(passManager_:  T0_)-> Tret_
  where
     T0_: Into<MlirPassManager>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerGetAsOpPassManager(Into::<MlirPassManager>::into(passManager_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirPassManagerRunOnOp<T0_, T1_>(passManager_:  T0_, op_:  T1_)-> Tret_
  where
     T0_: Into<MlirPassManager>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerRunOnOp(Into::<MlirPassManager>::into(passManager_), Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPassManagerEnableIRPrinting<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(passManager_:  T0_, printBeforeAll_:  T1_, printAfterAll_:  T2_, printModuleScope_:  T3_, printAfterOnlyOnChange_:  T4_, printAfterOnlyOnFailure_:  T5_, flags_:  T6_, treePrintingPath_:  T7_)
  where
     T0_: Into<MlirPassManager>,  T1_: Into<u8>,  T2_: Into<u8>,  T3_: Into<u8>,  T4_: Into<u8>,  T5_: Into<u8>,  T6_: Into<MlirOpPrintingFlags>,  T7_: Into<MlirStringRef>
  {
    unsafe {
      crate::Pass::mlirPassManagerEnableIRPrinting(Into::<MlirPassManager>::into(passManager_), Into::<u8>::into(printBeforeAll_), Into::<u8>::into(printAfterAll_), Into::<u8>::into(printModuleScope_), Into::<u8>::into(printAfterOnlyOnChange_), Into::<u8>::into(printAfterOnlyOnFailure_), Into::<MlirOpPrintingFlags>::into(flags_), Into::<MlirStringRef>::into(treePrintingPath_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPassManagerEnableVerifier<T0_, T1_>(passManager_:  T0_, enable_:  T1_)
  where
     T0_: Into<MlirPassManager>,  T1_: Into<u8>
  {
    unsafe {
      crate::Pass::mlirPassManagerEnableVerifier(Into::<MlirPassManager>::into(passManager_), Into::<u8>::into(enable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpPassManager> {
  pub unsafe fn mlirPassManagerGetNestedUnder<T0_, T1_>(passManager_:  T0_, operationName_:  T1_)-> Tret_
  where
     T0_: Into<MlirPassManager>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirPassManagerGetNestedUnder(Into::<MlirPassManager>::into(passManager_), Into::<MlirStringRef>::into(operationName_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpPassManager> {
  pub unsafe fn mlirOpPassManagerGetNestedUnder<T0_, T1_>(passManager_:  T0_, operationName_:  T1_)-> Tret_
  where
     T0_: Into<MlirOpPassManager>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirOpPassManagerGetNestedUnder(Into::<MlirOpPassManager>::into(passManager_), Into::<MlirStringRef>::into(operationName_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPassManagerAddOwnedPass<T0_, T1_>(passManager_:  T0_, pass_:  T1_)
  where
     T0_: Into<MlirPassManager>,  T1_: Into<MlirPass>
  {
    unsafe {
      crate::Pass::mlirPassManagerAddOwnedPass(Into::<MlirPassManager>::into(passManager_), Into::<MlirPass>::into(pass_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPassManagerAddOwnedPass<T0_, T1_>(passManager_:  T0_, pass_:  T1_)
  where
     T0_: Into<MlirOpPassManager>,  T1_: Into<MlirPass>
  {
    unsafe {
      crate::Pass::mlirOpPassManagerAddOwnedPass(Into::<MlirOpPassManager>::into(passManager_), Into::<MlirPass>::into(pass_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirOpPassManagerAddPipeline<T0_, T1_, T2_, T3_>(passManager_:  T0_, pipelineElements_:  T1_, callback_:  T2_, userData_:  T3_)-> Tret_
  where
     T0_: Into<MlirOpPassManager>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirOpPassManagerAddPipeline(Into::<MlirOpPassManager>::into(passManager_), Into::<MlirStringRef>::into(pipelineElements_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPrintPassPipeline<T0_, T1_, T2_>(passManager_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirOpPassManager>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::Pass::mlirPrintPassPipeline(Into::<MlirOpPassManager>::into(passManager_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirParsePassPipeline<T0_, T1_, T2_, T3_>(passManager_:  T0_, pipeline_:  T1_, callback_:  T2_, userData_:  T3_)-> Tret_
  where
     T0_: Into<MlirOpPassManager>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirParsePassPipeline(Into::<MlirOpPassManager>::into(passManager_), Into::<MlirStringRef>::into(pipeline_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateExternalPass<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(passID_:  T0_, name_:  T1_, argument_:  T2_, description_:  T3_, opName_:  T4_, nDependentDialects_:  T5_, dependentDialects_:  T6_, callbacks_:  T7_, userData_:  T8_)-> Tret_
  where
     T0_: Into<MlirTypeID>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirStringRef>,  T3_: Into<MlirStringRef>,  T4_: Into<MlirStringRef>,  T5_: Into<std::ffi::c_long>,  T6_: Into<*mut MlirDialectHandle>,  T7_: Into<MlirExternalPassCallbacks>,  T8_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Pass::mlirCreateExternalPass(Into::<MlirTypeID>::into(passID_), Into::<MlirStringRef>::into(name_), Into::<MlirStringRef>::into(argument_), Into::<MlirStringRef>::into(description_), Into::<MlirStringRef>::into(opName_), Into::<std::ffi::c_long>::into(nDependentDialects_), Into::<*mut MlirDialectHandle>::into(dependentDialects_), Into::<MlirExternalPassCallbacks>::into(callbacks_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirExternalPassSignalFailure<T0_>(pass_:  T0_)
  where
     T0_: Into<MlirExternalPass>
  {
    unsafe {
      crate::Pass::mlirExternalPassSignalFailure(Into::<MlirExternalPass>::into(pass_))
    }
  }
}

