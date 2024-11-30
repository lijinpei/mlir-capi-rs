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
use crate::IR::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirAffineMap {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirAffineMap = StructMlirAffineMap;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirAffineMapGetContext(affineMap: MlirAffineMap) -> MlirContext;

  pub fn mlirAffineMapIsNull(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapEqual(a1: MlirAffineMap, a2: MlirAffineMap) -> u8;

  pub fn mlirAffineMapPrint(affineMap: MlirAffineMap, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirAffineMapDump(affineMap: MlirAffineMap) -> ();

  pub fn mlirAffineMapEmptyGet(ctx: MlirContext) -> MlirAffineMap;

  pub fn mlirAffineMapZeroResultGet(ctx: MlirContext, dimCount: std::ffi::c_long, symbolCount: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapGet(ctx: MlirContext, dimCount: std::ffi::c_long, symbolCount: std::ffi::c_long, nAffineExprs: std::ffi::c_long, affineExprs: *mut MlirAffineExpr) -> MlirAffineMap;

  pub fn mlirAffineMapConstantGet(ctx: MlirContext, val: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapMultiDimIdentityGet(ctx: MlirContext, numDims: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapMinorIdentityGet(ctx: MlirContext, dims: std::ffi::c_long, results: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapPermutationGet(ctx: MlirContext, size: std::ffi::c_long, permutation: *mut std::ffi::c_uint) -> MlirAffineMap;

  pub fn mlirAffineMapIsIdentity(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapIsMinorIdentity(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapIsEmpty(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapIsSingleConstant(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapGetSingleConstantResult(affineMap: MlirAffineMap) -> std::ffi::c_long;

  pub fn mlirAffineMapGetNumDims(affineMap: MlirAffineMap) -> std::ffi::c_long;

  pub fn mlirAffineMapGetNumSymbols(affineMap: MlirAffineMap) -> std::ffi::c_long;

  pub fn mlirAffineMapGetNumResults(affineMap: MlirAffineMap) -> std::ffi::c_long;

  pub fn mlirAffineMapGetResult(affineMap: MlirAffineMap, pos: std::ffi::c_long) -> MlirAffineExpr;

  pub fn mlirAffineMapGetNumInputs(affineMap: MlirAffineMap) -> std::ffi::c_long;

  pub fn mlirAffineMapIsProjectedPermutation(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapIsPermutation(affineMap: MlirAffineMap) -> u8;

  pub fn mlirAffineMapGetSubMap(affineMap: MlirAffineMap, size: std::ffi::c_long, resultPos: *mut std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapGetMajorSubMap(affineMap: MlirAffineMap, numResults: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapGetMinorSubMap(affineMap: MlirAffineMap, numResults: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapReplace(affineMap: MlirAffineMap, expression: MlirAffineExpr, replacement: MlirAffineExpr, numResultDims: std::ffi::c_long, numResultSyms: std::ffi::c_long) -> MlirAffineMap;

  pub fn mlirAffineMapCompressUnusedSymbols(affineMaps: *mut MlirAffineMap, size: std::ffi::c_long, result: *mut std::ffi::c_void, populateResult: *mut extern fn (*mut std::ffi::c_void, std::ffi::c_long, MlirAffineMap) -> ()) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirAffineMapGetContext<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetContext(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsNull<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsNull(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapEqual<T0_, T1_>(a1_:  T0_, a2_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapEqual(Into::<MlirAffineMap>::into(a1_), Into::<MlirAffineMap>::into(a2_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAffineMapPrint<T0_, T1_, T2_>(affineMap_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::AffineMap::mlirAffineMapPrint(Into::<MlirAffineMap>::into(affineMap_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAffineMapDump<T0_>(affineMap_:  T0_)
  where
     T0_: Into<MlirAffineMap>
  {
    unsafe {
      crate::AffineMap::mlirAffineMapDump(Into::<MlirAffineMap>::into(affineMap_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapEmptyGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapEmptyGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapZeroResultGet<T0_, T1_, T2_>(ctx_:  T0_, dimCount_:  T1_, symbolCount_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapZeroResultGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(dimCount_), Into::<std::ffi::c_long>::into(symbolCount_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, dimCount_:  T1_, symbolCount_:  T2_, nAffineExprs_:  T3_, affineExprs_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*mut MlirAffineExpr>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(dimCount_), Into::<std::ffi::c_long>::into(symbolCount_), Into::<std::ffi::c_long>::into(nAffineExprs_), Into::<*mut MlirAffineExpr>::into(affineExprs_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapConstantGet<T0_, T1_>(ctx_:  T0_, val_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapConstantGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(val_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapMultiDimIdentityGet<T0_, T1_>(ctx_:  T0_, numDims_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapMultiDimIdentityGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(numDims_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapMinorIdentityGet<T0_, T1_, T2_>(ctx_:  T0_, dims_:  T1_, results_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapMinorIdentityGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(dims_), Into::<std::ffi::c_long>::into(results_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapPermutationGet<T0_, T1_, T2_>(ctx_:  T0_, size_:  T1_, permutation_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapPermutationGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(size_), Into::<*mut std::ffi::c_uint>::into(permutation_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsIdentity<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsIdentity(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsMinorIdentity<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsMinorIdentity(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsEmpty<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsEmpty(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsSingleConstant<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsSingleConstant(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineMapGetSingleConstantResult<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetSingleConstantResult(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineMapGetNumDims<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetNumDims(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineMapGetNumSymbols<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetNumSymbols(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineMapGetNumResults<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetNumResults(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineExpr> {
  pub unsafe fn mlirAffineMapGetResult<T0_, T1_>(affineMap_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetResult(Into::<MlirAffineMap>::into(affineMap_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirAffineMapGetNumInputs<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetNumInputs(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsProjectedPermutation<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsProjectedPermutation(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAffineMapIsPermutation<T0_>(affineMap_:  T0_)-> Tret_
  where
     T0_: Into<MlirAffineMap>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapIsPermutation(Into::<MlirAffineMap>::into(affineMap_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapGetSubMap<T0_, T1_, T2_>(affineMap_:  T0_, size_:  T1_, resultPos_:  T2_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetSubMap(Into::<MlirAffineMap>::into(affineMap_), Into::<std::ffi::c_long>::into(size_), Into::<*mut std::ffi::c_long>::into(resultPos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapGetMajorSubMap<T0_, T1_>(affineMap_:  T0_, numResults_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetMajorSubMap(Into::<MlirAffineMap>::into(affineMap_), Into::<std::ffi::c_long>::into(numResults_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapGetMinorSubMap<T0_, T1_>(affineMap_:  T0_, numResults_:  T1_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapGetMinorSubMap(Into::<MlirAffineMap>::into(affineMap_), Into::<std::ffi::c_long>::into(numResults_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirAffineMapReplace<T0_, T1_, T2_, T3_, T4_>(affineMap_:  T0_, expression_:  T1_, replacement_:  T2_, numResultDims_:  T3_, numResultSyms_:  T4_)-> Tret_
  where
     T0_: Into<MlirAffineMap>,  T1_: Into<MlirAffineExpr>,  T2_: Into<MlirAffineExpr>,  T3_: Into<std::ffi::c_long>,  T4_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::AffineMap::mlirAffineMapReplace(Into::<MlirAffineMap>::into(affineMap_), Into::<MlirAffineExpr>::into(expression_), Into::<MlirAffineExpr>::into(replacement_), Into::<std::ffi::c_long>::into(numResultDims_), Into::<std::ffi::c_long>::into(numResultSyms_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAffineMapCompressUnusedSymbols<T0_, T1_, T2_, T3_>(affineMaps_:  T0_, size_:  T1_, result_:  T2_, populateResult_:  T3_)
  where
     T0_: Into<*mut MlirAffineMap>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*mut std::ffi::c_void>,  T3_: Into<*mut extern fn (*mut std::ffi::c_void, std::ffi::c_long, MlirAffineMap) -> ()>
  {
    unsafe {
      crate::AffineMap::mlirAffineMapCompressUnusedSymbols(Into::<*mut MlirAffineMap>::into(affineMaps_), Into::<std::ffi::c_long>::into(size_), Into::<*mut std::ffi::c_void>::into(result_), Into::<*mut extern fn (*mut std::ffi::c_void, std::ffi::c_long, MlirAffineMap) -> ()>::into(populateResult_))
    }
  }
}

