// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Defines accessors for compiled modules.

use crate::{errors::PartialVMResult, file_format::*, internals::ModuleIndex};
use move_core_types::{
    account_address::AccountAddress,
    identifier::{IdentStr, Identifier},
    language_storage::ModuleId,
};

/// Represents accessors for a compiled module.
///
/// This is a trait to allow working across different wrappers for `CompiledModule`.
pub trait ModuleAccess: Sync {
    /// Returns the `CompiledModule` that will be used for accesses.
    fn as_module(&self) -> &CompiledModule;

    fn self_handle_idx(&self) -> ModuleHandleIndex {
        self.as_module().self_module_handle_idx
    }

    /// Returns the `ModuleHandle` for `self`.
    fn self_handle(&self) -> &ModuleHandle {
        let handle = self.module_handle_at(self.self_handle_idx());
        debug_assert!(handle.address.into_index() < self.as_module().address_identifiers.len()); // invariant
        debug_assert!(handle.name.into_index() < self.as_module().identifiers.len()); // invariant
        handle
    }

    /// Returns the name of the module.
    fn name(&self) -> &IdentStr {
        self.identifier_at(self.self_handle().name)
    }

    /// Returns the address of the module.
    fn address(&self) -> &AccountAddress {
        self.address_identifier_at(self.self_handle().address)
    }

    fn struct_name(&self, idx: StructDefinitionIndex) -> &IdentStr {
        let struct_def = self.struct_def_at(idx);
        let handle = self.struct_handle_at(struct_def.struct_handle);
        self.identifier_at(handle.name)
    }

    fn module_handle_at(&self, idx: ModuleHandleIndex) -> &ModuleHandle {
        let handle = &self.as_module().module_handles[idx.into_index()];
        debug_assert!(handle.address.into_index() < self.as_module().address_identifiers.len()); // invariant
        debug_assert!(handle.name.into_index() < self.as_module().identifiers.len()); // invariant
        handle
    }

    fn struct_handle_at(&self, idx: StructHandleIndex) -> &StructHandle {
        let handle = &self.as_module().struct_handles[idx.into_index()];
        debug_assert!(handle.module.into_index() < self.as_module().module_handles.len()); // invariant
        handle
    }

    fn function_handle_at(&self, idx: FunctionHandleIndex) -> &FunctionHandle {
        let handle = &self.as_module().function_handles[idx.into_index()];
        debug_assert!(handle.parameters.into_index() < self.as_module().signatures.len()); // invariant
        debug_assert!(handle.return_.into_index() < self.as_module().signatures.len()); // invariant
        handle
    }

    fn field_handle_at(&self, idx: FieldHandleIndex) -> &FieldHandle {
        let handle = &self.as_module().field_handles[idx.into_index()];
        debug_assert!(handle.owner.into_index() < self.as_module().struct_defs.len()); // invariant
        handle
    }

    fn struct_instantiation_at(&self, idx: StructDefInstantiationIndex) -> &StructDefInstantiation {
        &self.as_module().struct_def_instantiations[idx.into_index()]
    }

    fn function_instantiation_at(&self, idx: FunctionInstantiationIndex) -> &FunctionInstantiation {
        &self.as_module().function_instantiations[idx.into_index()]
    }

    fn field_instantiation_at(&self, idx: FieldInstantiationIndex) -> &FieldInstantiation {
        &self.as_module().field_instantiations[idx.into_index()]
    }

    fn signature_at(&self, idx: SignatureIndex) -> &Signature {
        &self.as_module().signatures[idx.into_index()]
    }

    fn identifier_at(&self, idx: IdentifierIndex) -> &IdentStr {
        &self.as_module().identifiers[idx.into_index()]
    }

    fn address_identifier_at(&self, idx: AddressIdentifierIndex) -> &AccountAddress {
        &self.as_module().address_identifiers[idx.into_index()]
    }

    fn constant_at(&self, idx: ConstantPoolIndex) -> &Constant {
        &self.as_module().constant_pool[idx.into_index()]
    }

    fn struct_def_at(&self, idx: StructDefinitionIndex) -> &StructDefinition {
        &self.as_module().struct_defs[idx.into_index()]
    }

    fn function_def_at(&self, idx: FunctionDefinitionIndex) -> &FunctionDefinition {
        let result = &self.as_module().function_defs[idx.into_index()];
        debug_assert!(result.function.into_index() < self.function_handles().len()); // invariant
        debug_assert!(match &result.code {
            Some(code) => code.locals.into_index() < self.signatures().len(),
            None => true,
        }); // invariant
        result
    }

