use crate::codegen::ir_generator::{IRGenError, IRGenResult, IRGenerator};
use crate::parser::stmt::Stmt;
use inkwell::values::BasicValueEnum;

impl<'ctx> IRGenerator<'ctx> {
    pub fn visit_stmt_impl(&mut self, stmt: &Stmt) -> IRGenResult<BasicValueEnum<'ctx>> {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr_impl(expr),

            Stmt::Var(vars) => {
                let mut last_value = self.gen_number_const(0.0).into();
                for (name, init_expr) in vars {
                    let value = if let Some(expr) = init_expr {
                        self.visit_expr_impl(expr)?
                    } else {
                        self.gen_number_const(0.0).into()
                    };

                    let alloca = self.declare_variable(name, self.get_value_type(value))?;
                    self.builder.build_store(alloca, value).map_err(|e| {
                        IRGenError::InvalidOperation(format!(
                            "Failed to store variable '{}': {}",
                            name, e
                        ))
                    })?;
                    last_value = value;
                }
                Ok(last_value)
            }

            Stmt::If(cond, then_stmt, else_stmt) => {
                let cond_value = self.visit_expr_impl(cond)?;

                let current_fn = self.current_function.ok_or_else(|| {
                    IRGenError::InvalidOperation("If statement outside function".to_string())
                })?;

                let then_block = self.context.append_basic_block(current_fn, "then");
                let else_block = self.context.append_basic_block(current_fn, "else");
                let merge_block = self.context.append_basic_block(current_fn, "merge");

                // Convert condition to i1
                let cond_i1 = self.convert_to_bool(cond_value)?;

                self.builder
                    .build_conditional_branch(cond_i1, then_block, else_block)
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!(
                            "Failed to build conditional branch: {}",
                            e
                        ))
                    })?;

                // Generate then block
                self.builder.position_at_end(then_block);
                let then_value = self.visit_stmt_impl(then_stmt)?;

                // Check if then block has terminator and note the final block
                let then_block_after = self.builder.get_insert_block();
                let then_has_terminator = then_block_after
                    .map(|bb| bb.get_terminator().is_some())
                    .unwrap_or(false);

                // Add branch to merge if no terminator
                if !then_has_terminator && then_block_after.is_some() {
                    self.builder
                        .build_unconditional_branch(merge_block)
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                        })?;
                }

                // Generate else block
                self.builder.position_at_end(else_block);
                let else_value = if let Some(else_stmt) = else_stmt {
                    self.visit_stmt_impl(else_stmt)?
                } else {
                    self.gen_number_const(0.0).into()
                };

                // Check if else block has terminator and note the final block
                let else_block_after = self.builder.get_insert_block();
                let else_has_terminator = else_block_after
                    .map(|bb| bb.get_terminator().is_some())
                    .unwrap_or(false);

                // Add branch to merge if no terminator
                if !else_has_terminator && else_block_after.is_some() {
                    self.builder
                        .build_unconditional_branch(merge_block)
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                        })?;
                }

                // Position at merge block
                self.builder.position_at_end(merge_block);

                // Handle different flow combinations
                if !then_has_terminator && !else_has_terminator {
                    // Both blocks flow to merge, create phi node if types match
                    if then_value.get_type() == else_value.get_type()
                        && then_block_after.is_some()
                        && else_block_after.is_some()
                    {
                        let phi = self
                            .builder
                            .build_phi(then_value.get_type(), "ifphi")
                            .map_err(|e| {
                                IRGenError::InvalidOperation(format!("Failed to build phi: {}", e))
                            })?;
                        phi.add_incoming(&[
                            (&then_value, then_block_after.unwrap()),
                            (&else_value, else_block_after.unwrap()),
                        ]);
                        Ok(phi.as_basic_value())
                    } else {
                        Ok(then_value)
                    }
                } else if !then_has_terminator {
                    // Only then block flows to merge
                    Ok(then_value)
                } else if !else_has_terminator {
                    // Only else block flows to merge
                    Ok(else_value)
                } else {
                    // Both blocks have terminators, merge block is unreachable
                    self.builder.build_unreachable().map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build unreachable: {}", e))
                    })?;
                    Ok(self.gen_number_const(0.0).into())
                }
            }

            Stmt::Block(stmts) => {
                let mut last_value = self.gen_number_const(0.0).into();
                for stmt in stmts {
                    // Check if current block already has a terminator
                    if let Some(current_block) = self.builder.get_insert_block() {
                        if current_block.get_terminator().is_some() {
                            // Current block is terminated, skip remaining statements
                            break;
                        }
                    }
                    last_value = self.visit_stmt_impl(stmt)?;
                }
                Ok(last_value)
            }

            Stmt::Return(expr_opt) => {
                let value = if let Some(expr) = expr_opt {
                    let expr_value = self.visit_expr_impl(expr)?;
                    self.convert_to_return_type(expr_value)?
                } else {
                    self.gen_number_const(0.0).into()
                };
                self.builder.build_return(Some(&value)).map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to build return: {}", e))
                })?;
                Ok(value)
            }

            Stmt::Break => {
                // Break statement - should only be used in loops
                // For now, we'll generate an unreachable instruction
                self.builder.build_unreachable().map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to build break: {}", e))
                })?;
                Ok(self.gen_number_const(0.0).into())
            }

            Stmt::Continue => {
                // Continue statement - should only be used in loops
                // For now, we'll generate an unreachable instruction
                self.builder.build_unreachable().map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to build continue: {}", e))
                })?;
                Ok(self.gen_number_const(0.0).into())
            }

            Stmt::While(cond, body) => self.generate_while_loop(cond, body),

            Stmt::DoUntil(body, cond) => self.generate_do_until_loop(body, cond),

            Stmt::Repeat(count_expr, body) => self.generate_repeat_loop(count_expr, body),

            Stmt::For(init, cond, update, body) => {
                // Generate for loop with correct parameter types
                let init_as_ref = init.as_deref();
                let cond_as_ref = cond.as_deref();
                let update_as_ref = update.as_deref();
                self.generate_for_loop(init_as_ref, cond_as_ref, update_as_ref, body)
            }
        }
    }

    fn generate_while_loop(
        &mut self,
        cond: &crate::parser::expr::Expr,
        body: &Stmt,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("While loop outside function".to_string())
        })?;

        let cond_block = self.context.append_basic_block(current_fn, "while_cond");
        let body_block = self.context.append_basic_block(current_fn, "while_body");
        let exit_block = self.context.append_basic_block(current_fn, "while_exit");

        // Jump to condition block
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;

        // Generate condition block
        self.builder.position_at_end(cond_block);
        let cond_value = self.visit_expr_impl(cond)?;
        let cond_i1 = self.convert_to_bool(cond_value)?;

        self.builder
            .build_conditional_branch(cond_i1, body_block, exit_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;

        // Generate body block
        self.builder.position_at_end(body_block);
        self.visit_stmt_impl(body)?;

        // Jump back to condition (if no terminator)
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                    })?;
            }
        }

        // Position at exit block
        self.builder.position_at_end(exit_block);
        Ok(self.gen_number_const(0.0).into())
    }

    fn generate_do_until_loop(
        &mut self,
        body: &Stmt,
        cond: &crate::parser::expr::Expr,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("Do-until loop outside function".to_string())
        })?;

        let body_block = self.context.append_basic_block(current_fn, "do_body");
        let cond_block = self.context.append_basic_block(current_fn, "do_cond");
        let exit_block = self.context.append_basic_block(current_fn, "do_exit");

        // Jump to body block first
        self.builder
            .build_unconditional_branch(body_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;

        // Generate body block
        self.builder.position_at_end(body_block);
        self.visit_stmt_impl(body)?;

        // Jump to condition (if no terminator)
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                    })?;
            }
        }

        // Generate condition block
        self.builder.position_at_end(cond_block);
        let cond_value = self.visit_expr_impl(cond)?;

        // Convert condition to i1 if needed (note: until means loop while NOT condition)
        let cond_i1 = match cond_value {
            BasicValueEnum::IntValue(int_val) => {
                let bool_val = if int_val.get_type() == self.type_mapping.get_bool_type() {
                    int_val
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
                                "Failed to convert to bool: {}",
                                e
                            ))
                        })?
                };
                // Invert for "until" semantics
                self.builder.build_not(bool_val, "not").map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to build not: {}", e))
                })?
            }
            BasicValueEnum::FloatValue(float_val) => {
                // until condition is true, so continue while condition is false
                self.builder
                    .build_float_compare(
                        inkwell::FloatPredicate::OEQ, // Equal to zero means false, continue
                        float_val,
                        float_val.get_type().const_zero(),
                        "until_cond",
                    )
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!(
                            "Failed to convert float to bool: {}",
                            e
                        ))
                    })?
            }
            _ => {
                return Err(IRGenError::TypeMismatch(
                    "Invalid condition type".to_string(),
                ));
            }
        };

        self.builder
            .build_conditional_branch(cond_i1, body_block, exit_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;

        // Position at exit block
        self.builder.position_at_end(exit_block);
        Ok(self.gen_number_const(0.0).into())
    }

    fn generate_repeat_loop(
        &mut self,
        count_expr: &crate::parser::expr::Expr,
        body: &Stmt,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self.current_function.ok_or_else(|| {
            IRGenError::InvalidOperation("Repeat loop outside function".to_string())
        })?;

        // Generate the count value
        let count_value = self.visit_expr_impl(count_expr)?;

        // Convert to integer if it's a float
        let count_int = match count_value {
            BasicValueEnum::IntValue(int_val) => int_val,
            BasicValueEnum::FloatValue(float_val) => self
                .builder
                .build_float_to_signed_int(
                    float_val,
                    self.type_mapping.get_int_type(),
                    "repeat_count",
                )
                .map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to convert float to int: {}", e))
                })?,
            _ => {
                return Err(IRGenError::TypeMismatch(
                    "Repeat count must be numeric".to_string(),
                ));
            }
        };

        // Allocate counter variable
        let counter_alloca = self
            .builder
            .build_alloca(self.type_mapping.get_int_type(), "repeat_counter")
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to allocate counter: {}", e))
            })?;
        let zero = self.type_mapping.get_int_type().const_zero();
        self.builder
            .build_store(counter_alloca, zero)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to store counter: {}", e)))?;

        let cond_block = self.context.append_basic_block(current_fn, "repeat_cond");
        let body_block = self.context.append_basic_block(current_fn, "repeat_body");
        let exit_block = self.context.append_basic_block(current_fn, "repeat_exit");

        // Jump to condition block
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;

        // Generate condition block
        self.builder.position_at_end(cond_block);
        let current_counter = self
            .builder
            .build_load(self.type_mapping.get_int_type(), counter_alloca, "counter")
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to load counter: {}", e)))?;

        let cond_result = if let BasicValueEnum::IntValue(counter_val) = current_counter {
            self.builder
                .build_int_compare(
                    inkwell::IntPredicate::SLT,
                    counter_val,
                    count_int,
                    "repeat_cond",
                )
                .map_err(|e| {
                    IRGenError::InvalidOperation(format!("Failed to compare counter: {}", e))
                })?
        } else {
            return Err(IRGenError::TypeMismatch(
                "Counter should be integer".to_string(),
            ));
        };

        self.builder
            .build_conditional_branch(cond_result, body_block, exit_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;

        // Generate body block
        self.builder.position_at_end(body_block);
        self.visit_stmt_impl(body)?;

        // Increment counter (if no terminator)
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                let current_counter = self
                    .builder
                    .build_load(self.type_mapping.get_int_type(), counter_alloca, "counter")
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to load counter: {}", e))
                    })?;

                if let BasicValueEnum::IntValue(counter_val) = current_counter {
                    let one = self.type_mapping.get_int_type().const_int(1, false);
                    let incremented = self
                        .builder
                        .build_int_add(counter_val, one, "inc_counter")
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!(
                                "Failed to increment counter: {}",
                                e
                            ))
                        })?;
                    self.builder
                        .build_store(counter_alloca, incremented)
                        .map_err(|e| {
                            IRGenError::InvalidOperation(format!("Failed to store counter: {}", e))
                        })?;
                }

                self.builder
                    .build_unconditional_branch(cond_block)
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                    })?;
            }
        }

        // Position at exit block
        self.builder.position_at_end(exit_block);
        Ok(self.gen_number_const(0.0).into())
    }

    fn generate_for_loop(
        &mut self,
        init: Option<&Stmt>,
        cond: Option<&crate::parser::expr::Expr>,
        update: Option<&Stmt>,
        body: &Stmt,
    ) -> IRGenResult<BasicValueEnum<'ctx>> {
        let current_fn = self
            .current_function
            .ok_or_else(|| IRGenError::InvalidOperation("For loop outside function".to_string()))?;

        // Execute initialization if present
        if let Some(init_stmt) = init {
            self.visit_stmt_impl(init_stmt)?;
        }

        let cond_block = self.context.append_basic_block(current_fn, "for_cond");
        let body_block = self.context.append_basic_block(current_fn, "for_body");
        let update_block = self.context.append_basic_block(current_fn, "for_update");
        let exit_block = self.context.append_basic_block(current_fn, "for_exit");

        // Jump to condition block
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;

        // Generate condition block
        self.builder.position_at_end(cond_block);
        let continue_loop = if let Some(cond_expr) = cond {
            let cond_value = self.visit_expr_impl(cond_expr)?;
            self.convert_to_bool(cond_value)?
        } else {
            // No condition means infinite loop
            self.type_mapping.get_bool_type().const_int(1, false)
        };

        self.builder
            .build_conditional_branch(continue_loop, body_block, exit_block)
            .map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e))
            })?;

        // Generate body block
        self.builder.position_at_end(body_block);
        self.visit_stmt_impl(body)?;

        // Jump to update (if no terminator)
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder
                    .build_unconditional_branch(update_block)
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build branch: {}", e))
                    })?;
            }
        }

        // Generate update block
        self.builder.position_at_end(update_block);
        if let Some(update_stmt) = update {
            // Need to handle update as a statement, not expression
            self.visit_stmt_impl(update_stmt)?;
        }

        // Jump back to condition
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;

        // Position at exit block and add terminator if needed
        self.builder.position_at_end(exit_block);

        // For loops with infinite conditions (;;), the exit block is unreachable
        // but still needs a terminator for LLVM verification
        if cond.is_none() {
            // Infinite loop case - exit block is unreachable
            self.builder.build_unreachable().map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build unreachable: {}", e))
            })?;
        }
        // For normal loops, the exit block should already be properly handled by conditional branches

        Ok(self.gen_number_const(0.0).into())
    }
}
