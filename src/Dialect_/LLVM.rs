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

pub const MlirLLVMCConvC: std::ffi::c_uint = 0;
pub const MlirLLVMCConvFast: std::ffi::c_uint = 8;
pub const MlirLLVMCConvCold: std::ffi::c_uint = 9;
pub const MlirLLVMCConvGHC: std::ffi::c_uint = 10;
pub const MlirLLVMCConvHiPE: std::ffi::c_uint = 11;
pub const MlirLLVMCConvAnyReg: std::ffi::c_uint = 13;
pub const MlirLLVMCConvPreserveMost: std::ffi::c_uint = 14;
pub const MlirLLVMCConvPreserveAll: std::ffi::c_uint = 15;
pub const MlirLLVMCConvSwift: std::ffi::c_uint = 16;
pub const MlirLLVMCConvCXX_FAST_TLS: std::ffi::c_uint = 17;
pub const MlirLLVMCConvTail: std::ffi::c_uint = 18;
pub const MlirLLVMCConvCFGuard_Check: std::ffi::c_uint = 19;
pub const MlirLLVMCConvSwiftTail: std::ffi::c_uint = 20;
pub const MlirLLVMCConvX86_StdCall: std::ffi::c_uint = 64;
pub const MlirLLVMCConvX86_FastCall: std::ffi::c_uint = 65;
pub const MlirLLVMCConvARM_APCS: std::ffi::c_uint = 66;
pub const MlirLLVMCConvARM_AAPCS: std::ffi::c_uint = 67;
pub const MlirLLVMCConvARM_AAPCS_VFP: std::ffi::c_uint = 68;
pub const MlirLLVMCConvMSP430_INTR: std::ffi::c_uint = 69;
pub const MlirLLVMCConvX86_ThisCall: std::ffi::c_uint = 70;
pub const MlirLLVMCConvPTX_Kernel: std::ffi::c_uint = 71;
pub const MlirLLVMCConvPTX_Device: std::ffi::c_uint = 72;
pub const MlirLLVMCConvSPIR_FUNC: std::ffi::c_uint = 75;
pub const MlirLLVMCConvSPIR_KERNEL: std::ffi::c_uint = 76;
pub const MlirLLVMCConvIntel_OCL_BI: std::ffi::c_uint = 77;
pub const MlirLLVMCConvX86_64_SysV: std::ffi::c_uint = 78;
pub const MlirLLVMCConvWin64: std::ffi::c_uint = 79;
pub const MlirLLVMCConvX86_VectorCall: std::ffi::c_uint = 80;
pub const MlirLLVMCConvDUMMY_HHVM: std::ffi::c_uint = 81;
pub const MlirLLVMCConvDUMMY_HHVM_C: std::ffi::c_uint = 82;
pub const MlirLLVMCConvX86_INTR: std::ffi::c_uint = 83;
pub const MlirLLVMCConvAVR_INTR: std::ffi::c_uint = 84;
pub const MlirLLVMCConvAVR_BUILTIN: std::ffi::c_uint = 86;
pub const MlirLLVMCConvAMDGPU_VS: std::ffi::c_uint = 87;
pub const MlirLLVMCConvAMDGPU_GS: std::ffi::c_uint = 88;
pub const MlirLLVMCConvAMDGPU_CS: std::ffi::c_uint = 90;
pub const MlirLLVMCConvAMDGPU_KERNEL: std::ffi::c_uint = 91;
pub const MlirLLVMCConvX86_RegCall: std::ffi::c_uint = 92;
pub const MlirLLVMCConvAMDGPU_HS: std::ffi::c_uint = 93;
pub const MlirLLVMCConvMSP430_BUILTIN: std::ffi::c_uint = 94;
pub const MlirLLVMCConvAMDGPU_LS: std::ffi::c_uint = 95;
pub const MlirLLVMCConvAMDGPU_ES: std::ffi::c_uint = 96;
pub const MlirLLVMCConvAArch64_VectorCall: std::ffi::c_uint = 97;
pub const MlirLLVMCConvAArch64_SVE_VectorCall: std::ffi::c_uint = 98;
pub const MlirLLVMCConvWASM_EmscriptenInvoke: std::ffi::c_uint = 99;
pub const MlirLLVMCConvAMDGPU_Gfx: std::ffi::c_uint = 100;
pub const MlirLLVMCConvM68k_INTR: std::ffi::c_uint = 101;
pub type EnumMlirLLVMCConv = std::ffi::c_uint;
pub type MlirLLVMCConv = EnumMlirLLVMCConv;