    fn module_handles(&self) -> &[ModuleHandle] {
        &self.as_module().module_handles
    }

    fn struct_handles(&self) -> &[StructHandle] {
        &self.as_module().struct_handles
    }

    fn function_handles(&self) -> &[FunctionHandle] {
        &self.as_module().function_handles
    }

    fn field_handles(&self) -> &[FieldHandle] {
        &self.as_module().field_handles
    }

    fn struct_instantiations(&self) -> &[StructDefInstantiation] {
        &self.as_module().struct_def_instantiations
    }

    fn function_instantiations(&self) -> &[FunctionInstantiation] {
        &self.as_module().function_instantiations
    }

    fn field_instantiations(&self) -> &[FieldInstantiation] {
        &self.as_module().field_instantiations
    }

    fn signatures(&self) -> &[Signature] {
        &self.as_module().signatures
    }

    fn constant_pool(&self) -> &[Constant] {
        &self.as_module().constant_pool
    }

    fn identifiers(&self) -> &[Identifier] {
        &self.as_module().identifiers
    }

    fn address_identifiers(&self) -> &[AccountAddress] {
        &self.as_module().address_identifiers
    }

    fn struct_defs(&self) -> &[StructDefinition] {
        &self.as_module().struct_defs
    }

    fn function_defs(&self) -> &[FunctionDefinition] {
        &self.as_module().function_defs
    }

    fn friend_decls(&self) -> &[ModuleHandle] {
        &self.as_module().friend_decls
    }

    fn module_id_for_handle(&self, module_handle_idx: &ModuleHandle) -> ModuleId {
        self.as_module().module_id_for_handle(module_handle_idx)
    }

    fn self_id(&self) -> ModuleId {
        self.as_module().self_id()
    }

    fn version(&self) -> u32 {
        self.as_module().version
    }

    fn immediate_dependencies(&self) -> Vec<ModuleId> {
        let self_handle = self.self_handle();
        self.module_handles()
            .iter()
            .filter(|&handle| handle != self_handle)
            .map(|handle| self.module_id_for_handle(handle))
            .collect()
    }

    fn immediate_friends(&self) -> Vec<ModuleId> {
        self.friend_decls()
            .iter()
            .map(|handle| self.module_id_for_handle(handle))
            .collect()
    }

    fn find_struct_def(&self, idx: StructHandleIndex) -> Option<&StructDefinition> {
        self.struct_defs().iter().find(|d| d.struct_handle == idx)
    }

    fn find_struct_def_by_name(&self, name: &IdentStr) -> Option<&StructDefinition> {
        self.struct_defs().iter().find(|def| {
            let handle = self.struct_handle_at(def.struct_handle);
            name == self.identifier_at(handle.name)
        })
    }

    // Return the `AbilitySet` of a `SignatureToken` given a context.
    // A `TypeParameter` has the abilities of its `constraints`.
    // `StructInstantiation` abilities are predicated on the particular instantiation
    fn abilities(
        &self,
        ty: &SignatureToken,
        constraints: &[AbilitySet],
    ) -> PartialVMResult<AbilitySet> {
        use SignatureToken::*;

        match ty {
            Bool | U8 | U16 | U32 | U64 | U128 | U256 | Address => Ok(AbilitySet::PRIMITIVES),

            Reference(_) | MutableReference(_) => Ok(AbilitySet::REFERENCES),
            Signer => Ok(AbilitySet::SIGNER),
            TypeParameter(idx) => Ok(constraints[*idx as usize]),
            Vector(ty) => AbilitySet::polymorphic_abilities(
                AbilitySet::VECTOR,
                vec![false],
                vec![self.abilities(ty, constraints)?],
            ),
            Struct(idx) => {
                let sh = self.struct_handle_at(*idx);
                Ok(sh.abilities)
            }
            StructInstantiation(struct_inst) => {
                let (idx, type_args) = &**struct_inst;
                let sh = self.struct_handle_at(*idx);
                let declared_abilities = sh.abilities;
                let type_arguments = type_args
                    .iter()
                    .map(|arg| self.abilities(arg, constraints))
                    .collect::<PartialVMResult<Vec<_>>>()?;
                AbilitySet::polymorphic_abilities(
                    declared_abilities,
                    sh.type_parameters.iter().map(|param| param.is_phantom),
                    type_arguments,
                )
            }
        }
    }
}

impl ModuleAccess for CompiledModule {
    fn as_module(&self) -> &CompiledModule {
        self
    }
}
