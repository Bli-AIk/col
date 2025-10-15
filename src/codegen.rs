use inkwell::context::Context;
use inkwell::types::*;
use std::collections::HashMap;

#[cfg(test)]
mod comprehensive_test;
pub mod ir_generator;
pub mod jit;
#[cfg(test)]
mod test;

/// Type mapping table for converting language types to LLVM types
pub struct TypeMapping<'ctx> {
    context: &'ctx Context,
    type_cache: HashMap<String, BasicTypeEnum<'ctx>>,
}

impl<'ctx> TypeMapping<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let mut mapping = Self {
            context,
            type_cache: HashMap::new(),
        };
        mapping.initialize_builtin_types();
        mapping
    }

    /// Initialize built-in types for the language
    fn initialize_builtin_types(&mut self) {
        // Number type (f64 for compatibility with GameMaker's real type)
        self.type_cache
            .insert("number".to_string(), self.context.f64_type().into());

        // Boolean type (i1)
        self.type_cache
            .insert("bool".to_string(), self.context.bool_type().into());

        // Integer type (i32)
        self.type_cache
            .insert("int".to_string(), self.context.i32_type().into());

        // String type (i8* - pointer to char)
        self.type_cache.insert(
            "string".to_string(),
            self.context
                .ptr_type(inkwell::AddressSpace::default())
                .into(),
        );

        // Void type for functions
        // Note: void is stored separately as it's not a BasicType
    }

    /// Get the LLVM type for a number/real value
    pub fn get_number_type(&self) -> FloatType<'ctx> {
        self.context.f64_type()
    }

    /// Get the LLVM type for an integer value
    pub fn get_int_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    /// Get the LLVM type for a boolean value
    pub fn get_bool_type(&self) -> IntType<'ctx> {
        self.context.bool_type()
    }

    /// Get the LLVM type for a string value
    pub fn get_string_type(&self) -> PointerType<'ctx> {
        self.context.ptr_type(inkwell::AddressSpace::default())
    }

    /// Get the LLVM void type for functions
    pub fn get_void_type(&self) -> VoidType<'ctx> {
        self.context.void_type()
    }

    /// Get a function type with specified parameter and return types
    pub fn get_function_type(
        &self,
        return_type: Option<BasicTypeEnum<'ctx>>,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
    ) -> FunctionType<'ctx> {
        match return_type {
            Some(ret_type) => ret_type.fn_type(param_types, false),
            None => self.get_void_type().fn_type(param_types, false),
        }
    }

    /// Get basic type by name
    pub fn get_type(&self, type_name: &str) -> Option<BasicTypeEnum<'ctx>> {
        self.type_cache.get(type_name).copied()
    }

    /// Register a custom type
    pub fn register_type(&mut self, name: String, llvm_type: BasicTypeEnum<'ctx>) {
        self.type_cache.insert(name, llvm_type);
    }

    /// Convert basic type to metadata type for function parameters
    pub fn parse_to_metadata_type(basic_type: BasicTypeEnum<'ctx>) -> BasicMetadataTypeEnum<'ctx> {
        match basic_type {
            BasicTypeEnum::IntType(t) => t.into(),
            BasicTypeEnum::FloatType(t) => t.into(),
            BasicTypeEnum::PointerType(t) => t.into(),
            BasicTypeEnum::ArrayType(t) => t.into(),
            BasicTypeEnum::StructType(t) => t.into(),
            BasicTypeEnum::VectorType(t) => t.into(),
            BasicTypeEnum::ScalableVectorType(t) => t.into(),
        }
    }
}
