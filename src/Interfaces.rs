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
pub type MlirTypesCallback = *mut extern fn (std::ffi::c_long, *mut MlirType, *mut std::ffi::c_void) -> ();
pub type MlirShapedTypeComponentsCallback = *mut extern fn (u8, std::ffi::c_long, *const std::ffi::c_long, MlirType, MlirAttribute, *mut std::ffi::c_void) -> ();

#[link(name = "MLIR-C")]
extern {

  pub fn mlirOperationImplementsInterface(operation: MlirOperation, interfaceTypeID: MlirTypeID) -> u8;

  pub fn mlirOperationImplementsInterfaceStatic(operationName: MlirStringRef, context: MlirContext, interfaceTypeID: MlirTypeID) -> u8;

  pub fn mlirInferTypeOpInterfaceTypeID() -> MlirTypeID;

  pub fn mlirInferTypeOpInterfaceInferReturnTypes(opName: MlirStringRef, context: MlirContext, location: MlirLocation, nOperands: std::ffi::c_long, operands: *mut MlirValue, attributes: MlirAttribute, properties: *mut std::ffi::c_void, nRegions: std::ffi::c_long, regions: *mut MlirRegion, callback: MlirTypesCallback, userData: *mut std::ffi::c_void) -> MlirLogicalResult;

  pub fn mlirInferShapedTypeOpInterfaceTypeID() -> MlirTypeID;

  pub fn mlirInferShapedTypeOpInterfaceInferReturnTypes(opName: MlirStringRef, context: MlirContext, location: MlirLocation, nOperands: std::ffi::c_long, operands: *mut MlirValue, attributes: MlirAttribute, properties: *mut std::ffi::c_void, nRegions: std::ffi::c_long, regions: *mut MlirRegion, callback: MlirShapedTypeComponentsCallback, userData: *mut std::ffi::c_void) -> MlirLogicalResult;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationImplementsInterface<T0_, T1_>(operation_:  T0_, interfaceTypeID_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirTypeID>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirOperationImplementsInterface(Into::<MlirOperation>::into(operation_), Into::<MlirTypeID>::into(interfaceTypeID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationImplementsInterfaceStatic<T0_, T1_, T2_>(operationName_:  T0_, context_:  T1_, interfaceTypeID_:  T2_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirContext>,  T2_: Into<MlirTypeID>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirOperationImplementsInterfaceStatic(Into::<MlirStringRef>::into(operationName_), Into::<MlirContext>::into(context_), Into::<MlirTypeID>::into(interfaceTypeID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirInferTypeOpInterfaceTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirInferTypeOpInterfaceTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirInferTypeOpInterfaceInferReturnTypes<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(opName_:  T0_, context_:  T1_, location_:  T2_, nOperands_:  T3_, operands_:  T4_, attributes_:  T5_, properties_:  T6_, nRegions_:  T7_, regions_:  T8_, callback_:  T9_, userData_:  T10_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirContext>,  T2_: Into<MlirLocation>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*mut MlirValue>,  T5_: Into<MlirAttribute>,  T6_: Into<*mut std::ffi::c_void>,  T7_: Into<std::ffi::c_long>,  T8_: Into<*mut MlirRegion>,  T9_: Into<MlirTypesCallback>,  T10_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirInferTypeOpInterfaceInferReturnTypes(Into::<MlirStringRef>::into(opName_), Into::<MlirContext>::into(context_), Into::<MlirLocation>::into(location_), Into::<std::ffi::c_long>::into(nOperands_), Into::<*mut MlirValue>::into(operands_), Into::<MlirAttribute>::into(attributes_), Into::<*mut std::ffi::c_void>::into(properties_), Into::<std::ffi::c_long>::into(nRegions_), Into::<*mut MlirRegion>::into(regions_), Into::<MlirTypesCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirInferShapedTypeOpInterfaceTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirInferShapedTypeOpInterfaceTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirInferShapedTypeOpInterfaceInferReturnTypes<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_>(opName_:  T0_, context_:  T1_, location_:  T2_, nOperands_:  T3_, operands_:  T4_, attributes_:  T5_, properties_:  T6_, nRegions_:  T7_, regions_:  T8_, callback_:  T9_, userData_:  T10_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirContext>,  T2_: Into<MlirLocation>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*mut MlirValue>,  T5_: Into<MlirAttribute>,  T6_: Into<*mut std::ffi::c_void>,  T7_: Into<std::ffi::c_long>,  T8_: Into<*mut MlirRegion>,  T9_: Into<MlirShapedTypeComponentsCallback>,  T10_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Interfaces::mlirInferShapedTypeOpInterfaceInferReturnTypes(Into::<MlirStringRef>::into(opName_), Into::<MlirContext>::into(context_), Into::<MlirLocation>::into(location_), Into::<std::ffi::c_long>::into(nOperands_), Into::<*mut MlirValue>::into(operands_), Into::<MlirAttribute>::into(attributes_), Into::<*mut std::ffi::c_void>::into(properties_), Into::<std::ffi::c_long>::into(nRegions_), Into::<*mut MlirRegion>::into(regions_), Into::<MlirShapedTypeComponentsCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

