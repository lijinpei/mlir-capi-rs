// Don't Edit! Generated with capi-ffi-gen
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::convert::{From, Into};
use std::marker::PhantomData;
use crate::Support::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirAsmState {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirAsmState = StructMlirAsmState;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirBytecodeWriterConfig {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirBytecodeWriterConfig = StructMlirBytecodeWriterConfig;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirContext {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirContext = StructMlirContext;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirDialect {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirDialect = StructMlirDialect;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirDialectRegistry {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirDialectRegistry = StructMlirDialectRegistry;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirOperation {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirOperation = StructMlirOperation;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirOpOperand {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirOpOperand = StructMlirOpOperand;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirOpPrintingFlags {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirOpPrintingFlags = StructMlirOpPrintingFlags;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirBlock {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirBlock = StructMlirBlock;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirRegion {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirRegion = StructMlirRegion;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirSymbolTable {
  pub ptr: *mut std::ffi::c_void,
}
pub type MlirSymbolTable = StructMlirSymbolTable;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirAttribute {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirAttribute = StructMlirAttribute;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirIdentifier {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirIdentifier = StructMlirIdentifier;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirLocation {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirLocation = StructMlirLocation;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirModule {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirModule = StructMlirModule;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirType {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirType = StructMlirType;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirValue {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirValue = StructMlirValue;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirNamedAttribute {
  pub name: MlirIdentifier,
  pub attribute: MlirAttribute,
}
pub type MlirNamedAttribute = StructMlirNamedAttribute;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirDialectHandle {
  pub ptr: *const std::ffi::c_void,
}
pub type MlirDialectHandle = StructMlirDialectHandle;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StructMlirOperationState {
  pub name: MlirStringRef,
  pub location: MlirLocation,
  pub nResults: std::ffi::c_long,
  pub results: *mut MlirType,
  pub nOperands: std::ffi::c_long,
  pub operands: *mut MlirValue,
  pub nRegions: std::ffi::c_long,
  pub regions: *mut MlirRegion,
  pub nSuccessors: std::ffi::c_long,
  pub successors: *mut MlirBlock,
  pub nAttributes: std::ffi::c_long,
  pub attributes: *mut MlirNamedAttribute,
  pub enableResultTypeInference: u8,
}
pub type MlirOperationState = StructMlirOperationState;

pub const MlirWalkResultAdvance: std::ffi::c_uint = 0;
pub const MlirWalkResultInterrupt: std::ffi::c_uint = 1;
pub const MlirWalkResultSkip: std::ffi::c_uint = 2;
pub type EnumMlirWalkResult = std::ffi::c_uint;
pub type MlirWalkResult = EnumMlirWalkResult;

pub const MlirWalkPreOrder: std::ffi::c_uint = 0;
pub const MlirWalkPostOrder: std::ffi::c_uint = 1;
pub type EnumMlirWalkOrder = std::ffi::c_uint;
pub type MlirWalkOrder = EnumMlirWalkOrder;
pub type MlirOperationWalkCallback = *mut extern fn (MlirOperation, *mut std::ffi::c_void) -> EnumMlirWalkResult;

#[link(name = "MLIR-C")]
extern {

  pub fn mlirContextCreate() -> MlirContext;

  pub fn mlirContextCreateWithThreading(threadingEnabled: u8) -> MlirContext;

  pub fn mlirContextCreateWithRegistry(registry: MlirDialectRegistry, threadingEnabled: u8) -> MlirContext;

  pub fn mlirContextEqual(ctx1: MlirContext, ctx2: MlirContext) -> u8;

  pub fn mlirContextIsNull(context: MlirContext) -> u8;

  pub fn mlirContextDestroy(context: MlirContext) -> ();

  pub fn mlirContextSetAllowUnregisteredDialects(context: MlirContext, allow: u8) -> ();

  pub fn mlirContextGetAllowUnregisteredDialects(context: MlirContext) -> u8;

  pub fn mlirContextGetNumRegisteredDialects(context: MlirContext) -> std::ffi::c_long;

  pub fn mlirContextAppendDialectRegistry(ctx: MlirContext, registry: MlirDialectRegistry) -> ();

  pub fn mlirContextGetNumLoadedDialects(context: MlirContext) -> std::ffi::c_long;

  pub fn mlirContextGetOrLoadDialect(context: MlirContext, name: MlirStringRef) -> MlirDialect;

  pub fn mlirContextEnableMultithreading(context: MlirContext, enable: u8) -> ();

  pub fn mlirContextLoadAllAvailableDialects(context: MlirContext) -> ();

  pub fn mlirContextIsRegisteredOperation(context: MlirContext, name: MlirStringRef) -> u8;

  pub fn mlirContextSetThreadPool(context: MlirContext, threadPool: MlirLlvmThreadPool) -> ();

  pub fn mlirDialectGetContext(dialect: MlirDialect) -> MlirContext;

  pub fn mlirDialectIsNull(dialect: MlirDialect) -> u8;

  pub fn mlirDialectEqual(dialect1: MlirDialect, dialect2: MlirDialect) -> u8;

  pub fn mlirDialectGetNamespace(dialect: MlirDialect) -> MlirStringRef;

  pub fn mlirDialectHandleGetNamespace(_: MlirDialectHandle) -> MlirStringRef;

  pub fn mlirDialectHandleInsertDialect(_: MlirDialectHandle, _: MlirDialectRegistry) -> ();

  pub fn mlirDialectHandleRegisterDialect(_: MlirDialectHandle, _: MlirContext) -> ();

  pub fn mlirDialectHandleLoadDialect(_: MlirDialectHandle, _: MlirContext) -> MlirDialect;

  pub fn mlirDialectRegistryCreate() -> MlirDialectRegistry;

  pub fn mlirDialectRegistryIsNull(registry: MlirDialectRegistry) -> u8;

  pub fn mlirDialectRegistryDestroy(registry: MlirDialectRegistry) -> ();

  pub fn mlirLocationGetAttribute(location: MlirLocation) -> MlirAttribute;

  pub fn mlirLocationFromAttribute(attribute: MlirAttribute) -> MlirLocation;

  pub fn mlirLocationFileLineColGet(context: MlirContext, filename: MlirStringRef, line: std::ffi::c_uint, col: std::ffi::c_uint) -> MlirLocation;

  pub fn mlirLocationCallSiteGet(callee: MlirLocation, caller: MlirLocation) -> MlirLocation;

  pub fn mlirLocationFusedGet(ctx: MlirContext, nLocations: std::ffi::c_long, locations: *const MlirLocation, metadata: MlirAttribute) -> MlirLocation;

  pub fn mlirLocationNameGet(context: MlirContext, name: MlirStringRef, childLoc: MlirLocation) -> MlirLocation;

  pub fn mlirLocationUnknownGet(context: MlirContext) -> MlirLocation;

  pub fn mlirLocationGetContext(location: MlirLocation) -> MlirContext;

  pub fn mlirLocationIsNull(location: MlirLocation) -> u8;

  pub fn mlirLocationEqual(l1: MlirLocation, l2: MlirLocation) -> u8;

  pub fn mlirLocationPrint(location: MlirLocation, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirModuleCreateEmpty(location: MlirLocation) -> MlirModule;

  pub fn mlirModuleCreateParse(context: MlirContext, module: MlirStringRef) -> MlirModule;

  pub fn mlirModuleGetContext(module: MlirModule) -> MlirContext;

  pub fn mlirModuleGetBody(module: MlirModule) -> MlirBlock;

  pub fn mlirModuleIsNull(module: MlirModule) -> u8;

  pub fn mlirModuleDestroy(module: MlirModule) -> ();

  pub fn mlirModuleGetOperation(module: MlirModule) -> MlirOperation;

  pub fn mlirModuleFromOperation(op: MlirOperation) -> MlirModule;

  pub fn mlirOperationStateGet(name: MlirStringRef, loc: MlirLocation) -> MlirOperationState;

  pub fn mlirOperationStateAddResults(state: *mut MlirOperationState, n: std::ffi::c_long, results: *const MlirType) -> ();

  pub fn mlirOperationStateAddOperands(state: *mut MlirOperationState, n: std::ffi::c_long, operands: *const MlirValue) -> ();

  pub fn mlirOperationStateAddOwnedRegions(state: *mut MlirOperationState, n: std::ffi::c_long, regions: *const MlirRegion) -> ();

  pub fn mlirOperationStateAddSuccessors(state: *mut MlirOperationState, n: std::ffi::c_long, successors: *const MlirBlock) -> ();

  pub fn mlirOperationStateAddAttributes(state: *mut MlirOperationState, n: std::ffi::c_long, attributes: *const MlirNamedAttribute) -> ();

  pub fn mlirOperationStateEnableResultTypeInference(state: *mut MlirOperationState) -> ();

  pub fn mlirAsmStateCreateForOperation(op: MlirOperation, flags: MlirOpPrintingFlags) -> MlirAsmState;

  pub fn mlirAsmStateCreateForValue(value: MlirValue, flags: MlirOpPrintingFlags) -> MlirAsmState;

  pub fn mlirAsmStateDestroy(state: MlirAsmState) -> ();

  pub fn mlirOpPrintingFlagsCreate() -> MlirOpPrintingFlags;

  pub fn mlirOpPrintingFlagsDestroy(flags: MlirOpPrintingFlags) -> ();

  pub fn mlirOpPrintingFlagsElideLargeElementsAttrs(flags: MlirOpPrintingFlags, largeElementLimit: std::ffi::c_long) -> ();

  pub fn mlirOpPrintingFlagsElideLargeResourceString(flags: MlirOpPrintingFlags, largeResourceLimit: std::ffi::c_long) -> ();

  pub fn mlirOpPrintingFlagsEnableDebugInfo(flags: MlirOpPrintingFlags, enable: u8, prettyForm: u8) -> ();

  pub fn mlirOpPrintingFlagsPrintGenericOpForm(flags: MlirOpPrintingFlags) -> ();

  pub fn mlirOpPrintingFlagsUseLocalScope(flags: MlirOpPrintingFlags) -> ();

  pub fn mlirOpPrintingFlagsAssumeVerified(flags: MlirOpPrintingFlags) -> ();

  pub fn mlirOpPrintingFlagsSkipRegions(flags: MlirOpPrintingFlags) -> ();

  pub fn mlirBytecodeWriterConfigCreate() -> MlirBytecodeWriterConfig;

  pub fn mlirBytecodeWriterConfigDestroy(config: MlirBytecodeWriterConfig) -> ();

  pub fn mlirBytecodeWriterConfigDesiredEmitVersion(flags: MlirBytecodeWriterConfig, version: std::ffi::c_long) -> ();

  pub fn mlirOperationCreate(state: *mut MlirOperationState) -> MlirOperation;

  pub fn mlirOperationCreateParse(context: MlirContext, sourceStr: MlirStringRef, sourceName: MlirStringRef) -> MlirOperation;

  pub fn mlirOperationClone(op: MlirOperation) -> MlirOperation;

  pub fn mlirOperationDestroy(op: MlirOperation) -> ();

  pub fn mlirOperationRemoveFromParent(op: MlirOperation) -> ();

  pub fn mlirOperationIsNull(op: MlirOperation) -> u8;

  pub fn mlirOperationEqual(op: MlirOperation, other: MlirOperation) -> u8;

  pub fn mlirOperationGetContext(op: MlirOperation) -> MlirContext;

  pub fn mlirOperationGetLocation(op: MlirOperation) -> MlirLocation;

  pub fn mlirOperationGetTypeID(op: MlirOperation) -> MlirTypeID;

  pub fn mlirOperationGetName(op: MlirOperation) -> MlirIdentifier;

  pub fn mlirOperationGetBlock(op: MlirOperation) -> MlirBlock;

  pub fn mlirOperationGetParentOperation(op: MlirOperation) -> MlirOperation;

  pub fn mlirOperationGetNumRegions(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetRegion(op: MlirOperation, pos: std::ffi::c_long) -> MlirRegion;

  pub fn mlirOperationGetNextInBlock(op: MlirOperation) -> MlirOperation;

  pub fn mlirOperationGetNumOperands(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetOperand(op: MlirOperation, pos: std::ffi::c_long) -> MlirValue;

  pub fn mlirOperationSetOperand(op: MlirOperation, pos: std::ffi::c_long, newValue: MlirValue) -> ();

  pub fn mlirOperationSetOperands(op: MlirOperation, nOperands: std::ffi::c_long, operands: *const MlirValue) -> ();

  pub fn mlirOperationGetNumResults(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetResult(op: MlirOperation, pos: std::ffi::c_long) -> MlirValue;

  pub fn mlirOperationGetNumSuccessors(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetSuccessor(op: MlirOperation, pos: std::ffi::c_long) -> MlirBlock;

  pub fn mlirOperationSetSuccessor(op: MlirOperation, pos: std::ffi::c_long, block: MlirBlock) -> ();

  pub fn mlirOperationHasInherentAttributeByName(op: MlirOperation, name: MlirStringRef) -> u8;

  pub fn mlirOperationGetInherentAttributeByName(op: MlirOperation, name: MlirStringRef) -> MlirAttribute;

  pub fn mlirOperationSetInherentAttributeByName(op: MlirOperation, name: MlirStringRef, attr: MlirAttribute) -> ();

  pub fn mlirOperationGetNumDiscardableAttributes(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetDiscardableAttribute(op: MlirOperation, pos: std::ffi::c_long) -> MlirNamedAttribute;

  pub fn mlirOperationGetDiscardableAttributeByName(op: MlirOperation, name: MlirStringRef) -> MlirAttribute;

  pub fn mlirOperationSetDiscardableAttributeByName(op: MlirOperation, name: MlirStringRef, attr: MlirAttribute) -> ();

  pub fn mlirOperationRemoveDiscardableAttributeByName(op: MlirOperation, name: MlirStringRef) -> u8;

  pub fn mlirOperationGetNumAttributes(op: MlirOperation) -> std::ffi::c_long;

  pub fn mlirOperationGetAttribute(op: MlirOperation, pos: std::ffi::c_long) -> MlirNamedAttribute;

  pub fn mlirOperationGetAttributeByName(op: MlirOperation, name: MlirStringRef) -> MlirAttribute;

  pub fn mlirOperationSetAttributeByName(op: MlirOperation, name: MlirStringRef, attr: MlirAttribute) -> ();

  pub fn mlirOperationRemoveAttributeByName(op: MlirOperation, name: MlirStringRef) -> u8;

  pub fn mlirOperationPrint(op: MlirOperation, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirOperationPrintWithFlags(op: MlirOperation, flags: MlirOpPrintingFlags, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirOperationPrintWithState(op: MlirOperation, state: MlirAsmState, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirOperationWriteBytecode(op: MlirOperation, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirOperationWriteBytecodeWithConfig(op: MlirOperation, config: MlirBytecodeWriterConfig, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> MlirLogicalResult;

  pub fn mlirOperationDump(op: MlirOperation) -> ();

  pub fn mlirOperationVerify(op: MlirOperation) -> u8;

  pub fn mlirOperationMoveAfter(op: MlirOperation, other: MlirOperation) -> ();

  pub fn mlirOperationMoveBefore(op: MlirOperation, other: MlirOperation) -> ();

  pub fn mlirOperationWalk(op: MlirOperation, callback: MlirOperationWalkCallback, userData: *mut std::ffi::c_void, walkOrder: EnumMlirWalkOrder) -> ();

  pub fn mlirRegionCreate() -> MlirRegion;

  pub fn mlirRegionDestroy(region: MlirRegion) -> ();

  pub fn mlirRegionIsNull(region: MlirRegion) -> u8;

  pub fn mlirRegionEqual(region: MlirRegion, other: MlirRegion) -> u8;

  pub fn mlirRegionGetFirstBlock(region: MlirRegion) -> MlirBlock;

  pub fn mlirRegionAppendOwnedBlock(region: MlirRegion, block: MlirBlock) -> ();

  pub fn mlirRegionInsertOwnedBlock(region: MlirRegion, pos: std::ffi::c_long, block: MlirBlock) -> ();

  pub fn mlirRegionInsertOwnedBlockAfter(region: MlirRegion, reference: MlirBlock, block: MlirBlock) -> ();

  pub fn mlirRegionInsertOwnedBlockBefore(region: MlirRegion, reference: MlirBlock, block: MlirBlock) -> ();

  pub fn mlirOperationGetFirstRegion(op: MlirOperation) -> MlirRegion;

  pub fn mlirRegionGetNextInOperation(region: MlirRegion) -> MlirRegion;

  pub fn mlirRegionTakeBody(target: MlirRegion, source: MlirRegion) -> ();

  pub fn mlirBlockCreate(nArgs: std::ffi::c_long, args: *const MlirType, locs: *const MlirLocation) -> MlirBlock;

  pub fn mlirBlockDestroy(block: MlirBlock) -> ();

  pub fn mlirBlockDetach(block: MlirBlock) -> ();

  pub fn mlirBlockIsNull(block: MlirBlock) -> u8;

  pub fn mlirBlockEqual(block: MlirBlock, other: MlirBlock) -> u8;

  pub fn mlirBlockGetParentOperation(_: MlirBlock) -> MlirOperation;

  pub fn mlirBlockGetParentRegion(block: MlirBlock) -> MlirRegion;

  pub fn mlirBlockGetNextInRegion(block: MlirBlock) -> MlirBlock;

  pub fn mlirBlockGetFirstOperation(block: MlirBlock) -> MlirOperation;

  pub fn mlirBlockGetTerminator(block: MlirBlock) -> MlirOperation;

  pub fn mlirBlockAppendOwnedOperation(block: MlirBlock, operation: MlirOperation) -> ();

  pub fn mlirBlockInsertOwnedOperation(block: MlirBlock, pos: std::ffi::c_long, operation: MlirOperation) -> ();

  pub fn mlirBlockInsertOwnedOperationAfter(block: MlirBlock, reference: MlirOperation, operation: MlirOperation) -> ();

  pub fn mlirBlockInsertOwnedOperationBefore(block: MlirBlock, reference: MlirOperation, operation: MlirOperation) -> ();

  pub fn mlirBlockGetNumArguments(block: MlirBlock) -> std::ffi::c_long;

  pub fn mlirBlockAddArgument(block: MlirBlock, r#type: MlirType, loc: MlirLocation) -> MlirValue;

  pub fn mlirBlockEraseArgument(block: MlirBlock, index: std::ffi::c_uint) -> ();

  pub fn mlirBlockInsertArgument(block: MlirBlock, pos: std::ffi::c_long, r#type: MlirType, loc: MlirLocation) -> MlirValue;

  pub fn mlirBlockGetArgument(block: MlirBlock, pos: std::ffi::c_long) -> MlirValue;

  pub fn mlirBlockPrint(block: MlirBlock, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirValueIsNull(value: MlirValue) -> u8;

  pub fn mlirValueEqual(value1: MlirValue, value2: MlirValue) -> u8;

  pub fn mlirValueIsABlockArgument(value: MlirValue) -> u8;

  pub fn mlirValueIsAOpResult(value: MlirValue) -> u8;

  pub fn mlirBlockArgumentGetOwner(value: MlirValue) -> MlirBlock;

  pub fn mlirBlockArgumentGetArgNumber(value: MlirValue) -> std::ffi::c_long;

  pub fn mlirBlockArgumentSetType(value: MlirValue, r#type: MlirType) -> ();

  pub fn mlirOpResultGetOwner(value: MlirValue) -> MlirOperation;

  pub fn mlirOpResultGetResultNumber(value: MlirValue) -> std::ffi::c_long;

  pub fn mlirValueGetType(value: MlirValue) -> MlirType;

  pub fn mlirValueSetType(value: MlirValue, r#type: MlirType) -> ();

  pub fn mlirValueDump(value: MlirValue) -> ();

  pub fn mlirValuePrint(value: MlirValue, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirValuePrintAsOperand(value: MlirValue, state: MlirAsmState, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirValueGetFirstUse(value: MlirValue) -> MlirOpOperand;

  pub fn mlirValueReplaceAllUsesOfWith(of: MlirValue, with: MlirValue) -> ();

  pub fn mlirValueReplaceAllUsesExcept(of: MlirValue, with: MlirValue, numExceptions: std::ffi::c_long, exceptions: *mut MlirOperation) -> ();

  pub fn mlirOpOperandIsNull(opOperand: MlirOpOperand) -> u8;

  pub fn mlirOpOperandGetValue(opOperand: MlirOpOperand) -> MlirValue;

  pub fn mlirOpOperandGetOwner(opOperand: MlirOpOperand) -> MlirOperation;

  pub fn mlirOpOperandGetOperandNumber(opOperand: MlirOpOperand) -> std::ffi::c_uint;

  pub fn mlirOpOperandGetNextUse(opOperand: MlirOpOperand) -> MlirOpOperand;

  pub fn mlirTypeParseGet(context: MlirContext, r#type: MlirStringRef) -> MlirType;

  pub fn mlirTypeGetContext(r#type: MlirType) -> MlirContext;

  pub fn mlirTypeGetTypeID(r#type: MlirType) -> MlirTypeID;

  pub fn mlirTypeGetDialect(r#type: MlirType) -> MlirDialect;

  pub fn mlirTypeIsNull(r#type: MlirType) -> u8;

  pub fn mlirTypeEqual(t1: MlirType, t2: MlirType) -> u8;

  pub fn mlirTypePrint(r#type: MlirType, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirTypeDump(r#type: MlirType) -> ();

  pub fn mlirAttributeParseGet(context: MlirContext, attr: MlirStringRef) -> MlirAttribute;

  pub fn mlirAttributeGetContext(attribute: MlirAttribute) -> MlirContext;

  pub fn mlirAttributeGetType(attribute: MlirAttribute) -> MlirType;

  pub fn mlirAttributeGetTypeID(attribute: MlirAttribute) -> MlirTypeID;

  pub fn mlirAttributeGetDialect(attribute: MlirAttribute) -> MlirDialect;

  pub fn mlirAttributeIsNull(attr: MlirAttribute) -> u8;

  pub fn mlirAttributeEqual(a1: MlirAttribute, a2: MlirAttribute) -> u8;

  pub fn mlirAttributePrint(attr: MlirAttribute, callback: MlirStringCallback, userData: *mut std::ffi::c_void) -> ();

  pub fn mlirAttributeDump(attr: MlirAttribute) -> ();

  pub fn mlirNamedAttributeGet(name: MlirIdentifier, attr: MlirAttribute) -> MlirNamedAttribute;

  pub fn mlirIdentifierGet(context: MlirContext, str: MlirStringRef) -> MlirIdentifier;

  pub fn mlirIdentifierGetContext(_: MlirIdentifier) -> MlirContext;

  pub fn mlirIdentifierEqual(ident: MlirIdentifier, other: MlirIdentifier) -> u8;

  pub fn mlirIdentifierStr(ident: MlirIdentifier) -> MlirStringRef;

  pub fn mlirSymbolTableGetSymbolAttributeName() -> MlirStringRef;

  pub fn mlirSymbolTableGetVisibilityAttributeName() -> MlirStringRef;

  pub fn mlirSymbolTableCreate(operation: MlirOperation) -> MlirSymbolTable;

  pub fn mlirSymbolTableIsNull(symbolTable: MlirSymbolTable) -> u8;

  pub fn mlirSymbolTableDestroy(symbolTable: MlirSymbolTable) -> ();

  pub fn mlirSymbolTableLookup(symbolTable: MlirSymbolTable, name: MlirStringRef) -> MlirOperation;

  pub fn mlirSymbolTableInsert(symbolTable: MlirSymbolTable, operation: MlirOperation) -> MlirAttribute;

  pub fn mlirSymbolTableErase(symbolTable: MlirSymbolTable, operation: MlirOperation) -> ();

  pub fn mlirSymbolTableReplaceAllSymbolUses(oldSymbol: MlirStringRef, newSymbol: MlirStringRef, from: MlirOperation) -> MlirLogicalResult;

  pub fn mlirSymbolTableWalkSymbolTables(from: MlirOperation, allSymUsesVisible: u8, callback: *mut extern fn (MlirOperation, u8, *mut std::ffi::c_void) -> (), userData: *mut std::ffi::c_void) -> ();

}

pub struct FFIVal_<Tret_> {e_: PhantomData<Tret_>,}
pub struct FFIVoid_;

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirContextCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextCreate()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirContextCreateWithThreading<T0_>(threadingEnabled_:  T0_)-> Tret_
  where
     T0_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextCreateWithThreading(Into::<u8>::into(threadingEnabled_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirContextCreateWithRegistry<T0_, T1_>(registry_:  T0_, threadingEnabled_:  T1_)-> Tret_
  where
     T0_: Into<MlirDialectRegistry>,  T1_: Into<u8>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextCreateWithRegistry(Into::<MlirDialectRegistry>::into(registry_), Into::<u8>::into(threadingEnabled_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirContextEqual<T0_, T1_>(ctx1_:  T0_, ctx2_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextEqual(Into::<MlirContext>::into(ctx1_), Into::<MlirContext>::into(ctx2_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirContextIsNull<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextIsNull(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextDestroy<T0_>(context_:  T0_)
  where
     T0_: Into<MlirContext>
  {
    unsafe {
      crate::IR::mlirContextDestroy(Into::<MlirContext>::into(context_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextSetAllowUnregisteredDialects<T0_, T1_>(context_:  T0_, allow_:  T1_)
  where
     T0_: Into<MlirContext>,  T1_: Into<u8>
  {
    unsafe {
      crate::IR::mlirContextSetAllowUnregisteredDialects(Into::<MlirContext>::into(context_), Into::<u8>::into(allow_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirContextGetAllowUnregisteredDialects<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextGetAllowUnregisteredDialects(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirContextGetNumRegisteredDialects<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextGetNumRegisteredDialects(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextAppendDialectRegistry<T0_, T1_>(ctx_:  T0_, registry_:  T1_)
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirDialectRegistry>
  {
    unsafe {
      crate::IR::mlirContextAppendDialectRegistry(Into::<MlirContext>::into(ctx_), Into::<MlirDialectRegistry>::into(registry_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirContextGetNumLoadedDialects<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextGetNumLoadedDialects(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialect> {
  pub unsafe fn mlirContextGetOrLoadDialect<T0_, T1_>(context_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextGetOrLoadDialect(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextEnableMultithreading<T0_, T1_>(context_:  T0_, enable_:  T1_)
  where
     T0_: Into<MlirContext>,  T1_: Into<u8>
  {
    unsafe {
      crate::IR::mlirContextEnableMultithreading(Into::<MlirContext>::into(context_), Into::<u8>::into(enable_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextLoadAllAvailableDialects<T0_>(context_:  T0_)
  where
     T0_: Into<MlirContext>
  {
    unsafe {
      crate::IR::mlirContextLoadAllAvailableDialects(Into::<MlirContext>::into(context_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirContextIsRegisteredOperation<T0_, T1_>(context_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirContextIsRegisteredOperation(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirContextSetThreadPool<T0_, T1_>(context_:  T0_, threadPool_:  T1_)
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirLlvmThreadPool>
  {
    unsafe {
      crate::IR::mlirContextSetThreadPool(Into::<MlirContext>::into(context_), Into::<MlirLlvmThreadPool>::into(threadPool_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirDialectGetContext<T0_>(dialect_:  T0_)-> Tret_
  where
     T0_: Into<MlirDialect>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectGetContext(Into::<MlirDialect>::into(dialect_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDialectIsNull<T0_>(dialect_:  T0_)-> Tret_
  where
     T0_: Into<MlirDialect>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectIsNull(Into::<MlirDialect>::into(dialect_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDialectEqual<T0_, T1_>(dialect1_:  T0_, dialect2_:  T1_)-> Tret_
  where
     T0_: Into<MlirDialect>,  T1_: Into<MlirDialect>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectEqual(Into::<MlirDialect>::into(dialect1_), Into::<MlirDialect>::into(dialect2_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirDialectGetNamespace<T0_>(dialect_:  T0_)-> Tret_
  where
     T0_: Into<MlirDialect>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectGetNamespace(Into::<MlirDialect>::into(dialect_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirDialectHandleGetNamespace<T0_>(arg0_:  T0_)-> Tret_
  where
     T0_: Into<MlirDialectHandle>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectHandleGetNamespace(Into::<MlirDialectHandle>::into(arg0_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirDialectHandleInsertDialect<T0_, T1_>(arg0_:  T0_, arg1_:  T1_)
  where
     T0_: Into<MlirDialectHandle>,  T1_: Into<MlirDialectRegistry>
  {
    unsafe {
      crate::IR::mlirDialectHandleInsertDialect(Into::<MlirDialectHandle>::into(arg0_), Into::<MlirDialectRegistry>::into(arg1_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirDialectHandleRegisterDialect<T0_, T1_>(arg0_:  T0_, arg1_:  T1_)
  where
     T0_: Into<MlirDialectHandle>,  T1_: Into<MlirContext>
  {
    unsafe {
      crate::IR::mlirDialectHandleRegisterDialect(Into::<MlirDialectHandle>::into(arg0_), Into::<MlirContext>::into(arg1_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialect> {
  pub unsafe fn mlirDialectHandleLoadDialect<T0_, T1_>(arg0_:  T0_, arg1_:  T1_)-> Tret_
  where
     T0_: Into<MlirDialectHandle>,  T1_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectHandleLoadDialect(Into::<MlirDialectHandle>::into(arg0_), Into::<MlirContext>::into(arg1_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialectRegistry> {
  pub unsafe fn mlirDialectRegistryCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectRegistryCreate()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirDialectRegistryIsNull<T0_>(registry_:  T0_)-> Tret_
  where
     T0_: Into<MlirDialectRegistry>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirDialectRegistryIsNull(Into::<MlirDialectRegistry>::into(registry_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirDialectRegistryDestroy<T0_>(registry_:  T0_)
  where
     T0_: Into<MlirDialectRegistry>
  {
    unsafe {
      crate::IR::mlirDialectRegistryDestroy(Into::<MlirDialectRegistry>::into(registry_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirLocationGetAttribute<T0_>(location_:  T0_)-> Tret_
  where
     T0_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationGetAttribute(Into::<MlirLocation>::into(location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationFromAttribute<T0_>(attribute_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationFromAttribute(Into::<MlirAttribute>::into(attribute_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationFileLineColGet<T0_, T1_, T2_, T3_>(context_:  T0_, filename_:  T1_, line_:  T2_, col_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<std::ffi::c_uint>,  T3_: Into<std::ffi::c_uint>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationFileLineColGet(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(filename_), Into::<std::ffi::c_uint>::into(line_), Into::<std::ffi::c_uint>::into(col_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationCallSiteGet<T0_, T1_>(callee_:  T0_, caller_:  T1_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationCallSiteGet(Into::<MlirLocation>::into(callee_), Into::<MlirLocation>::into(caller_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationFusedGet<T0_, T1_, T2_, T3_>(ctx_:  T0_, nLocations_:  T1_, locations_:  T2_, metadata_:  T3_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirLocation>,  T3_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationFusedGet(Into::<MlirContext>::into(ctx_), Into::<std::ffi::c_long>::into(nLocations_), Into::<*const MlirLocation>::into(locations_), Into::<MlirAttribute>::into(metadata_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationNameGet<T0_, T1_, T2_>(context_:  T0_, name_:  T1_, childLoc_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationNameGet(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(name_), Into::<MlirLocation>::into(childLoc_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirLocationUnknownGet<T0_>(context_:  T0_)-> Tret_
  where
     T0_: Into<MlirContext>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationUnknownGet(Into::<MlirContext>::into(context_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirLocationGetContext<T0_>(location_:  T0_)-> Tret_
  where
     T0_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationGetContext(Into::<MlirLocation>::into(location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLocationIsNull<T0_>(location_:  T0_)-> Tret_
  where
     T0_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationIsNull(Into::<MlirLocation>::into(location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirLocationEqual<T0_, T1_>(l1_:  T0_, l2_:  T1_)-> Tret_
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirLocationEqual(Into::<MlirLocation>::into(l1_), Into::<MlirLocation>::into(l2_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirLocationPrint<T0_, T1_, T2_>(location_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirLocation>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirLocationPrint(Into::<MlirLocation>::into(location_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirModule> {
  pub unsafe fn mlirModuleCreateEmpty<T0_>(location_:  T0_)-> Tret_
  where
     T0_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleCreateEmpty(Into::<MlirLocation>::into(location_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirModule> {
  pub unsafe fn mlirModuleCreateParse<T0_, T1_>(context_:  T0_, module_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleCreateParse(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirModuleGetContext<T0_>(module_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleGetContext(Into::<MlirModule>::into(module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirModuleGetBody<T0_>(module_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleGetBody(Into::<MlirModule>::into(module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirModuleIsNull<T0_>(module_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleIsNull(Into::<MlirModule>::into(module_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirModuleDestroy<T0_>(module_:  T0_)
  where
     T0_: Into<MlirModule>
  {
    unsafe {
      crate::IR::mlirModuleDestroy(Into::<MlirModule>::into(module_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirModuleGetOperation<T0_>(module_:  T0_)-> Tret_
  where
     T0_: Into<MlirModule>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleGetOperation(Into::<MlirModule>::into(module_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirModule> {
  pub unsafe fn mlirModuleFromOperation<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirModuleFromOperation(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperationState> {
  pub unsafe fn mlirOperationStateGet<T0_, T1_>(name_:  T0_, loc_:  T1_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationStateGet(Into::<MlirStringRef>::into(name_), Into::<MlirLocation>::into(loc_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateAddResults<T0_, T1_, T2_>(state_:  T0_, n_:  T1_, results_:  T2_)
  where
     T0_: Into<*mut MlirOperationState>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirType>
  {
    unsafe {
      crate::IR::mlirOperationStateAddResults(Into::<*mut MlirOperationState>::into(state_), Into::<std::ffi::c_long>::into(n_), Into::<*const MlirType>::into(results_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateAddOperands<T0_, T1_, T2_>(state_:  T0_, n_:  T1_, operands_:  T2_)
  where
     T0_: Into<*mut MlirOperationState>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirValue>
  {
    unsafe {
      crate::IR::mlirOperationStateAddOperands(Into::<*mut MlirOperationState>::into(state_), Into::<std::ffi::c_long>::into(n_), Into::<*const MlirValue>::into(operands_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateAddOwnedRegions<T0_, T1_, T2_>(state_:  T0_, n_:  T1_, regions_:  T2_)
  where
     T0_: Into<*mut MlirOperationState>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirRegion>
  {
    unsafe {
      crate::IR::mlirOperationStateAddOwnedRegions(Into::<*mut MlirOperationState>::into(state_), Into::<std::ffi::c_long>::into(n_), Into::<*const MlirRegion>::into(regions_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateAddSuccessors<T0_, T1_, T2_>(state_:  T0_, n_:  T1_, successors_:  T2_)
  where
     T0_: Into<*mut MlirOperationState>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirBlock>
  {
    unsafe {
      crate::IR::mlirOperationStateAddSuccessors(Into::<*mut MlirOperationState>::into(state_), Into::<std::ffi::c_long>::into(n_), Into::<*const MlirBlock>::into(successors_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateAddAttributes<T0_, T1_, T2_>(state_:  T0_, n_:  T1_, attributes_:  T2_)
  where
     T0_: Into<*mut MlirOperationState>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirNamedAttribute>
  {
    unsafe {
      crate::IR::mlirOperationStateAddAttributes(Into::<*mut MlirOperationState>::into(state_), Into::<std::ffi::c_long>::into(n_), Into::<*const MlirNamedAttribute>::into(attributes_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationStateEnableResultTypeInference<T0_>(state_:  T0_)
  where
     T0_: Into<*mut MlirOperationState>
  {
    unsafe {
      crate::IR::mlirOperationStateEnableResultTypeInference(Into::<*mut MlirOperationState>::into(state_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAsmState> {
  pub unsafe fn mlirAsmStateCreateForOperation<T0_, T1_>(op_:  T0_, flags_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOpPrintingFlags>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAsmStateCreateForOperation(Into::<MlirOperation>::into(op_), Into::<MlirOpPrintingFlags>::into(flags_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAsmState> {
  pub unsafe fn mlirAsmStateCreateForValue<T0_, T1_>(value_:  T0_, flags_:  T1_)-> Tret_
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirOpPrintingFlags>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAsmStateCreateForValue(Into::<MlirValue>::into(value_), Into::<MlirOpPrintingFlags>::into(flags_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAsmStateDestroy<T0_>(state_:  T0_)
  where
     T0_: Into<MlirAsmState>
  {
    unsafe {
      crate::IR::mlirAsmStateDestroy(Into::<MlirAsmState>::into(state_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpPrintingFlags> {
  pub unsafe fn mlirOpPrintingFlagsCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpPrintingFlagsCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsDestroy<T0_>(flags_:  T0_)
  where
     T0_: Into<MlirOpPrintingFlags>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsDestroy(Into::<MlirOpPrintingFlags>::into(flags_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsElideLargeElementsAttrs<T0_, T1_>(flags_:  T0_, largeElementLimit_:  T1_)
  where
     T0_: Into<MlirOpPrintingFlags>,  T1_: Into<std::ffi::c_long>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsElideLargeElementsAttrs(Into::<MlirOpPrintingFlags>::into(flags_), Into::<std::ffi::c_long>::into(largeElementLimit_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsElideLargeResourceString<T0_, T1_>(flags_:  T0_, largeResourceLimit_:  T1_)
  where
     T0_: Into<MlirOpPrintingFlags>,  T1_: Into<std::ffi::c_long>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsElideLargeResourceString(Into::<MlirOpPrintingFlags>::into(flags_), Into::<std::ffi::c_long>::into(largeResourceLimit_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsEnableDebugInfo<T0_, T1_, T2_>(flags_:  T0_, enable_:  T1_, prettyForm_:  T2_)
  where
     T0_: Into<MlirOpPrintingFlags>,  T1_: Into<u8>,  T2_: Into<u8>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsEnableDebugInfo(Into::<MlirOpPrintingFlags>::into(flags_), Into::<u8>::into(enable_), Into::<u8>::into(prettyForm_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsPrintGenericOpForm<T0_>(flags_:  T0_)
  where
     T0_: Into<MlirOpPrintingFlags>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsPrintGenericOpForm(Into::<MlirOpPrintingFlags>::into(flags_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsUseLocalScope<T0_>(flags_:  T0_)
  where
     T0_: Into<MlirOpPrintingFlags>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsUseLocalScope(Into::<MlirOpPrintingFlags>::into(flags_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsAssumeVerified<T0_>(flags_:  T0_)
  where
     T0_: Into<MlirOpPrintingFlags>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsAssumeVerified(Into::<MlirOpPrintingFlags>::into(flags_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOpPrintingFlagsSkipRegions<T0_>(flags_:  T0_)
  where
     T0_: Into<MlirOpPrintingFlags>
  {
    unsafe {
      crate::IR::mlirOpPrintingFlagsSkipRegions(Into::<MlirOpPrintingFlags>::into(flags_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBytecodeWriterConfig> {
  pub unsafe fn mlirBytecodeWriterConfigCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBytecodeWriterConfigCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBytecodeWriterConfigDestroy<T0_>(config_:  T0_)
  where
     T0_: Into<MlirBytecodeWriterConfig>
  {
    unsafe {
      crate::IR::mlirBytecodeWriterConfigDestroy(Into::<MlirBytecodeWriterConfig>::into(config_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBytecodeWriterConfigDesiredEmitVersion<T0_, T1_>(flags_:  T0_, version_:  T1_)
  where
     T0_: Into<MlirBytecodeWriterConfig>,  T1_: Into<std::ffi::c_long>
  {
    unsafe {
      crate::IR::mlirBytecodeWriterConfigDesiredEmitVersion(Into::<MlirBytecodeWriterConfig>::into(flags_), Into::<std::ffi::c_long>::into(version_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOperationCreate<T0_>(state_:  T0_)-> Tret_
  where
     T0_: Into<*mut MlirOperationState>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationCreate(Into::<*mut MlirOperationState>::into(state_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOperationCreateParse<T0_, T1_, T2_>(context_:  T0_, sourceStr_:  T1_, sourceName_:  T2_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationCreateParse(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(sourceStr_), Into::<MlirStringRef>::into(sourceName_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOperationClone<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationClone(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationDestroy<T0_>(op_:  T0_)
  where
     T0_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirOperationDestroy(Into::<MlirOperation>::into(op_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationRemoveFromParent<T0_>(op_:  T0_)
  where
     T0_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirOperationRemoveFromParent(Into::<MlirOperation>::into(op_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationIsNull<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationIsNull(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationEqual<T0_, T1_>(op_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationEqual(Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(other_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirOperationGetContext<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetContext(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLocation> {
  pub unsafe fn mlirOperationGetLocation<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetLocation(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirOperationGetTypeID<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetTypeID(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIdentifier> {
  pub unsafe fn mlirOperationGetName<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetName(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirOperationGetBlock<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetBlock(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOperationGetParentOperation<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetParentOperation(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumRegions<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumRegions(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRegion> {
  pub unsafe fn mlirOperationGetRegion<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetRegion(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOperationGetNextInBlock<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNextInBlock(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumOperands<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumOperands(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirOperationGetOperand<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetOperand(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetOperand<T0_, T1_, T2_>(op_:  T0_, pos_:  T1_, newValue_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirValue>
  {
    unsafe {
      crate::IR::mlirOperationSetOperand(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirValue>::into(newValue_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetOperands<T0_, T1_, T2_>(op_:  T0_, nOperands_:  T1_, operands_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<*const MlirValue>
  {
    unsafe {
      crate::IR::mlirOperationSetOperands(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(nOperands_), Into::<*const MlirValue>::into(operands_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumResults<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumResults(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirOperationGetResult<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetResult(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumSuccessors<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumSuccessors(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirOperationGetSuccessor<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetSuccessor(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetSuccessor<T0_, T1_, T2_>(op_:  T0_, pos_:  T1_, block_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirOperationSetSuccessor(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationHasInherentAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationHasInherentAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirOperationGetInherentAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetInherentAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetInherentAttributeByName<T0_, T1_, T2_>(op_:  T0_, name_:  T1_, attr_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirAttribute>
  {
    unsafe {
      crate::IR::mlirOperationSetInherentAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_), Into::<MlirAttribute>::into(attr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumDiscardableAttributes<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumDiscardableAttributes(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirNamedAttribute> {
  pub unsafe fn mlirOperationGetDiscardableAttribute<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetDiscardableAttribute(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirOperationGetDiscardableAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetDiscardableAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetDiscardableAttributeByName<T0_, T1_, T2_>(op_:  T0_, name_:  T1_, attr_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirAttribute>
  {
    unsafe {
      crate::IR::mlirOperationSetDiscardableAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_), Into::<MlirAttribute>::into(attr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationRemoveDiscardableAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationRemoveDiscardableAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOperationGetNumAttributes<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetNumAttributes(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirNamedAttribute> {
  pub unsafe fn mlirOperationGetAttribute<T0_, T1_>(op_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetAttribute(Into::<MlirOperation>::into(op_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirOperationGetAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationSetAttributeByName<T0_, T1_, T2_>(op_:  T0_, name_:  T1_, attr_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirAttribute>
  {
    unsafe {
      crate::IR::mlirOperationSetAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_), Into::<MlirAttribute>::into(attr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationRemoveAttributeByName<T0_, T1_>(op_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationRemoveAttributeByName(Into::<MlirOperation>::into(op_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationPrint<T0_, T1_, T2_>(op_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirOperationPrint(Into::<MlirOperation>::into(op_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationPrintWithFlags<T0_, T1_, T2_, T3_>(op_:  T0_, flags_:  T1_, callback_:  T2_, userData_:  T3_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOpPrintingFlags>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirOperationPrintWithFlags(Into::<MlirOperation>::into(op_), Into::<MlirOpPrintingFlags>::into(flags_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationPrintWithState<T0_, T1_, T2_, T3_>(op_:  T0_, state_:  T1_, callback_:  T2_, userData_:  T3_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirAsmState>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirOperationPrintWithState(Into::<MlirOperation>::into(op_), Into::<MlirAsmState>::into(state_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationWriteBytecode<T0_, T1_, T2_>(op_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirOperationWriteBytecode(Into::<MlirOperation>::into(op_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirOperationWriteBytecodeWithConfig<T0_, T1_, T2_, T3_>(op_:  T0_, config_:  T1_, callback_:  T2_, userData_:  T3_)-> Tret_
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirBytecodeWriterConfig>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationWriteBytecodeWithConfig(Into::<MlirOperation>::into(op_), Into::<MlirBytecodeWriterConfig>::into(config_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationDump<T0_>(op_:  T0_)
  where
     T0_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirOperationDump(Into::<MlirOperation>::into(op_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOperationVerify<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationVerify(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationMoveAfter<T0_, T1_>(op_:  T0_, other_:  T1_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirOperationMoveAfter(Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(other_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationMoveBefore<T0_, T1_>(op_:  T0_, other_:  T1_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirOperationMoveBefore(Into::<MlirOperation>::into(op_), Into::<MlirOperation>::into(other_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirOperationWalk<T0_, T1_, T2_, T3_>(op_:  T0_, callback_:  T1_, userData_:  T2_, walkOrder_:  T3_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<MlirOperationWalkCallback>,  T2_: Into<*mut std::ffi::c_void>,  T3_: Into<EnumMlirWalkOrder>
  {
    unsafe {
      crate::IR::mlirOperationWalk(Into::<MlirOperation>::into(op_), Into::<MlirOperationWalkCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_), Into::<EnumMlirWalkOrder>::into(walkOrder_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRegion> {
  pub unsafe fn mlirRegionCreate()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirRegionCreate()
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionDestroy<T0_>(region_:  T0_)
  where
     T0_: Into<MlirRegion>
  {
    unsafe {
      crate::IR::mlirRegionDestroy(Into::<MlirRegion>::into(region_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirRegionIsNull<T0_>(region_:  T0_)-> Tret_
  where
     T0_: Into<MlirRegion>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirRegionIsNull(Into::<MlirRegion>::into(region_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirRegionEqual<T0_, T1_>(region_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirRegion>,  T1_: Into<MlirRegion>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirRegionEqual(Into::<MlirRegion>::into(region_), Into::<MlirRegion>::into(other_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirRegionGetFirstBlock<T0_>(region_:  T0_)-> Tret_
  where
     T0_: Into<MlirRegion>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirRegionGetFirstBlock(Into::<MlirRegion>::into(region_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionAppendOwnedBlock<T0_, T1_>(region_:  T0_, block_:  T1_)
  where
     T0_: Into<MlirRegion>,  T1_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirRegionAppendOwnedBlock(Into::<MlirRegion>::into(region_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionInsertOwnedBlock<T0_, T1_, T2_>(region_:  T0_, pos_:  T1_, block_:  T2_)
  where
     T0_: Into<MlirRegion>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirRegionInsertOwnedBlock(Into::<MlirRegion>::into(region_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionInsertOwnedBlockAfter<T0_, T1_, T2_>(region_:  T0_, reference_:  T1_, block_:  T2_)
  where
     T0_: Into<MlirRegion>,  T1_: Into<MlirBlock>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirRegionInsertOwnedBlockAfter(Into::<MlirRegion>::into(region_), Into::<MlirBlock>::into(reference_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionInsertOwnedBlockBefore<T0_, T1_, T2_>(region_:  T0_, reference_:  T1_, block_:  T2_)
  where
     T0_: Into<MlirRegion>,  T1_: Into<MlirBlock>,  T2_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirRegionInsertOwnedBlockBefore(Into::<MlirRegion>::into(region_), Into::<MlirBlock>::into(reference_), Into::<MlirBlock>::into(block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRegion> {
  pub unsafe fn mlirOperationGetFirstRegion<T0_>(op_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOperationGetFirstRegion(Into::<MlirOperation>::into(op_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRegion> {
  pub unsafe fn mlirRegionGetNextInOperation<T0_>(region_:  T0_)-> Tret_
  where
     T0_: Into<MlirRegion>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirRegionGetNextInOperation(Into::<MlirRegion>::into(region_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirRegionTakeBody<T0_, T1_>(target_:  T0_, source_:  T1_)
  where
     T0_: Into<MlirRegion>,  T1_: Into<MlirRegion>
  {
    unsafe {
      crate::IR::mlirRegionTakeBody(Into::<MlirRegion>::into(target_), Into::<MlirRegion>::into(source_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirBlockCreate<T0_, T1_, T2_>(nArgs_:  T0_, args_:  T1_, locs_:  T2_)-> Tret_
  where
     T0_: Into<std::ffi::c_long>,  T1_: Into<*const MlirType>,  T2_: Into<*const MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockCreate(Into::<std::ffi::c_long>::into(nArgs_), Into::<*const MlirType>::into(args_), Into::<*const MlirLocation>::into(locs_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockDestroy<T0_>(block_:  T0_)
  where
     T0_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirBlockDestroy(Into::<MlirBlock>::into(block_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockDetach<T0_>(block_:  T0_)
  where
     T0_: Into<MlirBlock>
  {
    unsafe {
      crate::IR::mlirBlockDetach(Into::<MlirBlock>::into(block_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirBlockIsNull<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockIsNull(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirBlockEqual<T0_, T1_>(block_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockEqual(Into::<MlirBlock>::into(block_), Into::<MlirBlock>::into(other_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirBlockGetParentOperation<T0_>(arg0_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetParentOperation(Into::<MlirBlock>::into(arg0_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirRegion> {
  pub unsafe fn mlirBlockGetParentRegion<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetParentRegion(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirBlockGetNextInRegion<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetNextInRegion(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirBlockGetFirstOperation<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetFirstOperation(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirBlockGetTerminator<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetTerminator(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockAppendOwnedOperation<T0_, T1_>(block_:  T0_, operation_:  T1_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirBlockAppendOwnedOperation(Into::<MlirBlock>::into(block_), Into::<MlirOperation>::into(operation_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockInsertOwnedOperation<T0_, T1_, T2_>(block_:  T0_, pos_:  T1_, operation_:  T2_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirBlockInsertOwnedOperation(Into::<MlirBlock>::into(block_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirOperation>::into(operation_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockInsertOwnedOperationAfter<T0_, T1_, T2_>(block_:  T0_, reference_:  T1_, operation_:  T2_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirBlockInsertOwnedOperationAfter(Into::<MlirBlock>::into(block_), Into::<MlirOperation>::into(reference_), Into::<MlirOperation>::into(operation_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockInsertOwnedOperationBefore<T0_, T1_, T2_>(block_:  T0_, reference_:  T1_, operation_:  T2_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirOperation>,  T2_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirBlockInsertOwnedOperationBefore(Into::<MlirBlock>::into(block_), Into::<MlirOperation>::into(reference_), Into::<MlirOperation>::into(operation_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirBlockGetNumArguments<T0_>(block_:  T0_)-> Tret_
  where
     T0_: Into<MlirBlock>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetNumArguments(Into::<MlirBlock>::into(block_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirBlockAddArgument<T0_, T1_, T2_>(block_:  T0_, type_:  T1_, loc_:  T2_)-> Tret_
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirType>,  T2_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockAddArgument(Into::<MlirBlock>::into(block_), Into::<MlirType>::into(type_), Into::<MlirLocation>::into(loc_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockEraseArgument<T0_, T1_>(block_:  T0_, index_:  T1_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<std::ffi::c_uint>
  {
    unsafe {
      crate::IR::mlirBlockEraseArgument(Into::<MlirBlock>::into(block_), Into::<std::ffi::c_uint>::into(index_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirBlockInsertArgument<T0_, T1_, T2_, T3_>(block_:  T0_, pos_:  T1_, type_:  T2_, loc_:  T3_)-> Tret_
  where
     T0_: Into<MlirBlock>,  T1_: Into<std::ffi::c_long>,  T2_: Into<MlirType>,  T3_: Into<MlirLocation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockInsertArgument(Into::<MlirBlock>::into(block_), Into::<std::ffi::c_long>::into(pos_), Into::<MlirType>::into(type_), Into::<MlirLocation>::into(loc_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirBlockGetArgument<T0_, T1_>(block_:  T0_, pos_:  T1_)-> Tret_
  where
     T0_: Into<MlirBlock>,  T1_: Into<std::ffi::c_long>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockGetArgument(Into::<MlirBlock>::into(block_), Into::<std::ffi::c_long>::into(pos_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockPrint<T0_, T1_, T2_>(block_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirBlock>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirBlockPrint(Into::<MlirBlock>::into(block_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirValueIsNull<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueIsNull(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirValueEqual<T0_, T1_>(value1_:  T0_, value2_:  T1_)-> Tret_
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueEqual(Into::<MlirValue>::into(value1_), Into::<MlirValue>::into(value2_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirValueIsABlockArgument<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueIsABlockArgument(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirValueIsAOpResult<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueIsAOpResult(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirBlock> {
  pub unsafe fn mlirBlockArgumentGetOwner<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockArgumentGetOwner(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirBlockArgumentGetArgNumber<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirBlockArgumentGetArgNumber(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirBlockArgumentSetType<T0_, T1_>(value_:  T0_, type_:  T1_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirType>
  {
    unsafe {
      crate::IR::mlirBlockArgumentSetType(Into::<MlirValue>::into(value_), Into::<MlirType>::into(type_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOpResultGetOwner<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpResultGetOwner(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_long> {
  pub unsafe fn mlirOpResultGetResultNumber<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpResultGetResultNumber(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirValueGetType<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueGetType(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValueSetType<T0_, T1_>(value_:  T0_, type_:  T1_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirType>
  {
    unsafe {
      crate::IR::mlirValueSetType(Into::<MlirValue>::into(value_), Into::<MlirType>::into(type_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValueDump<T0_>(value_:  T0_)
  where
     T0_: Into<MlirValue>
  {
    unsafe {
      crate::IR::mlirValueDump(Into::<MlirValue>::into(value_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValuePrint<T0_, T1_, T2_>(value_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirValuePrint(Into::<MlirValue>::into(value_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValuePrintAsOperand<T0_, T1_, T2_, T3_>(value_:  T0_, state_:  T1_, callback_:  T2_, userData_:  T3_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirAsmState>,  T2_: Into<MlirStringCallback>,  T3_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirValuePrintAsOperand(Into::<MlirValue>::into(value_), Into::<MlirAsmState>::into(state_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpOperand> {
  pub unsafe fn mlirValueGetFirstUse<T0_>(value_:  T0_)-> Tret_
  where
     T0_: Into<MlirValue>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirValueGetFirstUse(Into::<MlirValue>::into(value_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValueReplaceAllUsesOfWith<T0_, T1_>(of_:  T0_, with_:  T1_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirValue>
  {
    unsafe {
      crate::IR::mlirValueReplaceAllUsesOfWith(Into::<MlirValue>::into(of_), Into::<MlirValue>::into(with_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirValueReplaceAllUsesExcept<T0_, T1_, T2_, T3_>(of_:  T0_, with_:  T1_, numExceptions_:  T2_, exceptions_:  T3_)
  where
     T0_: Into<MlirValue>,  T1_: Into<MlirValue>,  T2_: Into<std::ffi::c_long>,  T3_: Into<*mut MlirOperation>
  {
    unsafe {
      crate::IR::mlirValueReplaceAllUsesExcept(Into::<MlirValue>::into(of_), Into::<MlirValue>::into(with_), Into::<std::ffi::c_long>::into(numExceptions_), Into::<*mut MlirOperation>::into(exceptions_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirOpOperandIsNull<T0_>(opOperand_:  T0_)-> Tret_
  where
     T0_: Into<MlirOpOperand>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpOperandIsNull(Into::<MlirOpOperand>::into(opOperand_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirValue> {
  pub unsafe fn mlirOpOperandGetValue<T0_>(opOperand_:  T0_)-> Tret_
  where
     T0_: Into<MlirOpOperand>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpOperandGetValue(Into::<MlirOpOperand>::into(opOperand_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirOpOperandGetOwner<T0_>(opOperand_:  T0_)-> Tret_
  where
     T0_: Into<MlirOpOperand>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpOperandGetOwner(Into::<MlirOpOperand>::into(opOperand_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<std::ffi::c_uint> {
  pub unsafe fn mlirOpOperandGetOperandNumber<T0_>(opOperand_:  T0_)-> Tret_
  where
     T0_: Into<MlirOpOperand>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpOperandGetOperandNumber(Into::<MlirOpOperand>::into(opOperand_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOpOperand> {
  pub unsafe fn mlirOpOperandGetNextUse<T0_>(opOperand_:  T0_)-> Tret_
  where
     T0_: Into<MlirOpOperand>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirOpOperandGetNextUse(Into::<MlirOpOperand>::into(opOperand_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirTypeParseGet<T0_, T1_>(context_:  T0_, type_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeParseGet(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirTypeGetContext<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeGetContext(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirTypeGetTypeID<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeGetTypeID(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialect> {
  pub unsafe fn mlirTypeGetDialect<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeGetDialect(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeIsNull<T0_>(type_:  T0_)-> Tret_
  where
     T0_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeIsNull(Into::<MlirType>::into(type_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirTypeEqual<T0_, T1_>(t1_:  T0_, t2_:  T1_)-> Tret_
  where
     T0_: Into<MlirType>,  T1_: Into<MlirType>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirTypeEqual(Into::<MlirType>::into(t1_), Into::<MlirType>::into(t2_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTypePrint<T0_, T1_, T2_>(type_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirType>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirTypePrint(Into::<MlirType>::into(type_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirTypeDump<T0_>(type_:  T0_)
  where
     T0_: Into<MlirType>
  {
    unsafe {
      crate::IR::mlirTypeDump(Into::<MlirType>::into(type_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirAttributeParseGet<T0_, T1_>(context_:  T0_, attr_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeParseGet(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirAttributeGetContext<T0_>(attribute_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeGetContext(Into::<MlirAttribute>::into(attribute_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirType> {
  pub unsafe fn mlirAttributeGetType<T0_>(attribute_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeGetType(Into::<MlirAttribute>::into(attribute_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirTypeID> {
  pub unsafe fn mlirAttributeGetTypeID<T0_>(attribute_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeGetTypeID(Into::<MlirAttribute>::into(attribute_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirDialect> {
  pub unsafe fn mlirAttributeGetDialect<T0_>(attribute_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeGetDialect(Into::<MlirAttribute>::into(attribute_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeIsNull<T0_>(attr_:  T0_)-> Tret_
  where
     T0_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeIsNull(Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirAttributeEqual<T0_, T1_>(a1_:  T0_, a2_:  T1_)-> Tret_
  where
     T0_: Into<MlirAttribute>,  T1_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirAttributeEqual(Into::<MlirAttribute>::into(a1_), Into::<MlirAttribute>::into(a2_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAttributePrint<T0_, T1_, T2_>(attr_:  T0_, callback_:  T1_, userData_:  T2_)
  where
     T0_: Into<MlirAttribute>,  T1_: Into<MlirStringCallback>,  T2_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirAttributePrint(Into::<MlirAttribute>::into(attr_), Into::<MlirStringCallback>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirAttributeDump<T0_>(attr_:  T0_)
  where
     T0_: Into<MlirAttribute>
  {
    unsafe {
      crate::IR::mlirAttributeDump(Into::<MlirAttribute>::into(attr_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirNamedAttribute> {
  pub unsafe fn mlirNamedAttributeGet<T0_, T1_>(name_:  T0_, attr_:  T1_)-> Tret_
  where
     T0_: Into<MlirIdentifier>,  T1_: Into<MlirAttribute>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirNamedAttributeGet(Into::<MlirIdentifier>::into(name_), Into::<MlirAttribute>::into(attr_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirIdentifier> {
  pub unsafe fn mlirIdentifierGet<T0_, T1_>(context_:  T0_, str_:  T1_)-> Tret_
  where
     T0_: Into<MlirContext>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirIdentifierGet(Into::<MlirContext>::into(context_), Into::<MlirStringRef>::into(str_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirContext> {
  pub unsafe fn mlirIdentifierGetContext<T0_>(arg0_:  T0_)-> Tret_
  where
     T0_: Into<MlirIdentifier>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirIdentifierGetContext(Into::<MlirIdentifier>::into(arg0_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirIdentifierEqual<T0_, T1_>(ident_:  T0_, other_:  T1_)-> Tret_
  where
     T0_: Into<MlirIdentifier>,  T1_: Into<MlirIdentifier>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirIdentifierEqual(Into::<MlirIdentifier>::into(ident_), Into::<MlirIdentifier>::into(other_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirIdentifierStr<T0_>(ident_:  T0_)-> Tret_
  where
     T0_: Into<MlirIdentifier>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirIdentifierStr(Into::<MlirIdentifier>::into(ident_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirSymbolTableGetSymbolAttributeName()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableGetSymbolAttributeName()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirStringRef> {
  pub unsafe fn mlirSymbolTableGetVisibilityAttributeName()-> Tret_
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableGetVisibilityAttributeName()
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirSymbolTable> {
  pub unsafe fn mlirSymbolTableCreate<T0_>(operation_:  T0_)-> Tret_
  where
     T0_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableCreate(Into::<MlirOperation>::into(operation_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<u8> {
  pub unsafe fn mlirSymbolTableIsNull<T0_>(symbolTable_:  T0_)-> Tret_
  where
     T0_: Into<MlirSymbolTable>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableIsNull(Into::<MlirSymbolTable>::into(symbolTable_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirSymbolTableDestroy<T0_>(symbolTable_:  T0_)
  where
     T0_: Into<MlirSymbolTable>
  {
    unsafe {
      crate::IR::mlirSymbolTableDestroy(Into::<MlirSymbolTable>::into(symbolTable_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirOperation> {
  pub unsafe fn mlirSymbolTableLookup<T0_, T1_>(symbolTable_:  T0_, name_:  T1_)-> Tret_
  where
     T0_: Into<MlirSymbolTable>,  T1_: Into<MlirStringRef>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableLookup(Into::<MlirSymbolTable>::into(symbolTable_), Into::<MlirStringRef>::into(name_))
      }
    )
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirAttribute> {
  pub unsafe fn mlirSymbolTableInsert<T0_, T1_>(symbolTable_:  T0_, operation_:  T1_)-> Tret_
  where
     T0_: Into<MlirSymbolTable>,  T1_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableInsert(Into::<MlirSymbolTable>::into(symbolTable_), Into::<MlirOperation>::into(operation_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirSymbolTableErase<T0_, T1_>(symbolTable_:  T0_, operation_:  T1_)
  where
     T0_: Into<MlirSymbolTable>,  T1_: Into<MlirOperation>
  {
    unsafe {
      crate::IR::mlirSymbolTableErase(Into::<MlirSymbolTable>::into(symbolTable_), Into::<MlirOperation>::into(operation_))
    }
  }
}

impl<Tret_> FFIVal_<Tret_> where Tret_: From<MlirLogicalResult> {
  pub unsafe fn mlirSymbolTableReplaceAllSymbolUses<T0_, T1_, T2_>(oldSymbol_:  T0_, newSymbol_:  T1_, from_:  T2_)-> Tret_
  where
     T0_: Into<MlirStringRef>,  T1_: Into<MlirStringRef>,  T2_: Into<MlirOperation>
  {
    Into::<Tret_>::into(
      unsafe {
        crate::IR::mlirSymbolTableReplaceAllSymbolUses(Into::<MlirStringRef>::into(oldSymbol_), Into::<MlirStringRef>::into(newSymbol_), Into::<MlirOperation>::into(from_))
      }
    )
  }
}

impl FFIVoid_ {
  pub unsafe fn mlirSymbolTableWalkSymbolTables<T0_, T1_, T2_, T3_>(from_:  T0_, allSymUsesVisible_:  T1_, callback_:  T2_, userData_:  T3_)
  where
     T0_: Into<MlirOperation>,  T1_: Into<u8>,  T2_: Into<*mut extern fn (MlirOperation, u8, *mut std::ffi::c_void) -> ()>,  T3_: Into<*mut std::ffi::c_void>
  {
    unsafe {
      crate::IR::mlirSymbolTableWalkSymbolTables(Into::<MlirOperation>::into(from_), Into::<u8>::into(allSymUsesVisible_), Into::<*mut extern fn (MlirOperation, u8, *mut std::ffi::c_void) -> ()>::into(callback_), Into::<*mut std::ffi::c_void>::into(userData_))
    }
  }
}

