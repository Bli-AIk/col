use crate::codegen::ir_generator::{IRGenError, IRGenResult, IRGenerator};
use inkwell::types::BasicTypeEnum;
use inkwell::values::*;

impl<'ctx> IRGenerator<'ctx> {
    /// Generate IR for a constant number value
    pub fn gen_number_const(&self, value: f64) -> FloatValue<'ctx> {
        self.type_mapping.get_number_type().const_float(value)
    }

    /// Generate IR for a constant boolean value
    pub fn gen_bool_const(&self, value: bool) -> IntValue<'ctx> {
        self.type_mapping
            .get_bool_type()
            .const_int(if value { 1 } else { 0 }, false)
    }

    /// Generate IR for a string constant
    pub fn gen_string_const(&self, value: &str) -> PointerValue<'ctx> {
        self.builder
            .build_global_string_ptr(value, "str_const")
            .expect("Failed to build string constant")
            .as_pointer_value()
    }

    /// Generate IR for a null value
    pub fn gen_null_const(&self) -> PointerValue<'ctx> {
        self.type_mapping.get_string_type().const_null()
    }

    /// Declare a variable in the current scope
    pub fn declare_variable(
        &mut self,
        name: &str,
        value_type: BasicTypeEnum<'ctx>,
    ) -> IRGenResult<PointerValue<'ctx>> {
        let alloca = self.builder.build_alloca(value_type, name).map_err(|e| {
            IRGenError::InvalidOperation(format!("Failed to allocate variable '{}': {}", name, e))
        })?;

        self.variables.insert(name.to_string(), alloca);
        self.variable_types.insert(name.to_string(), value_type);
        Ok(alloca)
    }

    /// Get a variable from the current scope
    pub fn get_variable(&self, name: &str) -> IRGenResult<PointerValue<'ctx>> {
        self.variables
            .get(name)
            .copied()
            .ok_or_else(|| IRGenError::UndefinedVariable(name.to_string()))
    }

    /// Load a variable's value
    pub fn load_variable(&self, name: &str) -> IRGenResult<BasicValueEnum<'ctx>> {
        let var_ptr = self.get_variable(name)?;
        // Get the type from our type tracking table
        let var_type = self.variable_types.get(name).ok_or_else(|| {
            IRGenError::InvalidOperation(format!(
                "Type information missing for variable '{}'",
                name
            ))
        })?;

        self.builder
            .build_load(*var_type, var_ptr, name)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to load variable '{}': {}", name, e))
            })
    }

    /// Store a value to a variable
    pub fn store_variable(&mut self, name: &str, value: BasicValueEnum<'ctx>) -> IRGenResult<()> {
        let var_ptr = self.get_variable(name)?;
        self.builder.build_store(var_ptr, value).map_err(|e| {
            IRGenError::InvalidOperation(format!("Failed to store to variable '{}': {}", name, e))
        })?;
        Ok(())
    }

    /// Convert a value to boolean for conditional operations
    pub fn convert_to_bool(&self, value: BasicValueEnum<'ctx>) -> IRGenResult<IntValue<'ctx>> {
        match value {
            BasicValueEnum::IntValue(int_val) => {
                if int_val.get_type() == self.type_mapping.get_bool_type() {
                    Ok(int_val)
                } else {
                    self.builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            int_val,
                            int_val.get_type().const_zero(),
                            "tobool",
                        )
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Failed to convert int to bool: {}",
                                e
                            ))
                        })
                }
            }
            BasicValueEnum::FloatValue(float_val) => self
                .builder
                .build_float_compare(
                    inkwell::FloatPredicate::ONE,
                    float_val,
                    float_val.get_type().const_zero(),
                    "tobool",
                )
                .map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to convert float to bool: {}", e))
                }),
            BasicValueEnum::PointerValue(ptr_val) => {
                let null_ptr = ptr_val.get_type().const_null();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, ptr_val, null_ptr, "tobool")
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!(
                            "Failed to convert pointer to bool: {}",
                            e
                        ))
                    })
            }
            _ => Err(IRGenError::TypeMismatch(
                "Cannot convert value to boolean".to_string(),
            )),
        }
    }

    /// Determine the type of a BasicValueEnum
    pub fn get_value_type(&self, value: BasicValueEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        match value {
            BasicValueEnum::IntValue(v) => v.get_type().into(),
            BasicValueEnum::FloatValue(v) => v.get_type().into(),
            BasicValueEnum::PointerValue(v) => v.get_type().into(),
            BasicValueEnum::ArrayValue(v) => v.get_type().into(),
            BasicValueEnum::StructValue(v) => v.get_type().into(),
            BasicValueEnum::VectorValue(v) => v.get_type().into(),
            _ => panic!("Unsupported value type"),
        }
    }

    /// Get module reference
    pub fn get_module(&self) -> &inkwell::module::Module<'ctx> {
        &self.module
    }

    /// Convert a value to match the expected function return type
    pub fn convert_to_return_type(
        &self,
        value: BasicValueEnum<'ctx>,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        // For now, all functions return double, so convert booleans to double
        match value {
            BasicValueEnum::IntValue(int_val)
                if int_val.get_type() == self.type_mapping.get_bool_type() =>
            {
                // Convert boolean to double: false -> 0.0, true -> 1.0
                // Use select instruction to ensure correct conversion
                let true_val = self.type_mapping.get_number_type().const_float(1.0);
                let false_val = self.type_mapping.get_number_type().const_float(0.0);
                let double_val = self
                    .builder
                    .build_select(int_val, true_val, false_val, "bool_to_double")
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!(
                            "Bool to double conversion failed: {}",
                            e
                        ))
                    })?;
                Ok(double_val.into())
            }
            _ => Ok(value), // Other types remain unchanged
        }
    }
}
