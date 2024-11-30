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
pub struct StructMlirRewriterBase {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirRewriterBase = StructMlirRewriterBase;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirFrozenRewritePatternSet {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirFrozenRewritePatternSet = StructMlirFrozenRewritePatternSet;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirGreedyRewriteDriverConfig {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirGreedyRewriteDriverConfig = StructMlirGreedyRewriteDriverConfig;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirRewritePatternSet {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirRewritePatternSet = StructMlirRewritePatternSet;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirPDLPatternModule {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirPDLPatternModule = StructMlirPDLPatternModule;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirRewriterBaseGetContext(rewriter: MlirRewriterBase) -> MlirContext;

  pub fn mlirRewriterBaseClearInsertionPoint(rewriter: MlirRewriterBase) -> ();

  pub fn mlirRewriterBaseSetInsertionPointBefore(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseSetInsertionPointAfter(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseSetInsertionPointAfterValue(rewriter: MlirRewriterBase, value: MlirValue) -> ();

  pub fn mlirRewriterBaseSetInsertionPointToStart(rewriter: MlirRewriterBase, block: MlirBlock) -> ();

  pub fn mlirRewriterBaseSetInsertionPointToEnd(rewriter: MlirRewriterBase, block: MlirBlock) -> ();

  pub fn mlirRewriterBaseGetInsertionBlock(rewriter: MlirRewriterBase) -> MlirBlock;

  pub fn mlirRewriterBaseGetBlock(rewriter: MlirRewriterBase) -> MlirBlock;

  pub fn mlirRewriterBaseCreateBlockBefore(rewriter: MlirRewriterBase, insertBefore: MlirBlock, nArgTypes: std::ffi::c_long, argTypes: *const MlirType, locations: *const MlirLocation) -> MlirBlock;

  pub fn mlirRewriterBaseInsert(rewriter: MlirRewriterBase, op: MlirOperation) -> MlirOperation;

  pub fn mlirRewriterBaseClone(rewriter: MlirRewriterBase, op: MlirOperation) -> MlirOperation;

  pub fn mlirRewriterBaseCloneWithoutRegions(rewriter: MlirRewriterBase, op: MlirOperation) -> MlirOperation;

  pub fn mlirRewriterBaseCloneRegionBefore(rewriter: MlirRewriterBase, region: MlirRegion, before: MlirBlock) -> ();

  pub fn mlirRewriterBaseInlineRegionBefore(rewriter: MlirRewriterBase, region: MlirRegion, before: MlirBlock) -> ();

  pub fn mlirRewriterBaseReplaceOpWithValues(rewriter: MlirRewriterBase, op: MlirOperation, nValues: std::ffi::c_long, values: *const MlirValue) -> ();

  pub fn mlirRewriterBaseReplaceOpWithOperation(rewriter: MlirRewriterBase, op: MlirOperation, newOp: MlirOperation) -> ();

  pub fn mlirRewriterBaseEraseOp(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseEraseBlock(rewriter: MlirRewriterBase, block: MlirBlock) -> ();

  pub fn mlirRewriterBaseInlineBlockBefore(rewriter: MlirRewriterBase, source: MlirBlock, op: MlirOperation, nArgValues: std::ffi::c_long, argValues: *const MlirValue) -> ();

  pub fn mlirRewriterBaseMergeBlocks(rewriter: MlirRewriterBase, source: MlirBlock, dest: MlirBlock, nArgValues: std::ffi::c_long, argValues: *const MlirValue) -> ();

  pub fn mlirRewriterBaseMoveOpBefore(rewriter: MlirRewriterBase, op: MlirOperation, existingOp: MlirOperation) -> ();

  pub fn mlirRewriterBaseMoveOpAfter(rewriter: MlirRewriterBase, op: MlirOperation, existingOp: MlirOperation) -> ();

  pub fn mlirRewriterBaseMoveBlockBefore(rewriter: MlirRewriterBase, block: MlirBlock, existingBlock: MlirBlock) -> ();

  pub fn mlirRewriterBaseStartOpModification(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseFinalizeOpModification(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseCancelOpModification(rewriter: MlirRewriterBase, op: MlirOperation) -> ();

  pub fn mlirRewriterBaseReplaceAllUsesWith(rewriter: MlirRewriterBase, from: MlirValue, to: MlirValue) -> ();

  pub fn mlirRewriterBaseReplaceAllValueRangeUsesWith(rewriter: MlirRewriterBase, nValues: std::ffi::c_long, from: *const MlirValue, to: *const MlirValue) -> ();

  pub fn mlirRewriterBaseReplaceAllOpUsesWithValueRange(rewriter: MlirRewriterBase, from: MlirOperation, nTo: std::ffi::c_long, to: *const MlirValue) -> ();

  pub fn mlirRewriterBaseReplaceAllOpUsesWithOperation(rewriter: MlirRewriterBase, from: MlirOperation, to: MlirOperation) -> ();

  pub fn mlirRewriterBaseReplaceOpUsesWithinBlock(rewriter: MlirRewriterBase, op: MlirOperation, nNewValues: std::ffi::c_long, newValues: *const MlirValue, block: MlirBlock) -> ();

  pub fn mlirRewriterBaseReplaceAllUsesExcept(rewriter: MlirRewriterBase, from: MlirValue, to: MlirValue, exceptedUser: MlirOperation) -> ();

  pub fn mlirIRRewriterCreate(context: MlirContext) -> MlirRewriterBase;

  pub fn mlirIRRewriterCreateFromOp(op: MlirOperation) -> MlirRewriterBase;

  pub fn mlirIRRewriterDestroy(rewriter: MlirRewriterBase) -> ();

  pub fn mlirFreezeRewritePattern(op: MlirRewritePatternSet) -> MlirFrozenRewritePatternSet;

  pub fn mlirFrozenRewritePatternSetDestroy(op: MlirFrozenRewritePatternSet) -> ();

  pub fn mlirApplyPatternsAndFoldGreedily(op: MlirModule, patterns: MlirFrozenRewritePatternSet, _: MlirGreedyRewriteDriverConfig) -> MlirLogicalResult;

  pub fn mlirPDLPatternModuleFromModule(op: MlirModule) -> MlirPDLPatternModule;

  pub fn mlirPDLPatternModuleDestroy(op: MlirPDLPatternModule) -> ();

  pub fn mlirRewritePatternSetFromPDLPatternModule(op: MlirPDLPatternModule) -> MlirRewritePatternSet;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirRewriterBaseGetContext<T0_>(rewriter_:  T0_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseGetContext(Into::<MlirRewriterBase>::into(rewriter_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseClearInsertionPoint<T0_>(rewriter_:  T0_)
  where
     T0_: Into<MlirRewriterBase>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseClearInsertionPoint(Into::<MlirRewriterBase>::into(rewriter_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseSetInsertionPointBefore<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseSetInsertionPointBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseSetInsertionPointAfter<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseSetInsertionPointAfter(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseSetInsertionPointAfterValue<T0_, T1_>(rewriter_:  T0_, value_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseSetInsertionPointAfterValue(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirValue>::into(value_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseSetInsertionPointToStart<T0_, T1_>(rewriter_:  T0_, block_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseSetInsertionPointToStart(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseSetInsertionPointToEnd<T0_, T1_>(rewriter_:  T0_, block_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseSetInsertionPointToEnd(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirRewriterBaseGetInsertionBlock<T0_>(rewriter_:  T0_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseGetInsertionBlock(Into::<MlirRewriterBase>::into(rewriter_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirRewriterBaseGetBlock<T0_>(rewriter_:  T0_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseGetBlock(Into::<MlirRewriterBase>::into(rewriter_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirRewriterBaseCreateBlockBefore<T0_, T1_, T2_, T3_, T4_>(rewriter_:  T0_, insertBefore_:  T1_, nArgTypes_:  T2_, argTypes_:  T3_, locations_:  T4_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirType>,  T4_: Into<*const MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseCreateBlockBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(insertBefore_), Into::<std::ffi::c_long>::into(nArgTypes_), Into::<*const MlirType>::into(argTypes_), Into::<*const MlirLocation>::into(locations_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirRewriterBaseInsert<T0_, T1_>(rewriter_:  T0_, op_:  T1_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseInsert(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirRewriterBaseClone<T0_, T1_>(rewriter_:  T0_, op_:  T1_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseClone(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirRewriterBaseCloneWithoutRegions<T0_, T1_>(rewriter_:  T0_, op_:  T1_)-> Tret_
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewriterBaseCloneWithoutRegions(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseCloneRegionBefore<T0_, T1_, T2_>(rewriter_:  T0_, region_:  T1_, before_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirRegion>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseCloneRegionBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirRegion>::into(region_), Into::<MlirBlock>::into(before_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseInlineRegionBefore<T0_, T1_, T2_>(rewriter_:  T0_, region_:  T1_, before_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirRegion>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseInlineRegionBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirRegion>::into(region_), Into::<MlirBlock>::into(before_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceOpWithValues<T0_, T1_, T2_, T3_>(rewriter_:  T0_, op_:  T1_, nValues_:  T2_, values_:  T3_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceOpWithValues(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(nValues_), Into::<*const MlirValue>::into(values_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceOpWithOperation<T0_, T1_, T2_>(rewriter_:  T0_, op_:  T1_, newOp_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceOpWithOperation(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(newOp_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseEraseOp<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseEraseOp(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseEraseBlock<T0_, T1_>(rewriter_:  T0_, block_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseEraseBlock(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseInlineBlockBefore<T0_, T1_, T2_, T3_, T4_>(rewriter_:  T0_, source_:  T1_, op_:  T2_, nArgValues_:  T3_, argValues_:  T4_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>,  T2_: Into<MlirOperation>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*const MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseInlineBlockBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(source_), Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(nArgValues_), Into::<*const MlirValue>::into(argValues_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseMergeBlocks<T0_, T1_, T2_, T3_, T4_>(rewriter_:  T0_, source_:  T1_, dest_:  T2_, nArgValues_:  T3_, argValues_:  T4_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>,  T2_: Into<MlirBlock>,  T3_: Into<std::ffi::c_long>,  T4_: Into<*const MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseMergeBlocks(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(source_), Into::<MlirBlock>::into(dest_), Into::<std::ffi::c_long>::into(nArgValues_), Into::<*const MlirValue>::into(argValues_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseMoveOpBefore<T0_, T1_, T2_>(rewriter_:  T0_, op_:  T1_, existingOp_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseMoveOpBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(existingOp_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseMoveOpAfter<T0_, T1_, T2_>(rewriter_:  T0_, op_:  T1_, existingOp_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseMoveOpAfter(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(existingOp_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseMoveBlockBefore<T0_, T1_, T2_>(rewriter_:  T0_, block_:  T1_, existingBlock_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirBlock>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseMoveBlockBefore(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirBlock>::into(block_), Into::<MlirBlock>::into(existingBlock_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseStartOpModification<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseStartOpModification(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseFinalizeOpModification<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseFinalizeOpModification(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseCancelOpModification<T0_, T1_>(rewriter_:  T0_, op_:  T1_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseCancelOpModification(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceAllUsesWith<T0_, T1_, T2_>(rewriter_:  T0_, from_:  T1_, to_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirValue>,  T2_: Into<MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceAllUsesWith(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirValue>::into(from_), Into::<MlirValue>::into(to_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceAllValueRangeUsesWith<T0_, T1_, T2_, T3_>(rewriter_:  T0_, nValues_:  T1_, from_:  T2_, to_:  T3_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirValue>,  T3_: Into<*const MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceAllValueRangeUsesWith(Into::<MlirRewriterBase>::into(rewriter_), Into::<std::ffi::c_long>::into(nValues_), Into::<*const MlirValue>::into(from_), Into::<*const MlirValue>::into(to_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceAllOpUsesWithValueRange<T0_, T1_, T2_, T3_>(rewriter_:  T0_, from_:  T1_, nTo_:  T2_, to_:  T3_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirValue>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceAllOpUsesWithValueRange(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(from_), Into::<std::ffi::c_long>::into(nTo_), Into::<*const MlirValue>::into(to_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceAllOpUsesWithOperation<T0_, T1_, T2_>(rewriter_:  T0_, from_:  T1_, to_:  T2_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceAllOpUsesWithOperation(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(from_), Into::<MlirOperation>::into(to_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceOpUsesWithinBlock<T0_, T1_, T2_, T3_, T4_>(rewriter_:  T0_, op_:  T1_, nNewValues_:  T2_, newValues_:  T3_, block_:  T4_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirOperation>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirValue>,  T4_: Into<MlirBlock>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceOpUsesWithinBlock(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(nNewValues_), Into::<*const MlirValue>::into(newValues_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRewriterBaseReplaceAllUsesExcept<T0_, T1_, T2_, T3_>(rewriter_:  T0_, from_:  T1_, to_:  T2_, exceptedUser_:  T3_)
  where
     T0_: Into<MlirRewriterBase>,  T1_: Into<MlirValue>,  T2_: Into<MlirValue>,  T3_: Into<MlirOperation>
  {
    unsafe {
      crate::Rewrite::mlirRewriterBaseReplaceAllUsesExcept(Into::<MlirRewriterBase>::into(rewriter_), Into::<MlirValue>::into(from_), Into::<MlirValue>::into(to_), Into::<MlirOperation>::into(exceptedUser_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRewriterBase> {
  pub unsafe fn mlirIRRewriterCreate<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirIRRewriterCreate(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRewriterBase> {
  pub unsafe fn mlirIRRewriterCreateFromOp<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirIRRewriterCreateFromOp(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirIRRewriterDestroy<T0_>(rewriter_:  T0_)
  where
     T0_: Into<MlirRewriterBase>
  {
    unsafe {
      crate::Rewrite::mlirIRRewriterDestroy(Into::<MlirRewriterBase>::into(rewriter_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirFrozenRewritePatternSet> {
  pub unsafe fn mlirFreezeRewritePattern<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirRewritePatternSet>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirFreezeRewritePattern(Into::<MlirRewritePatternSet>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirFrozenRewritePatternSetDestroy<T0_>(op_:  T0_)
  where
     T0_: Into<MlirFrozenRewritePatternSet>
  {
    unsafe {
      crate::Rewrite::mlirFrozenRewritePatternSetDestroy(Into::<MlirFrozenRewritePatternSet>::into(op_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirApplyPatternsAndFoldGreedily<T0_, T1_, T2_>(op_:  T0_, patterns_:  T1_, arg2_:  T2_)-> Tret_
  where
     T0_: Into<MlirModule>,  T1_: Into<MlirFrozenRewritePatternSet>,  T2_: Into<MlirGreedyRewriteDriverConfig>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirApplyPatternsAndFoldGreedily(Into::<MlirModule>::into(op_), Into::<MlirFrozenRewritePatternSet>::into(patterns_), Into::<MlirGreedyRewriteDriverConfig>::into(arg2_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPDLPatternModule> {
  pub unsafe fn mlirPDLPatternModuleFromModule<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirPDLPatternModuleFromModule(Into::<MlirModule>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirPDLPatternModuleDestroy<T0_>(op_:  T0_)
  where
     T0_: Into<MlirPDLPatternModule>
  {
    unsafe {
      crate::Rewrite::mlirPDLPatternModuleDestroy(Into::<MlirPDLPatternModule>::into(op_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRewritePatternSet> {
  pub unsafe fn mlirRewritePatternSetFromPDLPatternModule<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirPDLPatternModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Rewrite::mlirRewritePatternSetFromPDLPatternModule(Into::<MlirPDLPatternModule>::into(op_))
      }
    )
  }
}