pub const MlirLLVMComdatAny: std::ffi::c_uint = 0;
pub const MlirLLVMComdatExactMatch: std::ffi::c_uint = 1;
pub const MlirLLVMComdatLargest: std::ffi::c_uint = 2;
pub const MlirLLVMComdatNoDeduplicate: std::ffi::c_uint = 3;
pub const MlirLLVMComdatSameSize: std::ffi::c_uint = 4;
pub type EnumMlirLLVMComdat = std::ffi::c_uint;
pub type MlirLLVMComdat = EnumMlirLLVMComdat;

pub const MlirLLVMLinkageExternal: std::ffi::c_uint = 0;
pub const MlirLLVMLinkageAvailableExternally: std::ffi::c_uint = 1;
pub const MlirLLVMLinkageLinkonce: std::ffi::c_uint = 2;
pub const MlirLLVMLinkageLinkonceODR: std::ffi::c_uint = 3;
pub const MlirLLVMLinkageWeak: std::ffi::c_uint = 4;
pub const MlirLLVMLinkageWeakODR: std::ffi::c_uint = 5;
pub const MlirLLVMLinkageAppending: std::ffi::c_uint = 6;
pub const MlirLLVMLinkageInternal: std::ffi::c_uint = 7;
pub const MlirLLVMLinkagePrivate: std::ffi::c_uint = 8;
pub const MlirLLVMLinkageExternWeak: std::ffi::c_uint = 9;
pub const MlirLLVMLinkageCommon: std::ffi::c_uint = 10;
pub type EnumMlirLLVMLinkage = std::ffi::c_uint;
pub type MlirLLVMLinkage = EnumMlirLLVMLinkage;

pub const MlirLLVMTypeEncodingAddress: std::ffi::c_uint = 1;
pub const MlirLLVMTypeEncodingBoolean: std::ffi::c_uint = 2;
pub const MlirLLVMTypeEncodingComplexFloat: std::ffi::c_uint = 49;
pub const MlirLLVMTypeEncodingFloatT: std::ffi::c_uint = 4;
pub const MlirLLVMTypeEncodingSigned: std::ffi::c_uint = 5;
pub const MlirLLVMTypeEncodingSignedChar: std::ffi::c_uint = 6;
pub const MlirLLVMTypeEncodingUnsigned: std::ffi::c_uint = 7;
pub const MlirLLVMTypeEncodingUnsignedChar: std::ffi::c_uint = 8;
pub const MlirLLVMTypeEncodingImaginaryFloat: std::ffi::c_uint = 9;
pub const MlirLLVMTypeEncodingPackedDecimal: std::ffi::c_uint = 10;
pub const MlirLLVMTypeEncodingNumericString: std::ffi::c_uint = 11;
pub const MlirLLVMTypeEncodingEdited: std::ffi::c_uint = 12;
pub const MlirLLVMTypeEncodingSignedFixed: std::ffi::c_uint = 13;
pub const MlirLLVMTypeEncodingUnsignedFixed: std::ffi::c_uint = 14;
pub const MlirLLVMTypeEncodingDecimalFloat: std::ffi::c_uint = 15;
pub const MlirLLVMTypeEncodingUTF: std::ffi::c_uint = 16;
pub const MlirLLVMTypeEncodingUCS: std::ffi::c_uint = 17;
pub const MlirLLVMTypeEncodingASCII: std::ffi::c_uint = 18;
pub const MlirLLVMTypeEncodingLoUser: std::ffi::c_uint = 128;
pub const MlirLLVMTypeEncodingHiUser: std::ffi::c_uint = 255;
pub type EnumMlirLLVMTypeEncoding = std::ffi::c_uint;
pub type MlirLLVMTypeEncoding = EnumMlirLLVMTypeEncoding;

pub const MlirLLVMDIEmissionKindNone: std::ffi::c_uint = 0;
pub const MlirLLVMDIEmissionKindFull: std::ffi::c_uint = 1;
pub const MlirLLVMDIEmissionKindLineTablesOnly: std::ffi::c_uint = 2;
pub const MlirLLVMDIEmissionKindDebugDirectivesOnly: std::ffi::c_uint = 3;
pub type EnumMlirLLVMDIEmissionKind = std::ffi::c_uint;
pub type MlirLLVMDIEmissionKind = EnumMlirLLVMDIEmissionKind;

