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

  pub fn mlirGetDialectHandle__gpu__() -> MlirDialectHandle;

  pub fn mlirTypeIsAGPUAsyncTokenType(r#type: MlirType) -> u8;

  pub fn mlirGPUAsyncTokenTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirAttributeIsAGPUObjectAttr(attr: MlirAttribute) -> u8;

  pub fn mlirGPUObjectAttrGet(mlirCtx: MlirContext, target: MlirAttribute, format: std::ffi::c_uint, objectStrRef: MlirStringRef, mlirObjectProps: MlirAttribute) -> MlirAttribute;

  pub fn mlirGPUObjectAttrGetWithKernels(mlirCtx: MlirContext, target: MlirAttribute, format: std::ffi::c_uint, objectStrRef: MlirStringRef, mlirObjectProps: MlirAttribute, mlirKernelsAttr: MlirAttribute) -> MlirAttribute;

  pub fn mlirGPUObjectAttrGetTarget(mlirObjectAttr: MlirAttribute) -> MlirAttribute;

  pub fn mlirGPUObjectAttrGetFormat(mlirObjectAttr: MlirAttribute) -> std::ffi::c_uint;

  pub fn mlirGPUObjectAttrGetObject(mlirObjectAttr: MlirAttribute) -> MlirStringRef;

  pub fn mlirGPUObjectAttrHasProperties(mlirObjectAttr: MlirAttribute) -> u8;

  pub fn mlirGPUObjectAttrGetProperties(mlirObjectAttr: MlirAttribute) -> MlirAttribute;

  pub fn mlirGPUObjectAttrHasKernels(mlirObjectAttr: MlirAttribute) -> u8;

  pub fn mlirGPUObjectAttrGetKernels(mlirObjectAttr: MlirAttribute) -> MlirAttribute;

  pub fn mlirRegisterGPUPasses() -> ();

  pub fn mlirCreateGPUGpuAsyncRegionPass() -> MlirPass;

  pub fn mlirRegisterGPUGpuAsyncRegionPass() -> ();

  pub fn mlirCreateGPUGpuDecomposeMemrefsPass() -> MlirPass;

  pub fn mlirRegisterGPUGpuDecomposeMemrefsPass() -> ();

  pub fn mlirCreateGPUGpuEliminateBarriers() -> MlirPass;

  pub fn mlirRegisterGPUGpuEliminateBarriers() -> ();

  pub fn mlirCreateGPUGpuKernelOutlining() -> MlirPass;

  pub fn mlirRegisterGPUGpuKernelOutlining() -> ();

  pub fn mlirCreateGPUGpuLaunchSinkIndexComputations() -> MlirPass;

  pub fn mlirRegisterGPUGpuLaunchSinkIndexComputations() -> ();

  pub fn mlirCreateGPUGpuMapParallelLoopsPass() -> MlirPass;

  pub fn mlirRegisterGPUGpuMapParallelLoopsPass() -> ();

  pub fn mlirCreateGPUGpuModuleToBinaryPass() -> MlirPass;

  pub fn mlirRegisterGPUGpuModuleToBinaryPass() -> ();

  pub fn mlirCreateGPUGpuNVVMAttachTarget() -> MlirPass;

  pub fn mlirRegisterGPUGpuNVVMAttachTarget() -> ();

  pub fn mlirCreateGPUGpuROCDLAttachTarget() -> MlirPass;

  pub fn mlirRegisterGPUGpuROCDLAttachTarget() -> ();

  pub fn mlirCreateGPUGpuSPIRVAttachTarget() -> MlirPass;

  pub fn mlirRegisterGPUGpuSPIRVAttachTarget() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__gpu__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGetDialectHandle__gpu__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsAGPUAsyncTokenType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirTypeIsAGPUAsyncTokenType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirGPUAsyncTokenTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUAsyncTokenTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsAGPUObjectAttr<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirAttributeIsAGPUObjectAttr(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirGPUObjectAttrGet<T0_, T1_, T2_, T3_, T4_>(mlirCtx_:  T0_, target_:  T1_, format_:  T2_, objectStrRef_:  T3_, mlirObjectProps_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<MlirStringRef>,  T4_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGet(Into::<MlirContext>::into(mlirCtx_), Into::<MlirAttribute>::into(target_), Into::<std::ffi::c_uint>::into(format_), Into::<MlirStringRef>::into(objectStrRef_), Into::<MlirAttribute>::into(mlirObjectProps_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirGPUObjectAttrGetWithKernels<T0_, T1_, T2_, T3_, T4_, T5_>(mlirCtx_:  T0_, target_:  T1_, format_:  T2_, objectStrRef_:  T3_, mlirObjectProps_:  T4_, mlirKernelsAttr_:  T5_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<MlirStringRef>,  T4_: Into<MlirAttribute>,  T5_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetWithKernels(Into::<MlirContext>::into(mlirCtx_), Into::<MlirAttribute>::into(target_), Into::<std::ffi::c_uint>::into(format_), Into::<MlirStringRef>::into(objectStrRef_), Into::<MlirAttribute>::into(mlirObjectProps_), Into::<MlirAttribute>::into(mlirKernelsAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirGPUObjectAttrGetTarget<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetTarget(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirGPUObjectAttrGetFormat<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetFormat(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirGPUObjectAttrGetObject<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetObject(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirGPUObjectAttrHasProperties<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrHasProperties(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirGPUObjectAttrGetProperties<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetProperties(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirGPUObjectAttrHasKernels<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrHasKernels(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirGPUObjectAttrGetKernels<T0_>(mlirObjectAttr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirGPUObjectAttrGetKernels(Into::<MlirAttribute>::into(mlirObjectAttr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuAsyncRegionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuAsyncRegionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuDecomposeMemrefsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuDecomposeMemrefsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuEliminateBarriers()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuEliminateBarriers()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuKernelOutlining()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuKernelOutlining()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuLaunchSinkIndexComputations()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuLaunchSinkIndexComputations()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuMapParallelLoopsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuMapParallelLoopsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuModuleToBinaryPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuModuleToBinaryPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuNVVMAttachTarget()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuNVVMAttachTarget()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuROCDLAttachTarget()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuROCDLAttachTarget()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateGPUGpuSPIRVAttachTarget()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::GPU::mlirCreateGPUGpuSPIRVAttachTarget()
      }
    )
  }
}

