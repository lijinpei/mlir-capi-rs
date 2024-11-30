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

  pub fn mlirRegisterConversionPasses() -> ();

  pub fn mlirCreateConversionArithToAMDGPUConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionArithToAMDGPUConversionPass() -> ();

  pub fn mlirCreateConversionArithToArmSMEConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionArithToArmSMEConversionPass() -> ();

  pub fn mlirCreateConversionArithToLLVMConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionArithToLLVMConversionPass() -> ();

  pub fn mlirCreateConversionConvertAMDGPUToROCDL() -> MlirPass;

  pub fn mlirRegisterConversionConvertAMDGPUToROCDL() -> ();

  pub fn mlirCreateConversionConvertAffineForToGPU() -> MlirPass;

  pub fn mlirRegisterConversionConvertAffineForToGPU() -> ();

  pub fn mlirCreateConversionConvertAffineToStandard() -> MlirPass;

  pub fn mlirRegisterConversionConvertAffineToStandard() -> ();

  pub fn mlirCreateConversionConvertArithToEmitC() -> MlirPass;

  pub fn mlirRegisterConversionConvertArithToEmitC() -> ();

  pub fn mlirCreateConversionConvertArithToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertArithToSPIRV() -> ();

  pub fn mlirCreateConversionConvertArmNeon2dToIntr() -> MlirPass;

  pub fn mlirRegisterConversionConvertArmNeon2dToIntr() -> ();

  pub fn mlirCreateConversionConvertArmSMEToLLVM() -> MlirPass;

  pub fn mlirRegisterConversionConvertArmSMEToLLVM() -> ();

  pub fn mlirCreateConversionConvertArmSMEToSCF() -> MlirPass;

  pub fn mlirRegisterConversionConvertArmSMEToSCF() -> ();

  pub fn mlirCreateConversionConvertAsyncToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertAsyncToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertBufferizationToMemRef() -> MlirPass;

  pub fn mlirRegisterConversionConvertBufferizationToMemRef() -> ();

  pub fn mlirCreateConversionConvertComplexToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertComplexToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertComplexToLibm() -> MlirPass;

  pub fn mlirRegisterConversionConvertComplexToLibm() -> ();

  pub fn mlirCreateConversionConvertComplexToSPIRVPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertComplexToSPIRVPass() -> ();

  pub fn mlirCreateConversionConvertComplexToStandard() -> MlirPass;

  pub fn mlirRegisterConversionConvertComplexToStandard() -> ();

  pub fn mlirCreateConversionConvertControlFlowToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertControlFlowToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertControlFlowToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertControlFlowToSPIRV() -> ();

  pub fn mlirCreateConversionConvertFuncToEmitC() -> MlirPass;

  pub fn mlirRegisterConversionConvertFuncToEmitC() -> ();

  pub fn mlirCreateConversionConvertFuncToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertFuncToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertFuncToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertFuncToSPIRV() -> ();

  pub fn mlirCreateConversionConvertGPUToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertGPUToSPIRV() -> ();

  pub fn mlirCreateConversionConvertGpuLaunchFuncToVulkanLaunchFunc() -> MlirPass;

  pub fn mlirRegisterConversionConvertGpuLaunchFuncToVulkanLaunchFunc() -> ();

  pub fn mlirCreateConversionConvertGpuOpsToLLVMSPVOps() -> MlirPass;

  pub fn mlirRegisterConversionConvertGpuOpsToLLVMSPVOps() -> ();

  pub fn mlirCreateConversionConvertGpuOpsToNVVMOps() -> MlirPass;

  pub fn mlirRegisterConversionConvertGpuOpsToNVVMOps() -> ();

  pub fn mlirCreateConversionConvertGpuOpsToROCDLOps() -> MlirPass;

  pub fn mlirRegisterConversionConvertGpuOpsToROCDLOps() -> ();

  pub fn mlirCreateConversionConvertIndexToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertIndexToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertIndexToSPIRVPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertIndexToSPIRVPass() -> ();

  pub fn mlirCreateConversionConvertLinalgToStandard() -> MlirPass;

  pub fn mlirRegisterConversionConvertLinalgToStandard() -> ();

  pub fn mlirCreateConversionConvertMathToFuncs() -> MlirPass;

  pub fn mlirRegisterConversionConvertMathToFuncs() -> ();

  pub fn mlirCreateConversionConvertMathToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertMathToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertMathToLibm() -> MlirPass;

  pub fn mlirRegisterConversionConvertMathToLibm() -> ();

  pub fn mlirCreateConversionConvertMathToROCDL() -> MlirPass;

  pub fn mlirRegisterConversionConvertMathToROCDL() -> ();

  pub fn mlirCreateConversionConvertMathToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertMathToSPIRV() -> ();

  pub fn mlirCreateConversionConvertMemRefToEmitC() -> MlirPass;

  pub fn mlirRegisterConversionConvertMemRefToEmitC() -> ();

  pub fn mlirCreateConversionConvertMemRefToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertMemRefToSPIRV() -> ();

  pub fn mlirCreateConversionConvertMeshToMPIPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertMeshToMPIPass() -> ();

  pub fn mlirCreateConversionConvertNVGPUToNVVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertNVGPUToNVVMPass() -> ();

  pub fn mlirCreateConversionConvertNVVMToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertNVVMToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertOpenACCToSCF() -> MlirPass;

  pub fn mlirRegisterConversionConvertOpenACCToSCF() -> ();

  pub fn mlirCreateConversionConvertOpenMPToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertOpenMPToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertPDLToPDLInterp() -> MlirPass;

  pub fn mlirRegisterConversionConvertPDLToPDLInterp() -> ();

  pub fn mlirCreateConversionConvertParallelLoopToGpu() -> MlirPass;

  pub fn mlirRegisterConversionConvertParallelLoopToGpu() -> ();

  pub fn mlirCreateConversionConvertSCFToOpenMPPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertSCFToOpenMPPass() -> ();

  pub fn mlirCreateConversionConvertSPIRVToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertSPIRVToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertShapeConstraints() -> MlirPass;

  pub fn mlirRegisterConversionConvertShapeConstraints() -> ();

  pub fn mlirCreateConversionConvertShapeToStandard() -> MlirPass;

  pub fn mlirRegisterConversionConvertShapeToStandard() -> ();

  pub fn mlirCreateConversionConvertTensorToLinalg() -> MlirPass;

  pub fn mlirRegisterConversionConvertTensorToLinalg() -> ();

  pub fn mlirCreateConversionConvertTensorToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertTensorToSPIRV() -> ();

  pub fn mlirCreateConversionConvertToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertToSPIRVPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertToSPIRVPass() -> ();

  pub fn mlirCreateConversionConvertVectorToArmSME() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToArmSME() -> ();

  pub fn mlirCreateConversionConvertVectorToGPU() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToGPU() -> ();

  pub fn mlirCreateConversionConvertVectorToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToLLVMPass() -> ();

  pub fn mlirCreateConversionConvertVectorToSCF() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToSCF() -> ();

  pub fn mlirCreateConversionConvertVectorToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToSPIRV() -> ();

  pub fn mlirCreateConversionConvertVectorToXeGPU() -> MlirPass;

  pub fn mlirRegisterConversionConvertVectorToXeGPU() -> ();

  pub fn mlirCreateConversionConvertVulkanLaunchFuncToVulkanCallsPass() -> MlirPass;

  pub fn mlirRegisterConversionConvertVulkanLaunchFuncToVulkanCallsPass() -> ();

  pub fn mlirCreateConversionFinalizeMemRefToLLVMConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionFinalizeMemRefToLLVMConversionPass() -> ();

  pub fn mlirCreateConversionGpuToLLVMConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionGpuToLLVMConversionPass() -> ();

  pub fn mlirCreateConversionLiftControlFlowToSCFPass() -> MlirPass;

  pub fn mlirRegisterConversionLiftControlFlowToSCFPass() -> ();

  pub fn mlirCreateConversionLowerHostCodeToLLVMPass() -> MlirPass;

  pub fn mlirRegisterConversionLowerHostCodeToLLVMPass() -> ();

  pub fn mlirCreateConversionMapMemRefStorageClass() -> MlirPass;

  pub fn mlirRegisterConversionMapMemRefStorageClass() -> ();

  pub fn mlirCreateConversionReconcileUnrealizedCasts() -> MlirPass;

  pub fn mlirRegisterConversionReconcileUnrealizedCasts() -> ();

  pub fn mlirCreateConversionSCFToControlFlow() -> MlirPass;

  pub fn mlirRegisterConversionSCFToControlFlow() -> ();

  pub fn mlirCreateConversionSCFToEmitC() -> MlirPass;

  pub fn mlirRegisterConversionSCFToEmitC() -> ();

  pub fn mlirCreateConversionSCFToSPIRV() -> MlirPass;

  pub fn mlirRegisterConversionSCFToSPIRV() -> ();

  pub fn mlirCreateConversionSetLLVMModuleDataLayoutPass() -> MlirPass;

  pub fn mlirRegisterConversionSetLLVMModuleDataLayoutPass() -> ();

  pub fn mlirCreateConversionTosaToArith() -> MlirPass;

  pub fn mlirRegisterConversionTosaToArith() -> ();

  pub fn mlirCreateConversionTosaToLinalg() -> MlirPass;

  pub fn mlirRegisterConversionTosaToLinalg() -> ();

  pub fn mlirCreateConversionTosaToLinalgNamed() -> MlirPass;

  pub fn mlirRegisterConversionTosaToLinalgNamed() -> ();

  pub fn mlirCreateConversionTosaToMLProgram() -> MlirPass;

  pub fn mlirRegisterConversionTosaToMLProgram() -> ();

  pub fn mlirCreateConversionTosaToSCF() -> MlirPass;

  pub fn mlirRegisterConversionTosaToSCF() -> ();

  pub fn mlirCreateConversionTosaToTensor() -> MlirPass;

  pub fn mlirRegisterConversionTosaToTensor() -> ();

  pub fn mlirCreateConversionUBToLLVMConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionUBToLLVMConversionPass() -> ();

  pub fn mlirCreateConversionUBToSPIRVConversionPass() -> MlirPass;

  pub fn mlirRegisterConversionUBToSPIRVConversionPass() -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionArithToAMDGPUConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionArithToAMDGPUConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionArithToArmSMEConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionArithToArmSMEConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionArithToLLVMConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionArithToLLVMConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertAMDGPUToROCDL()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertAMDGPUToROCDL()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertAffineForToGPU()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertAffineForToGPU()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertAffineToStandard()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertAffineToStandard()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertArithToEmitC()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertArithToEmitC()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertArithToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertArithToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertArmNeon2dToIntr()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertArmNeon2dToIntr()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertArmSMEToLLVM()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertArmSMEToLLVM()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertArmSMEToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertArmSMEToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertAsyncToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertAsyncToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertBufferizationToMemRef()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertBufferizationToMemRef()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertComplexToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertComplexToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertComplexToLibm()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertComplexToLibm()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertComplexToSPIRVPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertComplexToSPIRVPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertComplexToStandard()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertComplexToStandard()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertControlFlowToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertControlFlowToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertControlFlowToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertControlFlowToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertFuncToEmitC()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertFuncToEmitC()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertFuncToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertFuncToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertFuncToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertFuncToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertGPUToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertGPUToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertGpuLaunchFuncToVulkanLaunchFunc()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertGpuLaunchFuncToVulkanLaunchFunc()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertGpuOpsToLLVMSPVOps()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertGpuOpsToLLVMSPVOps()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertGpuOpsToNVVMOps()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertGpuOpsToNVVMOps()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertGpuOpsToROCDLOps()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertGpuOpsToROCDLOps()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertIndexToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertIndexToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertIndexToSPIRVPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertIndexToSPIRVPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertLinalgToStandard()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertLinalgToStandard()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMathToFuncs()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMathToFuncs()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMathToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMathToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMathToLibm()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMathToLibm()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMathToROCDL()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMathToROCDL()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMathToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMathToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMemRefToEmitC()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMemRefToEmitC()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMemRefToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMemRefToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertMeshToMPIPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertMeshToMPIPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertNVGPUToNVVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertNVGPUToNVVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertNVVMToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertNVVMToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertOpenACCToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertOpenACCToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertOpenMPToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertOpenMPToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertPDLToPDLInterp()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertPDLToPDLInterp()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertParallelLoopToGpu()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertParallelLoopToGpu()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertSCFToOpenMPPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertSCFToOpenMPPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertSPIRVToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertSPIRVToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertShapeConstraints()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertShapeConstraints()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertShapeToStandard()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertShapeToStandard()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertTensorToLinalg()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertTensorToLinalg()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertTensorToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertTensorToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertToSPIRVPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertToSPIRVPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToArmSME()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToArmSME()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToGPU()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToGPU()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVectorToXeGPU()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVectorToXeGPU()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionConvertVulkanLaunchFuncToVulkanCallsPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionConvertVulkanLaunchFuncToVulkanCallsPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionFinalizeMemRefToLLVMConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionFinalizeMemRefToLLVMConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionGpuToLLVMConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionGpuToLLVMConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionLiftControlFlowToSCFPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionLiftControlFlowToSCFPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionLowerHostCodeToLLVMPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionLowerHostCodeToLLVMPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionMapMemRefStorageClass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionMapMemRefStorageClass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionReconcileUnrealizedCasts()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionReconcileUnrealizedCasts()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionSCFToControlFlow()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionSCFToControlFlow()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionSCFToEmitC()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionSCFToEmitC()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionSCFToSPIRV()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionSCFToSPIRV()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionSetLLVMModuleDataLayoutPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionSetLLVMModuleDataLayoutPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToArith()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToArith()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToLinalg()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToLinalg()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToLinalgNamed()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToLinalgNamed()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToMLProgram()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToMLProgram()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToSCF()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToSCF()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionTosaToTensor()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionTosaToTensor()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionUBToLLVMConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionUBToLLVMConversionPass()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirPass> {
  pub unsafe fn mlirCreateConversionUBToSPIRVConversionPass()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Conversion::mlirCreateConversionUBToSPIRVConversionPass()
      }
    )
  }
}

