// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Support::*;
use crate::Pass::*;
use crate::IR::*;
use crate::Support::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirRegisterTransformsPasses() -> ();

  pub fn mlirCreateTransformsCSE() -> MlirPass;

  pub fn mlirRegisterTransformsCSE() -> ();

  pub fn mlirCreateTransformsCanonicalizer() -> MlirPass;

  pub fn mlirRegisterTransformsCanonicalizer() -> ();

  pub fn mlirCreateTransformsCompositeFixedPointPass() -> MlirPass;

  pub fn mlirRegisterTransformsCompositeFixedPointPass() -> ();

  pub fn mlirCreateTransformsControlFlowSink() -> MlirPass;

  pub fn mlirRegisterTransformsControlFlowSink() -> ();

  pub fn mlirCreateTransformsGenerateRuntimeVerification() -> MlirPass;

  pub fn mlirRegisterTransformsGenerateRuntimeVerification() -> ();

  pub fn mlirCreateTransformsInliner() -> MlirPass;

  pub fn mlirRegisterTransformsInliner() -> ();

  pub fn mlirCreateTransformsLocationSnapshot() -> MlirPass;

  pub fn mlirRegisterTransformsLocationSnapshot() -> ();

  pub fn mlirCreateTransformsLoopInvariantCodeMotion() -> MlirPass;

  pub fn mlirRegisterTransformsLoopInvariantCodeMotion() -> ();

  pub fn mlirCreateTransformsLoopInvariantSubsetHoisting() -> MlirPass;

  pub fn mlirRegisterTransformsLoopInvariantSubsetHoisting() -> ();

  pub fn mlirCreateTransformsMem2Reg() -> MlirPass;

  pub fn mlirRegisterTransformsMem2Reg() -> ();

  pub fn mlirCreateTransformsPrintIRPass() -> MlirPass;

  pub fn mlirRegisterTransformsPrintIRPass() -> ();

  pub fn mlirCreateTransformsPrintOpStats() -> MlirPass;

  pub fn mlirRegisterTransformsPrintOpStats() -> ();

  pub fn mlirCreateTransformsRemoveDeadValues() -> MlirPass;

  pub fn mlirRegisterTransformsRemoveDeadValues() -> ();

  pub fn mlirCreateTransformsSCCP() -> MlirPass;

  pub fn mlirRegisterTransformsSCCP() -> ();

  pub fn mlirCreateTransformsSROA() -> MlirPass;

  pub fn mlirRegisterTransformsSROA() -> ();

  pub fn mlirCreateTransformsStripDebugInfo() -> MlirPass;

  pub fn mlirRegisterTransformsStripDebugInfo() -> ();

  pub fn mlirCreateTransformsSymbolDCE() -> MlirPass;

  pub fn mlirRegisterTransformsSymbolDCE() -> ();

  pub fn mlirCreateTransformsSymbolPrivatize() -> MlirPass;

  pub fn mlirRegisterTransformsSymbolPrivatize() -> ();

  pub fn mlirCreateTransformsTopologicalSort() -> MlirPass;

  pub fn mlirRegisterTransformsTopologicalSort() -> ();

  pub fn mlirCreateTransformsViewOpGraph() -> MlirPass;

  pub fn mlirRegisterTransformsViewOpGraph() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsCSE()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsCSE()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsCanonicalizer()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsCanonicalizer()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsCompositeFixedPointPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsCompositeFixedPointPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsControlFlowSink()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsControlFlowSink()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsGenerateRuntimeVerification()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsGenerateRuntimeVerification()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsInliner()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsInliner()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsLocationSnapshot()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsLocationSnapshot()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsLoopInvariantCodeMotion()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsLoopInvariantCodeMotion()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsLoopInvariantSubsetHoisting()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsLoopInvariantSubsetHoisting()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsMem2Reg()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsMem2Reg()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsPrintIRPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsPrintIRPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsPrintOpStats()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsPrintOpStats()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsRemoveDeadValues()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsRemoveDeadValues()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsSCCP()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsSCCP()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsSROA()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsSROA()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsStripDebugInfo()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsStripDebugInfo()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsSymbolDCE()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsSymbolDCE()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsSymbolPrivatize()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsSymbolPrivatize()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsTopologicalSort()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsTopologicalSort()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateTransformsViewOpGraph()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Transforms::mlirCreateTransformsViewOpGraph()
      }
    )
  }
}

