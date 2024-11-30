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

  pub fn mlirLinalgFillBuiltinNamedOpRegion(mlirOp: MlirOperation) -> ();

  pub fn mlirGetDialectHandle__linalg__() -> MlirDialectHandle;

  pub fn mlirRegisterLinalgPasses() -> ();

  pub fn mlirCreateLinalgConvertElementwiseToLinalgPass() -> MlirPass;

  pub fn mlirRegisterLinalgConvertElementwiseToLinalgPass() -> ();

  pub fn mlirCreateLinalgConvertLinalgToAffineLoopsPass() -> MlirPass;

  pub fn mlirRegisterLinalgConvertLinalgToAffineLoopsPass() -> ();

  pub fn mlirCreateLinalgConvertLinalgToLoopsPass() -> MlirPass;

  pub fn mlirRegisterLinalgConvertLinalgToLoopsPass() -> ();

  pub fn mlirCreateLinalgConvertLinalgToParallelLoopsPass() -> MlirPass;

  pub fn mlirRegisterLinalgConvertLinalgToParallelLoopsPass() -> ();

  pub fn mlirCreateLinalgLinalgBlockPackMatmul() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgBlockPackMatmul() -> ();

  pub fn mlirCreateLinalgLinalgDetensorizePass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgDetensorizePass() -> ();

  pub fn mlirCreateLinalgLinalgElementwiseOpFusionPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgElementwiseOpFusionPass() -> ();

  pub fn mlirCreateLinalgLinalgFoldUnitExtentDimsPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgFoldUnitExtentDimsPass() -> ();

  pub fn mlirCreateLinalgLinalgGeneralizeNamedOpsPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgGeneralizeNamedOpsPass() -> ();

  pub fn mlirCreateLinalgLinalgInlineScalarOperandsPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgInlineScalarOperandsPass() -> ();

  pub fn mlirCreateLinalgLinalgNamedOpConversionPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgNamedOpConversionPass() -> ();

  pub fn mlirCreateLinalgLinalgSpecializeGenericOpsPass() -> MlirPass;

  pub fn mlirRegisterLinalgLinalgSpecializeGenericOpsPass() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl FFIVoid_ {
  pub unsafe fn mlirLinalgFillBuiltinNamedOpRegion<T0_>(mlirOp_:  T0_)
  where
     T0_: Into<MlirOperation>
  {
    unsafe {
      crate::Dialect_::Linalg::mlirLinalgFillBuiltinNamedOpRegion(Into::<MlirOperation>::into(mlirOp_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__linalg__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirGetDialectHandle__linalg__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgConvertElementwiseToLinalgPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgConvertElementwiseToLinalgPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgConvertLinalgToAffineLoopsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgConvertLinalgToAffineLoopsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgConvertLinalgToLoopsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgConvertLinalgToLoopsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgConvertLinalgToParallelLoopsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgConvertLinalgToParallelLoopsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgBlockPackMatmul()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgBlockPackMatmul()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgDetensorizePass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgDetensorizePass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgElementwiseOpFusionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgElementwiseOpFusionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgFoldUnitExtentDimsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgFoldUnitExtentDimsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgGeneralizeNamedOpsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgGeneralizeNamedOpsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgInlineScalarOperandsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgInlineScalarOperandsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgNamedOpConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgNamedOpConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateLinalgLinalgSpecializeGenericOpsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::Linalg::mlirCreateLinalgLinalgSpecializeGenericOpsPass()
      }
    )
  }
}

