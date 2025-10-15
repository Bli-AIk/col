use super::ir_generator::{IRGenError, IRGenResult, IRGenerator};
use crate::parser::expr::Expr;
use inkwell::values::*;

/// Binary operation types
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison operations
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical operations
    And,
    Or,
    Xor,
    // Bitwise operations
    BitAnd,
    BitOr,
    BitXor,
}

impl<'ctx> IRGenerator<'ctx> {
    pub fn visit_expr_impl(&mut self, expr: &Expr) -> IRGenResult<BasicValueEnum<'ctx>> {
        match expr {
            Expr::Number(n) => Ok(self.gen_number_const(*n).into()),
            Expr::String(s) => Ok(self.gen_string_const(s).into()),
            Expr::True(_) => Ok(self.gen_bool_const(true).into()),
            Expr::False(_) => Ok(self.gen_bool_const(false).into()),
            Expr::Null => Ok(self.gen_null_const().into()),

            Expr::Identifier(name) => self.load_variable(name),

            Expr::Call(name, args) => {
                let function = self.get_function(name)?;
                let arg_values: Result<Vec<_>, _> =
                    args.iter().map(|arg| self.visit_expr_impl(arg)).collect();
                let arg_values = arg_values?;

                // Convert BasicValueEnum to BasicMetadataValueEnum
                let metadata_args: Vec<BasicMetadataValueEnum> = arg_values
                    .iter()
                    .map(|val| match val {
                        BasicValueEnum::IntValue(v) => (*v).into(),
                        BasicValueEnum::FloatValue(v) => (*v).into(),
                        BasicValueEnum::PointerValue(v) => (*v).into(),
                        BasicValueEnum::ArrayValue(v) => (*v).into(),
                        BasicValueEnum::StructValue(v) => (*v).into(),
                        BasicValueEnum::VectorValue(v) => (*v).into(),
                        _ => panic!("Unsupported value type for function call"),
                    })
                    .collect();

                let call_value = self
                    .builder
                    .build_call(function, &metadata_args, "call")
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build call: {}", e))
                    })?;

