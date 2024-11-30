// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use llvm_capi::Types::*;
use crate::IR::*;
use crate::Support::*;
use crate::Support::*;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirTranslateModuleToLLVMIR(module: MlirOperation, context: LLVMContextRef) -> LLVMModuleRef;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<LLVMModuleRef> {
  pub unsafe fn mlirTranslateModuleToLLVMIR<T0_, T1_>(module_:  T0_, context_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<LLVMContextRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Target_::LLVMIR::mlirTranslateModuleToLLVMIR(Into::<MlirOperation>::into(module_), Into::<LLVMContextRef>::into(context_))
      }
    )
  }
}

