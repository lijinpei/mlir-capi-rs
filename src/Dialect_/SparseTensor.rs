// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::AffineMap::*;
use crate::AffineExpr::*;
use crate::IR::*;
use crate::Support::*;
use crate::IR::*;
use crate::IR::*;
pub type MlirSparseTensorLevelType = std::ffi::c_ulong;

pub const MLIR_SPARSE_TENSOR_LEVEL_DENSE: std::ffi::c_uint = 65536;
pub const MLIR_SPARSE_TENSOR_LEVEL_BATCH: std::ffi::c_uint = 131072;
pub const MLIR_SPARSE_TENSOR_LEVEL_COMPRESSED: std::ffi::c_uint = 262144;
pub const MLIR_SPARSE_TENSOR_LEVEL_SINGLETON: std::ffi::c_uint = 524288;
pub const MLIR_SPARSE_TENSOR_LEVEL_LOOSE_COMPRESSED: std::ffi::c_uint = 1048576;
pub const MLIR_SPARSE_TENSOR_LEVEL_N_OUT_OF_M: std::ffi::c_uint = 2097152;
pub type EnumMlirSparseTensorLevelFormat = std::ffi::c_uint;

pub const MLIR_SPARSE_PROPERTY_NON_UNIQUE: std::ffi::c_uint = 1;
pub const MLIR_SPARSE_PROPERTY_NON_ORDERED: std::ffi::c_uint = 2;
pub const MLIR_SPARSE_PROPERTY_SOA: std::ffi::c_uint = 4;
pub type EnumMlirSparseTensorLevelPropertyNondefault = std::ffi::c_uint;
use crate::Pass::*;
use crate::IR::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirGetDialectHandle__sparse_tensor__() -> MlirDialectHandle;

  pub fn mlirAttributeIsASparseTensorEncodingAttr(attr: MlirAttribute) -> u8;

  pub fn mlirSparseTensorEncodingAttrGet(ctx: MlirContext, lvlRank: std::ffi::c_long, lvlTypes: *const std::ffi::c_ulong, dimToLvl: MlirAffineMap, lvlTodim: MlirAffineMap, posWidth: std::ffi::c_int, crdWidth: std::ffi::c_int, explicitVal: MlirAttribute, implicitVal: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseTensorEncodingGetLvlRank(attr: MlirAttribute) -> std::ffi::c_long;

  pub fn mlirSparseTensorEncodingAttrGetLvlType(attr: MlirAttribute, lvl: std::ffi::c_long) -> std::ffi::c_ulong;

  pub fn mlirSparseTensorEncodingAttrGetLvlFmt(attr: MlirAttribute, lvl: std::ffi::c_long) -> EnumMlirSparseTensorLevelFormat;

  pub fn mlirSparseTensorEncodingAttrGetDimToLvl(attr: MlirAttribute) -> MlirAffineMap;

  pub fn mlirSparseTensorEncodingAttrGetLvlToDim(attr: MlirAttribute) -> MlirAffineMap;

  pub fn mlirSparseTensorEncodingAttrGetPosWidth(attr: MlirAttribute) -> std::ffi::c_int;

  pub fn mlirSparseTensorEncodingAttrGetCrdWidth(attr: MlirAttribute) -> std::ffi::c_int;

  pub fn mlirSparseTensorEncodingAttrGetExplicitVal(attr: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseTensorEncodingAttrGetImplicitVal(attr: MlirAttribute) -> MlirAttribute;

  pub fn mlirSparseTensorEncodingAttrGetStructuredN(lvlType: std::ffi::c_ulong) -> std::ffi::c_uint;

  pub fn mlirSparseTensorEncodingAttrGetStructuredM(lvlType: std::ffi::c_ulong) -> std::ffi::c_uint;

  pub fn mlirSparseTensorEncodingAttrBuildLvlType(lvlFmt: EnumMlirSparseTensorLevelFormat, properties: *const EnumMlirSparseTensorLevelPropertyNondefault, propSize: std::ffi::c_uint, n: std::ffi::c_uint, m: std::ffi::c_uint) -> std::ffi::c_ulong;

  pub fn mlirRegisterSparseTensorPasses() -> ();

  pub fn mlirCreateSparseTensorLowerForeachToSCF() -> MlirPass;

  pub fn mlirRegisterSparseTensorLowerForeachToSCF() -> ();

  pub fn mlirCreateSparseTensorLowerSparseIterationToSCF() -> MlirPass;

  pub fn mlirRegisterSparseTensorLowerSparseIterationToSCF() -> ();

  pub fn mlirCreateSparseTensorLowerSparseOpsToForeach() -> MlirPass;

  pub fn mlirRegisterSparseTensorLowerSparseOpsToForeach() -> ();

  pub fn mlirCreateSparseTensorPreSparsificationRewrite() -> MlirPass;

  pub fn mlirRegisterSparseTensorPreSparsificationRewrite() -> ();

  pub fn mlirCreateSparseTensorSparseAssembler() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseAssembler() -> ();

  pub fn mlirCreateSparseTensorSparseBufferRewrite() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseBufferRewrite() -> ();

  pub fn mlirCreateSparseTensorSparseGPUCodegen() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseGPUCodegen() -> ();

  pub fn mlirCreateSparseTensorSparseReinterpretMap() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseReinterpretMap() -> ();

  pub fn mlirCreateSparseTensorSparseSpaceCollapse() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseSpaceCollapse() -> ();

  pub fn mlirCreateSparseTensorSparseTensorCodegen() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseTensorCodegen() -> ();

  pub fn mlirCreateSparseTensorSparseTensorConversionPass() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseTensorConversionPass() -> ();

  pub fn mlirCreateSparseTensorSparseVectorization() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparseVectorization() -> ();

  pub fn mlirCreateSparseTensorSparsificationAndBufferization() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparsificationAndBufferization() -> ();

  pub fn mlirCreateSparseTensorSparsificationPass() -> MlirPass;

  pub fn mlirRegisterSparseTensorSparsificationPass() -> ();

  pub fn mlirCreateSparseTensorStageSparseOperations() -> MlirPass;

  pub fn mlirRegisterSparseTensorStageSparseOperations() -> ();

  pub fn mlirCreateSparseTensorStorageSpecifierToLLVM() -> MlirPass;

  pub fn mlirRegisterSparseTensorStorageSpecifierToLLVM() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__sparse_tensor__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirGetDialectHandle__sparse_tensor__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsASparseTensorEncodingAttr<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirAttributeIsASparseTensorEncodingAttr(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseTensorEncodingAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, lvlRank_:  T1_, lvlTypes_:  T2_, dimToLvl_:  T3_, lvlTodim_:  T4_, posWidth_:  T5_, crdWidth_:  T6_, explicitVal_:  T7_, implicitVal_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const std::ffi::c_ulong>,  T3_: Into<MlirAffineMap>,  T4_: Into<MlirAffineMap>,  T5_: Into<std::ffi::c_int>,  T6_: Into<std::ffi::c_int>,  T7_: Into<MlirAttribute>,  T8_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(lvlRank_), Into::<*const std::ffi::c_ulong>::into(lvlTypes_), Into::<MlirAffineMap>::into(dimToLvl_), Into::<MlirAffineMap>::into(lvlTodim_), Into::<std::ffi::c_int>::into(posWidth_), Into::<std::ffi::c_int>::into(crdWidth_), Into::<MlirAttribute>::into(explicitVal_), Into::<MlirAttribute>::into(implicitVal_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirSparseTensorEncodingGetLvlRank<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingGetLvlRank(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetLvlType<T0_, T1_>(attr_:  T0_, lvl_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetLvlType(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(lvl_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<EnumMlirSparseTensorLevelFormat> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetLvlFmt<T0_, T1_>(attr_:  T0_, lvl_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetLvlFmt(Into::<MlirAttribute>::into(attr_), Into::<std::ffi::c_long>::into(lvl_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetDimToLvl<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetDimToLvl(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAffineMap> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetLvlToDim<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetLvlToDim(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetPosWidth<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetPosWidth(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_int> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetCrdWidth<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetCrdWidth(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetExplicitVal<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetExplicitVal(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetImplicitVal<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetImplicitVal(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetStructuredN<T0_>(lvlType_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetStructuredN(Into::<std::ffi::c_ulong>::into(lvlType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirSparseTensorEncodingAttrGetStructuredM<T0_>(lvlType_:  T0_)-> Tret_
  where
     T0_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrGetStructuredM(Into::<std::ffi::c_ulong>::into(lvlType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_ulong> {
  pub unsafe fn mlirSparseTensorEncodingAttrBuildLvlType<T0_, T1_, T2_, T3_, T4_>(lvlFmt_:  T0_, properties_:  T1_, propSize_:  T2_, n_:  T3_, m_:  T4_)-> Tret_
  where
     T0_: Into<EnumMlirSparseTensorLevelFormat>,  T1_: Into<*const EnumMlirSparseTensorLevelPropertyNondefault>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirSparseTensorEncodingAttrBuildLvlType(Into::<EnumMlirSparseTensorLevelFormat>::into(lvlFmt_), Into::<*const EnumMlirSparseTensorLevelPropertyNondefault>::into(properties_), Into::<std::ffi::c_uint>::into(propSize_), Into::<std::ffi::c_uint>::into(n_), Into::<std::ffi::c_uint>::into(m_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorLowerForeachToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorLowerForeachToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorLowerSparseIterationToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorLowerSparseIterationToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorLowerSparseOpsToForeach()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorLowerSparseOpsToForeach()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorPreSparsificationRewrite()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorPreSparsificationRewrite()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseAssembler()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseAssembler()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseBufferRewrite()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseBufferRewrite()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseGPUCodegen()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseGPUCodegen()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseReinterpretMap()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseReinterpretMap()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseSpaceCollapse()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseSpaceCollapse()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseTensorCodegen()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseTensorCodegen()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseTensorConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseTensorConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparseVectorization()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparseVectorization()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparsificationAndBufferization()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparsificationAndBufferization()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorSparsificationPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorSparsificationPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorStageSparseOperations()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorStageSparseOperations()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateSparseTensorStorageSpecifierToLLVM()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::SparseTensor::mlirCreateSparseTensorStorageSpecifierToLLVM()
      }
    )
  }
}

