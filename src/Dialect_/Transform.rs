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

  pub fn mlirGetDialectHandle__transform__() -> MlirDialectHandle;

  pub fn mlirTypeIsATransformAnyOpType(r#type: MlirType) -> u8;

  pub fn mlirTransformAnyOpTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTransformAnyOpTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsATransformAnyParamType(r#type: MlirType) -> u8;

  pub fn mlirTransformAnyParamTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTransformAnyParamTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsATransformAnyValueType(r#type: MlirType) -> u8;

  pub fn mlirTransformAnyValueTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTransformAnyValueTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsATransformOperationType(r#type: MlirType) -> u8;

  pub fn mlirTransformOperationTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTransformOperationTypeGet(ctx: MlirContext, operationName: MlirStringRef) -> MlirType;

  pub fn mlirTransformOperationTypeGetOperationName(r#type: MlirType) -> MlirStringRef;

  pub fn mlirTypeIsATransformParamType(r#type: MlirType) -> u8;

  pub fn mlirTransformParamTypeGetTypeID() -> MlirTypeID;

  pub fn mlirTransformParamTypeGet(ctx: MlirContext, r#type: MlirType) -> MlirType;

  pub fn mlirTransformParamTypeGetType(r#type: MlirType) -> MlirType;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__transform__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirGetDialectHandle__transform__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATransformAnyOpType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTypeIsATransformAnyOpType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTransformAnyOpTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyOpTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformAnyOpTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyOpTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATransformAnyParamType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTypeIsATransformAnyParamType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTransformAnyParamTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyParamTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformAnyParamTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyParamTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATransformAnyValueType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTypeIsATransformAnyValueType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTransformAnyValueTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyValueTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformAnyValueTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformAnyValueTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATransformOperationType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTypeIsATransformOperationType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTransformOperationTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformOperationTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformOperationTypeGet<T0_, T1_>(ctx_:  T0_, operationName_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformOperationTypeGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(operationName_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirTransformOperationTypeGetOperationName<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformOperationTypeGetOperationName(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsATransformParamType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTypeIsATransformParamType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTransformParamTypeGetTypeID()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformParamTypeGetTypeID()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformParamTypeGet<T0_, T1_>(ctx_:  T0_, type_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformParamTypeGet(Into::<MlirContext>::into(ctx_), Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTransformParamTypeGetType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Transform::mlirTransformParamTypeGetType(Into::<MlirType>::into(type_))
      }
    )
  }
}

