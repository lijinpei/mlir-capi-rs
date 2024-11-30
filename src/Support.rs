// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirLlvmThreadPool {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirLlvmThreadPool = StructMlirLlvmThreadPool;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirTypeID {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirTypeID = StructMlirTypeID;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirTypeIDAllocator {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirTypeIDAllocator = StructMlirTypeIDAllocator;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirStringRef {
  pub data: *const std::ffi::c_char,
  pub length: std::ffi::c_ulong,
}
pub type MlirStringRef = StructMlirStringRef;
pub type MlirStringCallback = *mut extern fn (MlirStringRef, *mut std::ffi::c_void) -> ();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirLogicalResult {
  pub value: std::ffi::c_schar,
}
pub type MlirLogicalResult = StructMlirLogicalResult;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirStringRefCreate(str: *const std::ffi::c_char, length: std::ffi::c_ulong) -> MlirStringRef;

  pub fn mlirStringRefCreateFromCString(str: *const std::ffi::c_char) -> MlirStringRef;

  pub fn mlirStringRefEqual(string: MlirStringRef, other: MlirStringRef) -> u8;

  pub fn mlirLogicalResultIsSuccess(res: MlirLogicalResult) -> u8;

  pub fn mlirLogicalResultIsFailure(res: MlirLogicalResult) -> u8;

  pub fn mlirLogicalResultSuccess() -> MlirLogicalResult;

  pub fn mlirLogicalResultFailure() -> MlirLogicalResult;

  pub fn mlirLlvmThreadPoolCreate() -> MlirLlvmThreadPool;

  pub fn mlirLlvmThreadPoolDestroy(pool: MlirLlvmThreadPool) -> ();

  pub fn mlirTypeIDCreate(ptr: *const std::ffi::c_void) -> MlirTypeID;

  pub fn mlirTypeIDIsNull(typeID: MlirTypeID) -> u8;

  pub fn mlirTypeIDEqual(typeID1: MlirTypeID, typeID2: MlirTypeID) -> u8;

  pub fn mlirTypeIDHashValue(typeID: MlirTypeID) -> std::ffi::c_ulong;

  pub fn mlirTypeIDAllocatorCreate() -> MlirTypeIDAllocator;

  pub fn mlirTypeIDAllocatorDestroy(allocator: MlirTypeIDAllocator) -> ();

  pub fn mlirTypeIDAllocatorAllocateTypeID(allocator: MlirTypeIDAllocator) -> MlirTypeID;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirStringRefCreate<T0_, T1_>(str_:  T0_, length_:  T1_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirStringRefCreate(Into::<*const std::ffi::c_char>::into(str_), Into::<std::ffi::c_ulong>::into(length_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirStringRefCreateFromCString<T0_>(str_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_char>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirStringRefCreateFromCString(Into::<*const std::ffi::c_char>::into(str_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirStringRefEqual<T0_, T1_>(string_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirStringRefEqual(Into::<MlirStringRef>::into(string_), Into::<MlirStringRef>::into(other_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLogicalResultIsSuccess<T0_>(res_:  T0_)-> Tret_
  where
     T0_: Into<MlirLogicalResult>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirLogicalResultIsSuccess(Into::<MlirLogicalResult>::into(res_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLogicalResultIsFailure<T0_>(res_:  T0_)-> Tret_
  where
     T0_: Into<MlirLogicalResult>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirLogicalResultIsFailure(Into::<MlirLogicalResult>::into(res_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirLogicalResultSuccess()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirLogicalResultSuccess()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirLogicalResultFailure()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirLogicalResultFailure()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLlvmThreadPool> {
  pub unsafe fn mlirLlvmThreadPoolCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirLlvmThreadPoolCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirLlvmThreadPoolDestroy<T0_>(pool_:  T0_)
  where
     T0_: Into<MlirLlvmThreadPool>
  {
    unsafe {
      crate::Support::mlirLlvmThreadPoolDestroy(Into::<MlirLlvmThreadPool>::into(pool_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTypeIDCreate<T0_>(ptr_:  T0_)-> Tret_
  where
     T0_: Into<*const std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDCreate(Into::<*const std::ffi::c_void>::into(ptr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIDIsNull<T0_>(typeID_:  T0_)-> Tret_
  where
     T0_: Into<MlirTypeID>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDIsNull(Into::<MlirTypeID>::into(typeID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIDEqual<T0_, T1_>(typeID1_:  T0_, typeID2_:  T1_)-> Tret_
  where
     T0_: Into<MlirTypeID>,  T1_: Into<MlirTypeID>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDEqual(Into::<MlirTypeID>::into(typeID1_), Into::<MlirTypeID>::into(typeID2_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirTypeIDHashValue<T0_>(typeID_:  T0_)-> Tret_
  where
     T0_: Into<MlirTypeID>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDHashValue(Into::<MlirTypeID>::into(typeID_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeIDAllocator> {
  pub unsafe fn mlirTypeIDAllocatorCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDAllocatorCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTypeIDAllocatorDestroy<T0_>(allocator_:  T0_)
  where
     T0_: Into<MlirTypeIDAllocator>
  {
    unsafe {
      crate::Support::mlirTypeIDAllocatorDestroy(Into::<MlirTypeIDAllocator>::into(allocator_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTypeIDAllocatorAllocateTypeID<T0_>(allocator_:  T0_)-> Tret_
  where
     T0_: Into<MlirTypeIDAllocator>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Support::mlirTypeIDAllocatorAllocateTypeID(Into::<MlirTypeIDAllocator>::into(allocator_))
      }
    )
  }
}