                call_value.try_as_basic_value().left().ok_or_else(|| {
                    IRGenError::InvalidOperation("Function call returned void".to_string())
                })
            }

            // Binary operations
            Expr::Addition(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Add, l, r)
            }
            Expr::Subtraction(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Sub, l, r)
            }
            Expr::Multiplication(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Mul, l, r)
            }
            Expr::Division(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Div, l, r)
            }
            Expr::Percent(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Mod, l, r)
            }

            // Comparison operations
            Expr::EqualEqual(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Eq, l, r)
            }
            Expr::NotEqual(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Ne, l, r)
            }
            Expr::Less(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Lt, l, r)
            }
            Expr::LessEqual(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Le, l, r)
            }
            Expr::Greater(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Gt, l, r)
            }
            Expr::GreaterEqual(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Ge, l, r)
            }

            // Logical operations (short-circuit evaluation)
            Expr::And(lhs, rhs) => self.generate_logical_and(lhs, rhs),
            Expr::Or(lhs, rhs) => self.generate_logical_or(lhs, rhs),

            // Assignment operations
            Expr::Equal(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let value = self.visit_expr_impl(rhs)?;
                    self.store_variable(name, value)?;
                    Ok(value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }
            Expr::PlusEqual(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let rhs_value = self.visit_expr_impl(rhs)?;
                    let new_value = self.gen_binary_op(BinaryOp::Add, current_value, rhs_value)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }
            Expr::MinusEqual(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let rhs_value = self.visit_expr_impl(rhs)?;
                    let new_value = self.gen_binary_op(BinaryOp::Sub, current_value, rhs_value)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }
            Expr::StarEqual(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let rhs_value = self.visit_expr_impl(rhs)?;
                    let new_value = self.gen_binary_op(BinaryOp::Mul, current_value, rhs_value)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }
            Expr::SlashEqual(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let rhs_value = self.visit_expr_impl(rhs)?;
                    let new_value = self.gen_binary_op(BinaryOp::Div, current_value, rhs_value)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }

            // Unary operations
            Expr::Not(expr) => {
                let value = self.visit_expr_impl(expr)?;
                let bool_value = self.convert_to_bool(value)?;
                let result = self.builder.build_not(bool_value, "not").map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to build not: {}", e))
                })?;
                Ok(result.into())
            }
            Expr::Negative(expr) => {
                let value = self.visit_expr_impl(expr)?;
                match value {
                    BasicValueEnum::FloatValue(float_val) => {
                        let result =
                            self.builder
                                .build_float_neg(float_val, "fneg")
                                .map_err(|e| {
                                    IRGenError::InvalidOperation(format!(
                                        "Failed to negate float: {}",
                                        e
                                    ))
                                })?;
                        Ok(result.into())
                    }
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_int_neg(int_val, "ineg").map_err(|e| {
                            IRGenError::InvalidOperation(format!("Failed to negate int: {}", e))
                        })?;
                        Ok(result.into())
                    }
                    _ => Err(IRGenError::TypeMismatch(
                        "Cannot negate non-numeric value".to_string(),
                    )),
                }
            }
            Expr::Positive(expr) => {
                // Positive is basically a no-op, just return the expression value
                self.visit_expr_impl(expr)
            }

            // Increment/Decrement operations
            Expr::PreIncrement(expr) => {
                if let Expr::Identifier(name) = expr.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let one = self.gen_number_const(1.0).into();
                    let new_value = self.gen_binary_op(BinaryOp::Add, current_value, one)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value) // Return new value for pre-increment
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Pre-increment only works on variables".to_string(),
                    ))
                }
            }
            Expr::PostIncrement(expr) => {
                if let Expr::Identifier(name) = expr.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let one = self.gen_number_const(1.0).into();
                    let new_value = self.gen_binary_op(BinaryOp::Add, current_value, one)?;
                    self.store_variable(name, new_value)?;
                    Ok(current_value) // Return old value for post-increment
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Post-increment only works on variables".to_string(),
                    ))
                }
            }
            Expr::PreDecrement(expr) => {
                if let Expr::Identifier(name) = expr.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let one = self.gen_number_const(1.0).into();
                    let new_value = self.gen_binary_op(BinaryOp::Sub, current_value, one)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value) // Return new value for pre-decrement
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Pre-decrement only works on variables".to_string(),
                    ))
                }
            }
            Expr::PostDecrement(expr) => {
                if let Expr::Identifier(name) = expr.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let one = self.gen_number_const(1.0).into();
                    let new_value = self.gen_binary_op(BinaryOp::Sub, current_value, one)?;
                    self.store_variable(name, new_value)?;
                    Ok(current_value) // Return old value for post-decrement
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Post-decrement only works on variables".to_string(),
                    ))
                }
            }

            // Ternary operator
            Expr::Ternary(cond, then_expr, else_expr) => {
                self.generate_ternary(cond, then_expr, else_expr)
            }

            // Parentheses are just pass-through
            Expr::Paren(expr) => self.visit_expr_impl(expr),

            // Additional expressions not yet handled
            Expr::BitNot(expr) => {
                let value = self.visit_expr_impl(expr)?;
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_not(int_val, "bitnot").map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Failed to build bitwise not: {}",
                                e
                            ))
                        })?;
                        Ok(result.into())
                    }
                    BasicValueEnum::FloatValue(float_val) => {
                        // Convert float to int, apply bitwise not, then convert back
                        let int_val = self
                            .builder
                            .build_float_to_signed_int(
                                float_val,
                                self.type_mapping.get_int_type(),
                                "f2i_bitnot",
                            )
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Float to int conversion failed: {}",
                                    e
                                ))
                            })?;
                        let not_result =
                            self.builder.build_not(int_val, "bitnot").map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Failed to build bitwise not: {}",
                                    e
                                ))
                            })?;
                        let float_result = self
                            .builder
                            .build_signed_int_to_float(
                                not_result,
                                self.type_mapping.get_number_type(),
                                "i2f_bitnot",
                            )
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Int to float conversion failed: {}",
                                    e
                                ))
                            })?;
                        Ok(float_result.into())
                    }
                    _ => Err(IRGenError::TypeMismatch(
                        "Bitwise not only works on integers and numbers".to_string(),
                    )),
                }
            }

            Expr::BitAnd(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::BitAnd, l, r)
            }
            Expr::BitOr(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::BitOr, l, r)
            }
            Expr::BitXor(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::BitXor, l, r)
            }
            Expr::Xor(lhs, rhs) => {
                let l = self.visit_expr_impl(lhs)?;
                let r = self.visit_expr_impl(rhs)?;
                self.gen_binary_op(BinaryOp::Xor, l, r)
            }

            Expr::PercentEqual(lhs, rhs) => {
                if let Expr::Identifier(name) = lhs.as_ref() {
                    let current_value = self.load_variable(name)?;
                    let rhs_value = self.visit_expr_impl(rhs)?;
                    let new_value = self.gen_binary_op(BinaryOp::Mod, current_value, rhs_value)?;
                    self.store_variable(name, new_value)?;
                    Ok(new_value)
                } else {
                    Err(IRGenError::InvalidOperation(
                        "Assignment target must be a variable".to_string(),
                    ))
                }
            }
        }
    }

    /// Generate IR for binary operations
    pub fn gen_binary_op(
        &self,
        op: BinaryOp,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = match op {
                    BinaryOp::Add => self.builder.build_float_add(l, r, "fadd").map(|v| v.into()),
                    BinaryOp::Sub => self.builder.build_float_sub(l, r, "fsub").map(|v| v.into()),
                    BinaryOp::Mul => self.builder.build_float_mul(l, r, "fmul").map(|v| v.into()),
                    BinaryOp::Div => self.builder.build_float_div(l, r, "fdiv").map(|v| v.into()),
                    BinaryOp::Mod => self.builder.build_float_rem(l, r, "frem").map(|v| v.into()),
                    BinaryOp::Eq => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::OEQ, l, r, "feq")
                        .map(|v| v.into()),
                    BinaryOp::Ne => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::ONE, l, r, "fne")
                        .map(|v| v.into()),
                    BinaryOp::Lt => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::OLT, l, r, "flt")
                        .map(|v| v.into()),
                    BinaryOp::Le => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::OLE, l, r, "fle")
                        .map(|v| v.into()),
                    BinaryOp::Gt => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::OGT, l, r, "fgt")
                        .map(|v| v.into()),
                    BinaryOp::Ge => self
                        .builder
                        .build_float_compare(inkwell::FloatPredicate::OGE, l, r, "fge")
                        .map(|v| v.into()),
                    // For bitwise operations on floats, convert to int, operate, then convert back
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        let l_int = self
                            .builder
                            .build_float_to_signed_int(l, self.type_mapping.get_int_type(), "f2i_l")
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Float to int conversion failed: {}",
                                    e
                                ))
                            })?;
                        let r_int = self
                            .builder
                            .build_float_to_signed_int(r, self.type_mapping.get_int_type(), "f2i_r")
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Float to int conversion failed: {}",
                                    e
                                ))
                            })?;

                        let int_result = match op {
                            BinaryOp::BitAnd => self.builder.build_and(l_int, r_int, "ibitand"),
                            BinaryOp::BitOr => self.builder.build_or(l_int, r_int, "ibitor"),
                            BinaryOp::BitXor => self.builder.build_xor(l_int, r_int, "ibitxor"),
                            _ => unreachable!(),
                        }
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!("Bitwise operation failed: {}", e))
                        })?;

                        let float_result = self
                            .builder
                            .build_signed_int_to_float(
                                int_result,
                                self.type_mapping.get_number_type(),
                                "i2f",
                            )
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Int to float conversion failed: {}",
                                    e
                                ))
                            })?;

                        Ok(float_result.into())
                    }
                    _ => {
                        return Err(IRGenError::InvalidOperation(format!(
                            "Unsupported float operation: {:?}",
                            op
                        )));
                    }
                };
                result.map_err(|e| {
                    IRGenError::InvalidOperation(format!("Float operation failed: {}", e))
                })
            }
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                // Check if these are booleans that need to be converted to floats for arithmetic
                let l_is_bool = l.get_type() == self.type_mapping.get_bool_type();
                let r_is_bool = r.get_type() == self.type_mapping.get_bool_type();

                if (l_is_bool || r_is_bool)
                    && matches!(
                        op,
                        BinaryOp::Add
                            | BinaryOp::Sub
                            | BinaryOp::Mul
                            | BinaryOp::Div
                            | BinaryOp::Mod
                    )
                {
                    // Convert booleans to floats for arithmetic operations
                    let l_float = if l_is_bool {
                        let true_val = self.type_mapping.get_number_type().const_float(1.0);
                        let false_val = self.type_mapping.get_number_type().const_float(0.0);
                        self.builder
                            .build_select(l, true_val, false_val, "bool_to_float")
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Bool to float conversion failed: {}",
                                    e
                                ))
                            })?
                    } else {
                        self.builder
                            .build_signed_int_to_float(
                                l,
                                self.type_mapping.get_number_type(),
                                "int_to_float",
                            )
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Int to float conversion failed: {}",
                                    e
                                ))
                            })?
                            .into()
                    };

                    let r_float = if r_is_bool {
                        let true_val = self.type_mapping.get_number_type().const_float(1.0);
                        let false_val = self.type_mapping.get_number_type().const_float(0.0);
                        self.builder
                            .build_select(r, true_val, false_val, "bool_to_float")
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Bool to float conversion failed: {}",
                                    e
                                ))
                            })?
                    } else {
                        self.builder
                            .build_signed_int_to_float(
                                r,
                                self.type_mapping.get_number_type(),
                                "int_to_float",
                            )
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!(
                                    "Int to float conversion failed: {}",
                                    e
                                ))
                            })?
                            .into()
                    };

                    return self.gen_binary_op(op, l_float, r_float);
                }

                let result = match op {
                    BinaryOp::Add => self.builder.build_int_add(l, r, "iadd").map(|v| v.into()),
                    BinaryOp::Sub => self.builder.build_int_sub(l, r, "isub").map(|v| v.into()),
                    BinaryOp::Mul => self.builder.build_int_mul(l, r, "imul").map(|v| v.into()),
                    BinaryOp::Div => self
                        .builder
                        .build_int_signed_div(l, r, "idiv")
                        .map(|v| v.into()),
                    BinaryOp::Mod => self
                        .builder
                        .build_int_signed_rem(l, r, "irem")
                        .map(|v| v.into()),
                    BinaryOp::Eq => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::EQ, l, r, "ieq")
                        .map(|v| v.into()),
                    BinaryOp::Ne => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::NE, l, r, "ine")
                        .map(|v| v.into()),
                    BinaryOp::Lt => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::SLT, l, r, "ilt")
                        .map(|v| v.into()),
                    BinaryOp::Le => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::SLE, l, r, "ile")
                        .map(|v| v.into()),
                    BinaryOp::Gt => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::SGT, l, r, "igt")
                        .map(|v| v.into()),
                    BinaryOp::Ge => self
                        .builder
                        .build_int_compare(inkwell::IntPredicate::SGE, l, r, "ige")
                        .map(|v| v.into()),
                    BinaryOp::And => self.builder.build_and(l, r, "iand").map(|v| v.into()),
                    BinaryOp::Or => self.builder.build_or(l, r, "ior").map(|v| v.into()),
                    BinaryOp::Xor => self.builder.build_xor(l, r, "ixor").map(|v| v.into()),
                    BinaryOp::BitAnd => self.builder.build_and(l, r, "ibitand").map(|v| v.into()),
                    BinaryOp::BitOr => self.builder.build_or(l, r, "ibitor").map(|v| v.into()),
                    BinaryOp::BitXor => self.builder.build_xor(l, r, "ibitxor").map(|v| v.into()),
                };
                result.map_err(|e| {
                    IRGenError::InvalidOperation(format!("Int operation failed: {}", e))
                })
            }
            // Handle mixed int/float operations by promoting int to float
            (BasicValueEnum::IntValue(l), BasicValueEnum::FloatValue(r)) => {
                // Check if left operand is boolean and convert accordingly
                let l_float = if l.get_type() == self.type_mapping.get_bool_type() {
                    let true_val = self.type_mapping.get_number_type().const_float(1.0);
                    let false_val = self.type_mapping.get_number_type().const_float(0.0);
                    self.builder
                        .build_select(l, true_val, false_val, "bool_to_float")
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Bool to float conversion failed: {}",
                                e
                            ))
                        })?
                } else {
                    self.builder
                        .build_signed_int_to_float(
                            l,
                            self.type_mapping.get_number_type(),
                            "int_to_float",
                        )
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Int to float conversion failed: {}",
                                e
                            ))
                        })?
                        .into()
                };
                self.gen_binary_op(op, l_float, r.into())
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::IntValue(r)) => {
                // Check if right operand is boolean and convert accordingly
                let r_float = if r.get_type() == self.type_mapping.get_bool_type() {
                    let true_val = self.type_mapping.get_number_type().const_float(1.0);
                    let false_val = self.type_mapping.get_number_type().const_float(0.0);
                    self.builder
                        .build_select(r, true_val, false_val, "bool_to_float")
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Bool to float conversion failed: {}",
                                e
                            ))
                        })?
                } else {
                    self.builder
                        .build_signed_int_to_float(
                            r,
                            self.type_mapping.get_number_type(),
                            "int_to_float",
                        )
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Int to float conversion failed: {}",
                                e
                            ))
                        })?
                        .into()
                };
                self.gen_binary_op(op, l.into(), r_float)
            }
            _ => Err(IRGenError::TypeMismatch(
                "Incompatible types for binary operation".to_string(),
            )),
        }
    }

    fn generate_logical_and(
        &mut self,
        lhs: &Expr,
        rhs: &Expr,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("Logical AND outside function".to_string())
        })?;

        let lhs_value = self.visit_expr_impl(lhs)?;
        let lhs_bool = self.convert_to_bool(lhs_value)?;

        let rhs_block = self.context.append_basic_block(current_fn, "and_rhs");
        let merge_block = self.context.append_basic_block(current_fn, "and_merge");

        // If lhs is false, short-circuit to merge with false
        self.builder
            .build_conditional_branch(lhs_bool, rhs_block, merge_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;
        let lhs_block = self.builder.get_insert_block().unwrap();

        // Evaluate rhs if lhs was true
        self.builder.position_at_end(rhs_block);
        let rhs_value = self.visit_expr_impl(rhs)?;
        let rhs_bool = self.convert_to_bool(rhs_value)?;

        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
        let rhs_end_block = self.builder.get_insert_block().unwrap();

        // Merge block with PHI
        self.builder.position_at_end(merge_block);
        let phi = self
            .builder
            .build_phi(self.type_mapping.get_bool_type(), "and_result")
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build phi: {}", e)))?;

        let false_val = self.type_mapping.get_bool_type().const_zero();
        phi.add_incoming(&[(&false_val, lhs_block), (&rhs_bool, rhs_end_block)]);

        Ok(phi.as_basic_value())
    }

    fn generate_logical_or(&mut self, lhs: &Expr, rhs: &Expr) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("Logical OR outside function".to_string())
        })?;

        let lhs_value = self.visit_expr_impl(lhs)?;
        let lhs_bool = self.convert_to_bool(lhs_value)?;

        let rhs_block = self.context.append_basic_block(current_fn, "or_rhs");
        let merge_block = self.context.append_basic_block(current_fn, "or_merge");

        // If lhs is true, short-circuit to merge with true
        self.builder
            .build_conditional_branch(lhs_bool, merge_block, rhs_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;
        let lhs_block = self.builder.get_insert_block().unwrap();

        // Evaluate rhs if lhs was false
        self.builder.position_at_end(rhs_block);
        let rhs_value = self.visit_expr_impl(rhs)?;
        let rhs_bool = self.convert_to_bool(rhs_value)?;

        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
        let rhs_end_block = self.builder.get_insert_block().unwrap();

        // Merge block with PHI
        self.builder.position_at_end(merge_block);
        let phi = self
            .builder
            .build_phi(self.type_mapping.get_bool_type(), "or_result")
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build phi: {}", e)))?;

        let true_val = self.type_mapping.get_bool_type().const_int(1, false);
        phi.add_incoming(&[(&true_val, lhs_block), (&rhs_bool, rhs_end_block)]);

        Ok(phi.as_basic_value())
    }

    fn generate_ternary(
        &mut self,
        cond: &Expr,
        then_expr: &Expr,
        else_expr: &Expr,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let cond_value = self.visit_expr_impl(cond)?;

        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("Ternary operator outside function".to_string())
        })?;

        let then_block = self.context.append_basic_block(current_fn, "ternary_then");
        let else_block = self.context.append_basic_block(current_fn, "ternary_else");
        let merge_block = self.context.append_basic_block(current_fn, "ternary_merge");

        // Convert condition to i1 if needed
        let cond_i1 = self.convert_to_bool(cond_value)?;

        self.builder
            .build_conditional_branch(cond_i1, then_block, else_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;

        // Generate then block
        self.builder.position_at_end(then_block);
        let then_value = self.visit_expr_impl(then_expr)?;
        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
        let then_end_block = self.builder.get_insert_block().unwrap();

        // Generate else block
        self.builder.position_at_end(else_block);
        let else_value = self.visit_expr_impl(else_expr)?;
        self.builder
            .build_unconditional_branch(merge_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
        let else_end_block = self.builder.get_insert_block().unwrap();

        // Merge block
        self.builder.position_at_end(merge_block);

        // Create phi node if values are compatible
        if then_value.get_type() == else_value.get_type() {
            let phi = self
                .builder
                .build_phi(then_value.get_type(), "ternaryphi")
                .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build phi: {}", e)))?;
            phi.add_incoming(&[(&then_value, then_end_block), (&else_value, else_end_block)]);
            Ok(phi.as_basic_value())
        } else {
            Ok(then_value)
        }
    }

    /// Get a function by name
    fn get_function(&self, name: &str) -> IRGenResult<FunctionValue<'ctx>> {
        self.functions
            .get(name)
            .copied()
            .ok_or_else(|| IRGenError::UndefinedFunction(name.to_string()))
    }
}
