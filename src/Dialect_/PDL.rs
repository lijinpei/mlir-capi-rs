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

  pub fn mlirGetDialectHandle__pdl__() -> MlirDialectHandle;

  pub fn mlirTypeIsAPDLType(r#type: MlirType) -> u8;

  pub fn mlirTypeIsAPDLAttributeType(r#type: MlirType) -> u8;

  pub fn mlirPDLAttributeTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsAPDLOperationType(r#type: MlirType) -> u8;

  pub fn mlirPDLOperationTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsAPDLRangeType(r#type: MlirType) -> u8;

  pub fn mlirPDLRangeTypeGet(elementType: MlirType) -> MlirType;

  pub fn mlirPDLRangeTypeGetElementType(r#type: MlirType) -> MlirType;

  pub fn mlirTypeIsAPDLTypeType(r#type: MlirType) -> u8;

  pub fn mlirPDLTypeTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirTypeIsAPDLValueType(r#type: MlirType) -> u8;

  pub fn mlirPDLValueTypeGet(ctx: MlirContext) -> MlirType;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__pdl__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirGetDialectHandle__pdl__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLAttributeType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLAttributeType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLAttributeTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLAttributeTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLOperationType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLOperationType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLOperationTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLOperationTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLRangeType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLRangeType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLRangeTypeGet<T0_>(elementType_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLRangeTypeGet(Into::<MlirType>::into(elementType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLRangeTypeGetElementType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLRangeTypeGetElementType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLTypeType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLTypeType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLTypeTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLTypeTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAPDLValueType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirTypeIsAPDLValueType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirPDLValueTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::PDL::mlirPDLValueTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