pub const MlirLLVMDINameTableKindDefault: std::ffi::c_uint = 0;
pub const MlirLLVMDINameTableKindGNU: std::ffi::c_uint = 1;
pub const MlirLLVMDINameTableKindNone: std::ffi::c_uint = 2;
pub const MlirLLVMDINameTableKindApple: std::ffi::c_uint = 3;
pub type EnumMlirLLVMDINameTableKind = std::ffi::c_uint;
pub type MlirLLVMDINameTableKind = EnumMlirLLVMDINameTableKind;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirGetDialectHandle__llvm__() -> MlirDialectHandle;

  pub fn mlirLLVMPointerTypeGet(ctx: MlirContext, addressSpace: std::ffi::c_uint) -> MlirType;

  pub fn mlirTypeIsALLVMPointerType(r#type: MlirType) -> u8;

  pub fn mlirLLVMPointerTypeGetAddressSpace(pointerType: MlirType) -> std::ffi::c_uint;

  pub fn mlirLLVMVoidTypeGet(ctx: MlirContext) -> MlirType;

  pub fn mlirLLVMArrayTypeGet(elementType: MlirType, numElements: std::ffi::c_uint) -> MlirType;

  pub fn mlirLLVMFunctionTypeGet(resultType: MlirType, nArgumentTypes: std::ffi::c_long, argumentTypes: *const MlirType, isVarArg: u8) -> MlirType;

  pub fn mlirTypeIsALLVMStructType(r#type: MlirType) -> u8;

  pub fn mlirLLVMStructTypeIsLiteral(r#type: MlirType) -> u8;

  pub fn mlirLLVMStructTypeGetNumElementTypes(r#type: MlirType) -> std::ffi::c_long;

  pub fn mlirLLVMStructTypeGetElementType(r#type: MlirType, position: std::ffi::c_long) -> MlirType;

  pub fn mlirLLVMStructTypeIsPacked(r#type: MlirType) -> u8;

  pub fn mlirLLVMStructTypeGetIdentifier(r#type: MlirType) -> MlirStringRef;

  pub fn mlirLLVMStructTypeIsOpaque(r#type: MlirType) -> u8;

  pub fn mlirLLVMStructTypeLiteralGet(ctx: MlirContext, nFieldTypes: std::ffi::c_long, fieldTypes: *const MlirType, isPacked: u8) -> MlirType;

  pub fn mlirLLVMStructTypeLiteralGetChecked(loc: MlirLocation, nFieldTypes: std::ffi::c_long, fieldTypes: *const MlirType, isPacked: u8) -> MlirType;

  pub fn mlirLLVMStructTypeIdentifiedGet(ctx: MlirContext, name: MlirStringRef) -> MlirType;

  pub fn mlirLLVMStructTypeIdentifiedNewGet(ctx: MlirContext, name: MlirStringRef, nFieldTypes: std::ffi::c_long, fieldTypes: *const MlirType, isPacked: u8) -> MlirType;

  pub fn mlirLLVMStructTypeOpaqueGet(ctx: MlirContext, name: MlirStringRef) -> MlirType;

  pub fn mlirLLVMStructTypeSetBody(structType: MlirType, nFieldTypes: std::ffi::c_long, fieldTypes: *const MlirType, isPacked: u8) -> MlirLogicalResult;

  pub fn mlirLLVMCConvAttrGet(ctx: MlirContext, cconv: EnumMlirLLVMCConv) -> MlirAttribute;

  pub fn mlirLLVMComdatAttrGet(ctx: MlirContext, comdat: EnumMlirLLVMComdat) -> MlirAttribute;

  pub fn mlirLLVMLinkageAttrGet(ctx: MlirContext, linkage: EnumMlirLLVMLinkage) -> MlirAttribute;

  pub fn mlirLLVMDINullTypeAttrGet(ctx: MlirContext) -> MlirAttribute;

  pub fn mlirLLVMDIExpressionElemAttrGet(ctx: MlirContext, opcode: std::ffi::c_uint, nArguments: std::ffi::c_long, arguments: *const std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirLLVMDIExpressionAttrGet(ctx: MlirContext, nOperations: std::ffi::c_long, operations: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIBasicTypeAttrGet(ctx: MlirContext, tag: std::ffi::c_uint, name: MlirAttribute, sizeInBits: std::ffi::c_ulong, encoding: EnumMlirLLVMTypeEncoding) -> MlirAttribute;

  pub fn mlirLLVMDICompositeTypeAttrGetRecSelf(recId: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDICompositeTypeAttrGet(ctx: MlirContext, recId: MlirAttribute, isRecSelf: u8, tag: std::ffi::c_uint, name: MlirAttribute, file: MlirAttribute, line: std::ffi::c_uint, scope: MlirAttribute, baseType: MlirAttribute, flags: std::ffi::c_long, sizeInBits: std::ffi::c_ulong, alignInBits: std::ffi::c_ulong, nElements: std::ffi::c_long, elements: *const MlirAttribute, dataLocation: MlirAttribute, rank: MlirAttribute, allocated: MlirAttribute, associated: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIDerivedTypeAttrGet(ctx: MlirContext, tag: std::ffi::c_uint, name: MlirAttribute, baseType: MlirAttribute, sizeInBits: std::ffi::c_ulong, alignInBits: std::ffi::c_uint, offsetInBits: std::ffi::c_ulong, dwarfAddressSpace: std::ffi::c_long, extraData: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIStringTypeAttrGet(ctx: MlirContext, tag: std::ffi::c_uint, name: MlirAttribute, sizeInBits: std::ffi::c_ulong, alignInBits: std::ffi::c_uint, stringLength: MlirAttribute, stringLengthExp: MlirAttribute, stringLocationExp: MlirAttribute, encoding: EnumMlirLLVMTypeEncoding) -> MlirAttribute;

  pub fn mlirLLVMDIDerivedTypeAttrGetBaseType(diDerivedType: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIFileAttrGet(ctx: MlirContext, name: MlirAttribute, directory: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDICompileUnitAttrGet(ctx: MlirContext, id: MlirAttribute, sourceLanguage: std::ffi::c_uint, file: MlirAttribute, producer: MlirAttribute, isOptimized: u8, emissionKind: EnumMlirLLVMDIEmissionKind, nameTableKind: EnumMlirLLVMDINameTableKind) -> MlirAttribute;

  pub fn mlirLLVMDIFlagsAttrGet(ctx: MlirContext, value: std::ffi::c_ulong) -> MlirAttribute;

  pub fn mlirLLVMDILexicalBlockAttrGet(ctx: MlirContext, scope: MlirAttribute, file: MlirAttribute, line: std::ffi::c_uint, column: std::ffi::c_uint) -> MlirAttribute;

  pub fn mlirLLVMDILexicalBlockFileAttrGet(ctx: MlirContext, scope: MlirAttribute, file: MlirAttribute, discriminator: std::ffi::c_uint) -> MlirAttribute;

  pub fn mlirLLVMDILocalVariableAttrGet(ctx: MlirContext, scope: MlirAttribute, name: MlirAttribute, diFile: MlirAttribute, line: std::ffi::c_uint, arg: std::ffi::c_uint, alignInBits: std::ffi::c_uint, diType: MlirAttribute, flags: std::ffi::c_long) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGetRecSelf(recId: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGet(ctx: MlirContext, recId: MlirAttribute, isRecSelf: u8, id: MlirAttribute, compileUnit: MlirAttribute, scope: MlirAttribute, name: MlirAttribute, linkageName: MlirAttribute, file: MlirAttribute, line: std::ffi::c_uint, scopeLine: std::ffi::c_uint, subprogramFlags: std::ffi::c_ulong, r#type: MlirAttribute, nRetainedNodes: std::ffi::c_long, retainedNodes: *const MlirAttribute, nAnnotations: std::ffi::c_long, annotations: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIAnnotationAttrGet(ctx: MlirContext, name: MlirAttribute, value: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGetScope(diSubprogram: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGetLine(diSubprogram: MlirAttribute) -> std::ffi::c_uint;

  pub fn mlirLLVMDISubprogramAttrGetScopeLine(diSubprogram: MlirAttribute) -> std::ffi::c_uint;

  pub fn mlirLLVMDISubprogramAttrGetCompileUnit(diSubprogram: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGetFile(diSubprogram: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubprogramAttrGetType(diSubprogram: MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDISubroutineTypeAttrGet(ctx: MlirContext, callingConvention: std::ffi::c_uint, nTypes: std::ffi::c_long, types: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIModuleAttrGet(ctx: MlirContext, file: MlirAttribute, scope: MlirAttribute, name: MlirAttribute, configMacros: MlirAttribute, includePath: MlirAttribute, apinotes: MlirAttribute, line: std::ffi::c_uint, isDecl: u8) -> MlirAttribute;

  pub fn mlirLLVMDIImportedEntityAttrGet(ctx: MlirContext, tag: std::ffi::c_uint, scope: MlirAttribute, entity: MlirAttribute, file: MlirAttribute, line: std::ffi::c_uint, name: MlirAttribute, nElements: std::ffi::c_long, elements: *const MlirAttribute) -> MlirAttribute;

  pub fn mlirLLVMDIModuleAttrGetScope(diModule: MlirAttribute) -> MlirAttribute;

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectHandle> {
  pub unsafe fn mlirGetDialectHandle__llvm__()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirGetDialectHandle__llvm__()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMPointerTypeGet<T0_, T1_>(ctx_:  T0_, addressSpace_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMPointerTypeGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(addressSpace_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsALLVMPointerType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirTypeIsALLVMPointerType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirLLVMPointerTypeGetAddressSpace<T0_>(pointerType_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMPointerTypeGetAddressSpace(Into::<MlirType>::into(pointerType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMVoidTypeGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMVoidTypeGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMArrayTypeGet<T0_, T1_>(elementType_:  T0_, numElements_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMArrayTypeGet(Into::<MlirType>::into(elementType_), Into::<std::ffi::c_uint>::into(numElements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMFunctionTypeGet<T0_, T1_, T2_, T3_>(resultType_:  T0_, nArgumentTypes_:  T1_, argumentTypes_:  T2_, isVarArg_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>,  T3_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMFunctionTypeGet(Into::<MlirType>::into(resultType_), Into::<std::ffi::c_long>::into(nArgumentTypes_), Into::<*const MlirType>::into(argumentTypes_), Into::<u8>::into(isVarArg_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsALLVMStructType<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirTypeIsALLVMStructType(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLLVMStructTypeIsLiteral<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeIsLiteral(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirLLVMStructTypeGetNumElementTypes<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeGetNumElementTypes(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeGetElementType<T0_, T1_>(type_:  T0_, position_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeGetElementType(Into::<MlirType>::into(type_), Into::<std::ffi::c_long>::into(position_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLLVMStructTypeIsPacked<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeIsPacked(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirLLVMStructTypeGetIdentifier<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeGetIdentifier(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLLVMStructTypeIsOpaque<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeIsOpaque(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeLiteralGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, nFieldTypes_:  T1_, fieldTypes_:  T2_, isPacked_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>,  T3_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeLiteralGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(nFieldTypes_), Into::<*const MlirType>::into(fieldTypes_), Into::<u8>::into(isPacked_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeLiteralGetChecked<T0_, T1_, T2_, T3_>(loc_:  T0_, nFieldTypes_:  T1_, fieldTypes_:  T2_, isPacked_:  T3_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>,  T3_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeLiteralGetChecked(Into::<MlirLocation>::into(loc_), Into::<std::ffi::c_long>::into(nFieldTypes_), Into::<*const MlirType>::into(fieldTypes_), Into::<u8>::into(isPacked_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeIdentifiedGet<T0_, T1_>(ctx_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeIdentifiedGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeIdentifiedNewGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, name_:  T1_, nFieldTypes_:  T2_, fieldTypes_:  T3_, isPacked_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirType>,  T4_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeIdentifiedNewGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(name_), Into::<std::ffi::c_long>::into(nFieldTypes_), Into::<*const MlirType>::into(fieldTypes_), Into::<u8>::into(isPacked_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirLLVMStructTypeOpaqueGet<T0_, T1_>(ctx_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeOpaqueGet(Into::<MlirContext>::into(ctx_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirLLVMStructTypeSetBody<T0_, T1_, T2_, T3_>(structType_:  T0_, nFieldTypes_:  T1_, fieldTypes_:  T2_, isPacked_:  T3_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>,  T3_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMStructTypeSetBody(Into::<MlirType>::into(structType_), Into::<std::ffi::c_long>::into(nFieldTypes_), Into::<*const MlirType>::into(fieldTypes_), Into::<u8>::into(isPacked_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMCConvAttrGet<T0_, T1_>(ctx_:  T0_, cconv_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<EnumMlirLLVMCConv>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMCConvAttrGet(Into::<MlirContext>::into(ctx_), Into::<EnumMlirLLVMCConv>::into(cconv_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMComdatAttrGet<T0_, T1_>(ctx_:  T0_, comdat_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<EnumMlirLLVMComdat>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMComdatAttrGet(Into::<MlirContext>::into(ctx_), Into::<EnumMlirLLVMComdat>::into(comdat_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMLinkageAttrGet<T0_, T1_>(ctx_:  T0_, linkage_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<EnumMlirLLVMLinkage>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMLinkageAttrGet(Into::<MlirContext>::into(ctx_), Into::<EnumMlirLLVMLinkage>::into(linkage_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDINullTypeAttrGet<T0_>(ctx_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDINullTypeAttrGet(Into::<MlirContext>::into(ctx_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIExpressionElemAttrGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, opcode_:  T1_, nArguments_:  T2_, arguments_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIExpressionElemAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(opcode_), Into::<std::ffi::c_long>::into(nArguments_), Into::<*const std::ffi::c_ulong>::into(arguments_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIExpressionAttrGet<T0_, T1_, T2_>(ctx_:  T0_, nOperations_:  T1_, operations_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIExpressionAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(nOperations_), Into::<*const MlirAttribute>::into(operations_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIBasicTypeAttrGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, tag_:  T1_, name_:  T2_, sizeInBits_:  T3_, encoding_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<MlirAttribute>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<EnumMlirLLVMTypeEncoding>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIBasicTypeAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(tag_), Into::<MlirAttribute>::into(name_), Into::<std::ffi::c_ulong>::into(sizeInBits_), Into::<EnumMlirLLVMTypeEncoding>::into(encoding_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDICompositeTypeAttrGetRecSelf<T0_>(recId_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDICompositeTypeAttrGetRecSelf(Into::<MlirAttribute>::into(recId_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDICompositeTypeAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_, T14_, T15_, T16_, T17_>(ctx_:  T0_, recId_:  T1_, isRecSelf_:  T2_, tag_:  T3_, name_:  T4_, file_:  T5_, line_:  T6_, scope_:  T7_, baseType_:  T8_, flags_:  T9_, sizeInBits_:  T10_, alignInBits_:  T11_, nElements_:  T12_, elements_:  T13_, dataLocation_:  T14_, rank_:  T15_, allocated_:  T16_, associated_:  T17_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<u8>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<MlirAttribute>,  T5_: Into<MlirAttribute>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<MlirAttribute>,  T8_: Into<MlirAttribute>,  T9_: Into<std::ffi::c_long>,  T10_: Into<std::ffi::c_ulong>,  T11_: Into<std::ffi::c_ulong>,  T12_: Into<std::ffi::c_long>,  T13_: Into<*const MlirAttribute>,  T14_: Into<MlirAttribute>,  T15_: Into<MlirAttribute>,  T16_: Into<MlirAttribute>,  T17_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDICompositeTypeAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(recId_), Into::<u8>::into(isRecSelf_), Into::<std::ffi::c_uint>::into(tag_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(file_), Into::<std::ffi::c_uint>::into(line_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(baseType_), Into::<std::ffi::c_long>::into(flags_), Into::<std::ffi::c_ulong>::into(sizeInBits_), Into::<std::ffi::c_ulong>::into(alignInBits_), Into::<std::ffi::c_long>::into(nElements_), Into::<*const MlirAttribute>::into(elements_), Into::<MlirAttribute>::into(dataLocation_), Into::<MlirAttribute>::into(rank_), Into::<MlirAttribute>::into(allocated_), Into::<MlirAttribute>::into(associated_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIDerivedTypeAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, tag_:  T1_, name_:  T2_, baseType_:  T3_, sizeInBits_:  T4_, alignInBits_:  T5_, offsetInBits_:  T6_, dwarfAddressSpace_:  T7_, extraData_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<MlirAttribute>,  T3_: Into<MlirAttribute>,  T4_: Into<std::ffi::c_ulong>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_ulong>,  T7_: Into<std::ffi::c_long>,  T8_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIDerivedTypeAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(tag_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(baseType_), Into::<std::ffi::c_ulong>::into(sizeInBits_), Into::<std::ffi::c_uint>::into(alignInBits_), Into::<std::ffi::c_ulong>::into(offsetInBits_), Into::<std::ffi::c_long>::into(dwarfAddressSpace_), Into::<MlirAttribute>::into(extraData_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIStringTypeAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, tag_:  T1_, name_:  T2_, sizeInBits_:  T3_, alignInBits_:  T4_, stringLength_:  T5_, stringLengthExp_:  T6_, stringLocationExp_:  T7_, encoding_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<MlirAttribute>,  T3_: Into<std::ffi::c_ulong>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<MlirAttribute>,  T6_: Into<MlirAttribute>,  T7_: Into<MlirAttribute>,  T8_: Into<EnumMlirLLVMTypeEncoding>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIStringTypeAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(tag_), Into::<MlirAttribute>::into(name_), Into::<std::ffi::c_ulong>::into(sizeInBits_), Into::<std::ffi::c_uint>::into(alignInBits_), Into::<MlirAttribute>::into(stringLength_), Into::<MlirAttribute>::into(stringLengthExp_), Into::<MlirAttribute>::into(stringLocationExp_), Into::<EnumMlirLLVMTypeEncoding>::into(encoding_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIDerivedTypeAttrGetBaseType<T0_>(diDerivedType_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIDerivedTypeAttrGetBaseType(Into::<MlirAttribute>::into(diDerivedType_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIFileAttrGet<T0_, T1_, T2_>(ctx_:  T0_, name_:  T1_, directory_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIFileAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(directory_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDICompileUnitAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_>(ctx_:  T0_, id_:  T1_, sourceLanguage_:  T2_, file_:  T3_, producer_:  T4_, isOptimized_:  T5_, emissionKind_:  T6_, nameTableKind_:  T7_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<MlirAttribute>,  T4_: Into<MlirAttribute>,  T5_: Into<u8>,  T6_: Into<EnumMlirLLVMDIEmissionKind>,  T7_: Into<EnumMlirLLVMDINameTableKind>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDICompileUnitAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(id_), Into::<std::ffi::c_uint>::into(sourceLanguage_), Into::<MlirAttribute>::into(file_), Into::<MlirAttribute>::into(producer_), Into::<u8>::into(isOptimized_), Into::<EnumMlirLLVMDIEmissionKind>::into(emissionKind_), Into::<EnumMlirLLVMDINameTableKind>::into(nameTableKind_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIFlagsAttrGet<T0_, T1_>(ctx_:  T0_, value_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_ulong>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIFlagsAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_ulong>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDILexicalBlockAttrGet<T0_, T1_, T2_, T3_, T4_>(ctx_:  T0_, scope_:  T1_, file_:  T2_, line_:  T3_, column_:  T4_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>,  T3_: Into<std::ffi::c_uint>,  T4_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDILexicalBlockAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(file_), Into::<std::ffi::c_uint>::into(line_), Into::<std::ffi::c_uint>::into(column_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDILexicalBlockFileAttrGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, scope_:  T1_, file_:  T2_, discriminator_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDILexicalBlockFileAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(file_), Into::<std::ffi::c_uint>::into(discriminator_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDILocalVariableAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, scope_:  T1_, name_:  T2_, diFile_:  T3_, line_:  T4_, arg_:  T5_, alignInBits_:  T6_, diType_:  T7_, flags_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>,  T3_: Into<MlirAttribute>,  T4_: Into<std::ffi::c_uint>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<std::ffi::c_uint>,  T7_: Into<MlirAttribute>,  T8_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDILocalVariableAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(diFile_), Into::<std::ffi::c_uint>::into(line_), Into::<std::ffi::c_uint>::into(arg_), Into::<std::ffi::c_uint>::into(alignInBits_), Into::<MlirAttribute>::into(diType_), Into::<std::ffi::c_long>::into(flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetRecSelf<T0_>(recId_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetRecSelf(Into::<MlirAttribute>::into(recId_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_, T9_, T10_, T11_, T12_, T13_, T14_, T15_, T16_>(ctx_:  T0_, recId_:  T1_, isRecSelf_:  T2_, id_:  T3_, compileUnit_:  T4_, scope_:  T5_, name_:  T6_, linkageName_:  T7_, file_:  T8_, line_:  T9_, scopeLine_:  T10_, subprogramFlags_:  T11_, type_:  T12_, nRetainedNodes_:  T13_, retainedNodes_:  T14_, nAnnotations_:  T15_, annotations_:  T16_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<u8>,  T3_: Into<MlirAttribute>,  T4_: Into<MlirAttribute>,  T5_: Into<MlirAttribute>,  T6_: Into<MlirAttribute>,  T7_: Into<MlirAttribute>,  T8_: Into<MlirAttribute>,  T9_: Into<std::ffi::c_uint>,  T10_: Into<std::ffi::c_uint>,  T11_: Into<std::ffi::c_ulong>,  T12_: Into<MlirAttribute>,  T13_: Into<std::ffi::c_long>,  T14_: Into<*const MlirAttribute>,  T15_: Into<std::ffi::c_long>,  T16_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(recId_), Into::<u8>::into(isRecSelf_), Into::<MlirAttribute>::into(id_), Into::<MlirAttribute>::into(compileUnit_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(linkageName_), Into::<MlirAttribute>::into(file_), Into::<std::ffi::c_uint>::into(line_), Into::<std::ffi::c_uint>::into(scopeLine_), Into::<std::ffi::c_ulong>::into(subprogramFlags_), Into::<MlirAttribute>::into(type_), Into::<std::ffi::c_long>::into(nRetainedNodes_), Into::<*const MlirAttribute>::into(retainedNodes_), Into::<std::ffi::c_long>::into(nAnnotations_), Into::<*const MlirAttribute>::into(annotations_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIAnnotationAttrGet<T0_, T1_, T2_>(ctx_:  T0_, name_:  T1_, value_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIAnnotationAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetScope<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetScope(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetLine<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetLine(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetScopeLine<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetScopeLine(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetCompileUnit<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetCompileUnit(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetFile<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetFile(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubprogramAttrGetType<T0_>(diSubprogram_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubprogramAttrGetType(Into::<MlirAttribute>::into(diSubprogram_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDISubroutineTypeAttrGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, callingConvention_:  T1_, nTypes_:  T2_, types_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDISubroutineTypeAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(callingConvention_), Into::<std::ffi::c_long>::into(nTypes_), Into::<*const MlirAttribute>::into(types_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIModuleAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, file_:  T1_, scope_:  T2_, name_:  T3_, configMacros_:  T4_, includePath_:  T5_, apinotes_:  T6_, line_:  T7_, isDecl_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirAttribute>,  T2_: Into<MlirAttribute>,  T3_: Into<MlirAttribute>,  T4_: Into<MlirAttribute>,  T5_: Into<MlirAttribute>,  T6_: Into<MlirAttribute>,  T7_: Into<std::ffi::c_uint>,  T8_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIModuleAttrGet(Into::<MlirContext>::into(ctx_), Into::<MlirAttribute>::into(file_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(name_), Into::<MlirAttribute>::into(configMacros_), Into::<MlirAttribute>::into(includePath_), Into::<MlirAttribute>::into(apinotes_), Into::<std::ffi::c_uint>::into(line_), Into::<u8>::into(isDecl_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIImportedEntityAttrGet<T0_, T1_, T2_, T3_, T4_, T5_, T6_, T7_, T8_>(ctx_:  T0_, tag_:  T1_, scope_:  T2_, entity_:  T3_, file_:  T4_, line_:  T5_, name_:  T6_, nElements_:  T7_, elements_:  T8_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_uint>,  T2_: Into<MlirAttribute>,  T3_: Into<MlirAttribute>,  T4_: Into<MlirAttribute>,  T5_: Into<std::ffi::c_uint>,  T6_: Into<MlirAttribute>,  T7_: Into<std::ffi::c_long>,  T8_: Into<*const MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIImportedEntityAttrGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_uint>::into(tag_), Into::<MlirAttribute>::into(scope_), Into::<MlirAttribute>::into(entity_), Into::<MlirAttribute>::into(file_), Into::<std::ffi::c_uint>::into(line_), Into::<MlirAttribute>::into(name_), Into::<std::ffi::c_long>::into(nElements_), Into::<*const MlirAttribute>::into(elements_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLLVMDIModuleAttrGetScope<T0_>(diModule_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::Dialect_::LLVM::mlirLLVMDIModuleAttrGetScope(Into::<MlirAttribute>::into(diModule_))
      }
    )
  }
}

