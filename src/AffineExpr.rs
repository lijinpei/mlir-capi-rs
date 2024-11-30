// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::AffineMap::*;
use crate::IR::*;
use crate::Support::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirAffineExpr {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirAffineExpr = StructMlirAffineExpr;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirAffineExprGetContext(affineExpr: MlirAffineExpr) -> MlirContext;

  pub fn mlirAffineExprEqual(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> u8;

  pub fn mlirAffineExprIsNull(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineExprPrint(affineExpr: MlirAffineExpr, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirAffineExprDump(affineExpr: MlirAffineExpr) -> ();

  pub fn mlirAffineExprIsSymbolicOrConstant(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineExprIsPureAffine(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineExprGetLargestKnownDivisor(affineExpr: MlirAffineExpr) -> std::ffi::c_long;

  pub fn mlirAffineExprIsMultipleOf(affineExpr: MlirAffineExpr, factor: std::ffi::c_long) -> u8;

  pub fn mlirAffineExprIsFunctionOfDim(affineExpr: MlirAffineExpr, position: std::ffi::c_long) -> u8;

  pub fn mlirAffineExprCompose(affineExpr: MlirAffineExpr, affineMap: StructMlirAffineMap) -> MlirAffineExpr;

  pub fn mlirAffineExprIsADim(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineDimExprGet(ctx: MlirContext, position: std::ffi::c_long) -> MlirAffineExpr;

  pub fn mlirAffineDimExprGetPosition(affineExpr: MlirAffineExpr) -> std::ffi::c_long;

  pub fn mlirAffineExprIsASymbol(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineSymbolExprGet(ctx: MlirContext, position: std::ffi::c_long) -> MlirAffineExpr;

  pub fn mlirAffineSymbolExprGetPosition(affineExpr: MlirAffineExpr) -> std::ffi::c_long;

  pub fn mlirAffineExprIsAConstant(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineConstantExprGet(ctx: MlirContext, constant: std::ffi::c_long) -> MlirAffineExpr;

  pub fn mlirAffineConstantExprGetValue(affineExpr: MlirAffineExpr) -> std::ffi::c_long;

  pub fn mlirAffineExprIsAAdd(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineAddExprGet(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineExprIsAMul(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineMulExprGet(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineExprIsAMod(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineModExprGet(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineExprIsAFloorDiv(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineFloorDivExprGet(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineExprIsACeilDiv(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineCeilDivExprGet(lhs: MlirAffineExpr, rhs: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineExprIsABinary(affineExpr: MlirAffineExpr) -> u8;

  pub fn mlirAffineBinaryOpExprGetLHS(affineExpr: MlirAffineExpr) -> MlirAffineExpr;

  pub fn mlirAffineBinaryOpExprGetRHS(affineExpr: MlirAffineExpr) -> MlirAffineExpr;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirAffineExprGetContext<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprGetContext(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprEqual<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprEqual(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsNull<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsNull(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAffineExprPrint<T0_, T1_, T2_>(affineExpr_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::AffineExpr::mlirAffineExprPrint(Into::<MlirAffineExpr>::into(affineExpr_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAffineExprDump<T0_>(affineExpr_:  T0_)
  where
     T0_: Into<MlirAffineExpr>
  {
    unsafe {
      crate::AffineExpr::mlirAffineExprDump(Into::<MlirAffineExpr>::into(affineExpr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsSymbolicOrConstant<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsSymbolicOrConstant(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsPureAffine<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsPureAffine(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineExprGetLargestKnownDivisor<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprGetLargestKnownDivisor(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsMultipleOf<T0_, T1_>(affineExpr_:  T0_, factor_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsMultipleOf(Into::<MlirAffineExpr>::into(affineExpr_), Into::<std::ffi::c_long>::into(factor_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsFunctionOfDim<T0_, T1_>(affineExpr_:  T0_, position_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsFunctionOfDim(Into::<MlirAffineExpr>::into(affineExpr_), Into::<std::ffi::c_long>::into(position_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineExprCompose<T0_, T1_>(affineExpr_:  T0_, affineMap_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<StructMlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprCompose(Into::<MlirAffineExpr>::into(affineExpr_), Into::<StructMlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsADim<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsADim(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineDimExprGet<T0_, T1_>(ctx_:  T0_, position_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineDimExprGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(position_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineDimExprGetPosition<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineDimExprGetPosition(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsASymbol<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsASymbol(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineSymbolExprGet<T0_, T1_>(ctx_:  T0_, position_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineSymbolExprGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(position_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineSymbolExprGetPosition<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineSymbolExprGetPosition(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsAConstant<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsAConstant(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineConstantExprGet<T0_, T1_>(ctx_:  T0_, constant_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineConstantExprGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(constant_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineConstantExprGetValue<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineConstantExprGetValue(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsAAdd<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsAAdd(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineAddExprGet<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineAddExprGet(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsAMul<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsAMul(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineMulExprGet<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineMulExprGet(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsAMod<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsAMod(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineModExprGet<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineModExprGet(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsAFloorDiv<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsAFloorDiv(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineFloorDivExprGet<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineFloorDivExprGet(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsACeilDiv<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsACeilDiv(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineCeilDivExprGet<T0_, T1_>(lhs_:  T0_, rhs_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>,  T1_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineCeilDivExprGet(Into::<MlirAffineExpr>::into(lhs_), Into::<MlirAffineExpr>::into(rhs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineExprIsABinary<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineExprIsABinary(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineBinaryOpExprGetLHS<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineBinaryOpExprGetLHS(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineBinaryOpExprGetRHS<T0_>(affineExpr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineExpr::mlirAffineBinaryOpExprGetRHS(Into::<MlirAffineExpr>::into(affineExpr_))
      }
    )
  }
}

