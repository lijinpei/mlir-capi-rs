// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::AffineExpr::*;
use crate::IR::*;
use crate::Support::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirIntegerSet {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirIntegerSet = StructMlirIntegerSet;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirIntegerSetGetContext(set: MlirIntegerSet) -> MlirContext;

  pub fn mlirIntegerSetIsNull(set: MlirIntegerSet) -> u8;

  pub fn mlirIntegerSetEqual(s1: MlirIntegerSet, s2: MlirIntegerSet) -> u8;

  pub fn mlirIntegerSetPrint(set: MlirIntegerSet, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirIntegerSetDump(set: MlirIntegerSet) -> ();

  pub fn mlirIntegerSetEmptyGet(context: MlirContext, numDims: std::ffi::c_long, numSymbols: std::ffi::c_long) -> MlirIntegerSet;

  pub fn mlirIntegerSetGet(context: MlirContext, numDims: std::ffi::c_long, numSymbols: std::ffi::c_long, numConstraints: std::ffi::c_long, constraints: *const MlirAffineExpr, eqFlags: *const u8) -> MlirIntegerSet;

  pub fn mlirIntegerSetReplaceGet(set: MlirIntegerSet, dimReplacements: *const MlirAffineExpr, symbolReplacements: *const MlirAffineExpr, numResultDims: std::ffi::c_long, numResultSymbols: std::ffi::c_long) -> MlirIntegerSet;

  pub fn mlirIntegerSetIsCanonicalEmpty(set: MlirIntegerSet) -> u8;

  pub fn mlirIntegerSetGetNumDims(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetNumSymbols(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetNumInputs(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetNumConstraints(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetNumEqualities(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetNumInequalities(set: MlirIntegerSet) -> std::ffi::c_long;

  pub fn mlirIntegerSetGetConstraint(set: MlirIntegerSet, pos: std::ffi::c_long) -> MlirAffineExpr;

  pub fn mlirIntegerSetIsConstraintEq(set: MlirIntegerSet, pos: std::ffi::c_long) -> u8;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirIntegerSetGetContext<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetContext(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerSetIsNull<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetIsNull(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerSetEqual<T0_, T1_>(s1_:  T0_, s2_:  T1_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>,  T1_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetEqual(Into::<MlirIntegerSet>::into(s1_), Into::<MlirIntegerSet>::into(s2_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirIntegerSetPrint<T0_, T1_, T2_>(set_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirIntegerSet>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IntegerSet::mlirIntegerSetPrint(Into::<MlirIntegerSet>::into(set_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirIntegerSetDump<T0_>(set_:  T0_)
  where
     T0_: Into<MlirIntegerSet>
  {
    unsafe {
      crate::IntegerSet::mlirIntegerSetDump(Into::<MlirIntegerSet>::into(set_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIntegerSet> {
  pub unsafe fn mlirIntegerSetEmptyGet<T0_, T1_, T2_>(context_:  T0_, numDims_:  T1_, numSymbols_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetEmptyGet(Into::<MlirContext>::into(context_), Into::<std::ffi::c_long>::into(numDims_), Into::<std::ffi::c_long>::into(numSymbols_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIntegerSet> {
  pub unsafe fn mlirIntegerSetGet<T0_, T1_, T2_, T3_, T4_, T5_>(context_:  T0_, numDims_:  T1_, numSymbols_:  T2_, numConstraints_:  T3_, constraints_:  T4_, eqFlags_:  T5_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*const MlirAffineExpr>,  T5_: Into<*const u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGet(Into::<MlirContext>::into(context_), Into::<std::ffi::c_long>::into(numDims_), Into::<std::ffi::c_long>::into(numSymbols_), Into::<std::ffi::c_long>::into(numConstraints_), Into::<*const MlirAffineExpr>::into(constraints_), Into::<*const u8>::into(eqFlags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIntegerSet> {
  pub unsafe fn mlirIntegerSetReplaceGet<T0_, T1_, T2_, T3_, T4_>(set_:  T0_, dimReplacements_:  T1_, symbolReplacements_:  T2_, numResultDims_:  T3_, numResultSymbols_:  T4_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>,  T1_: Into<*const MlirAffineExpr>,  T2_: Into<*const MlirAffineExpr>,  T3_: Into<std::ffi::c_long>,  T4_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetReplaceGet(Into::<MlirIntegerSet>::into(set_), Into::<*const MlirAffineExpr>::into(dimReplacements_), Into::<*const MlirAffineExpr>::into(symbolReplacements_), Into::<std::ffi::c_long>::into(numResultDims_), Into::<std::ffi::c_long>::into(numResultSymbols_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerSetIsCanonicalEmpty<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetIsCanonicalEmpty(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumDims<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumDims(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumSymbols<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumSymbols(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumInputs<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumInputs(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumConstraints<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumConstraints(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumEqualities<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumEqualities(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirIntegerSetGetNumInequalities<T0_>(set_:  T0_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetNumInequalities(Into::<MlirIntegerSet>::into(set_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirIntegerSetGetConstraint<T0_, T1_>(set_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetGetConstraint(Into::<MlirIntegerSet>::into(set_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIntegerSetIsConstraintEq<T0_, T1_>(set_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirIntegerSet>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IntegerSet::mlirIntegerSetIsConstraintEq(Into::<MlirIntegerSet>::into(set_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

