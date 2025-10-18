; ModuleID = 'main_module'
source_filename = "main_module"

define double @main() {
entry:
  %x = alloca double, align 8
  store double 1.000000e+01, ptr %x, align 8
  %y = alloca double, align 8
  store double 5.000000e+00, ptr %y, align 8
  %x1 = load double, ptr %x, align 8
  %y2 = load double, ptr %y, align 8
  %fadd = fadd double %x1, %y2
  store double %fadd, ptr %x, align 8
  %y3 = load double, ptr %y, align 8
  %fsub = fsub double %y3, 2.000000e+00
  store double %fsub, ptr %y, align 8
  %x4 = load double, ptr %x, align 8
  %fmul = fmul double %x4, 2.000000e+00
  store double %fmul, ptr %x, align 8
  %y5 = load double, ptr %y, align 8
  %fdiv = fdiv double %y5, 3.000000e+00
  store double %fdiv, ptr %y, align 8
  %a = alloca double, align 8
  store double 1.500000e+01, ptr %a, align 8
  %b = alloca double, align 8
  store double 7.000000e+00, ptr %b, align 8
  %a6 = load double, ptr %a, align 8
  %b7 = load double, ptr %b, align 8
  %f2i_l = fptosi double %a6 to i32
  %f2i_r = fptosi double %b7 to i32
  %ibitand = and i32 %f2i_l, %f2i_r
  %i2f = sitofp i32 %ibitand to double
  %c = alloca double, align 8
  store double %i2f, ptr %c, align 8
  %a8 = load double, ptr %a, align 8
  %b9 = load double, ptr %b, align 8
  %f2i_l10 = fptosi double %a8 to i32
  %f2i_r11 = fptosi double %b9 to i32
  %ibitor = or i32 %f2i_l10, %f2i_r11
  %i2f12 = sitofp i32 %ibitor to double
  %d = alloca double, align 8
  store double %i2f12, ptr %d, align 8
  %a13 = load double, ptr %a, align 8
  %b14 = load double, ptr %b, align 8
  %f2i_l15 = fptosi double %a13 to i32
  %f2i_r16 = fptosi double %b14 to i32
  %ibitxor = xor i32 %f2i_l15, %f2i_r16
  %i2f17 = sitofp i32 %ibitxor to double
  %e = alloca double, align 8
  store double %i2f17, ptr %e, align 8
  %a18 = load double, ptr %a, align 8
  %f2i_bitnot = fptosi double %a18 to i32
  %bitnot = xor i32 %f2i_bitnot, -1
  %i2f_bitnot = sitofp i32 %bitnot to double
  %f = alloca double, align 8
  store double %i2f_bitnot, ptr %f, align 8
  %i = alloca double, align 8
  store double 0.000000e+00, ptr %i, align 8
  %i19 = load double, ptr %i, align 8
  %fadd20 = fadd double %i19, 1.000000e+00
  store double %fadd20, ptr %i, align 8
  %i21 = load double, ptr %i, align 8
  %fadd22 = fadd double %i21, 1.000000e+00
  store double %fadd22, ptr %i, align 8
  %i23 = load double, ptr %i, align 8
  %fsub24 = fsub double %i23, 1.000000e+00
  store double %fsub24, ptr %i, align 8
  %i25 = load double, ptr %i, align 8
  %fsub26 = fsub double %i25, 1.000000e+00
  store double %fsub26, ptr %i, align 8
  %x27 = load double, ptr %x, align 8
  %y28 = load double, ptr %y, align 8
  %fgt = fcmp ogt double %x27, %y28
  br i1 %fgt, label %ternary_then, label %ternary_else

ternary_then:                                     ; preds = %entry
  %x29 = load double, ptr %x, align 8
  br label %ternary_merge

ternary_else:                                     ; preds = %entry
  %y30 = load double, ptr %y, align 8
  br label %ternary_merge

ternary_merge:                                    ; preds = %ternary_else, %ternary_then
  %ternaryphi = phi double [ %x29, %ternary_then ], [ %y30, %ternary_else ]
  %max = alloca double, align 8
  store double %ternaryphi, ptr %max, align 8
  %x31 = load double, ptr %x, align 8
  %y32 = load double, ptr %y, align 8
  %feq = fcmp oeq double %x31, %y32
  %isEqual = alloca i1, align 1
  store i1 %feq, ptr %isEqual, align 1
  %x33 = load double, ptr %x, align 8
  %y34 = load double, ptr %y, align 8
  %fne = fcmp one double %x33, %y34
  %isNotEqual = alloca i1, align 1
  store i1 %fne, ptr %isNotEqual, align 1
  %x35 = load double, ptr %x, align 8
  %y36 = load double, ptr %y, align 8
  %fgt37 = fcmp ogt double %x35, %y36
  %isGreater = alloca i1, align 1
  store i1 %fgt37, ptr %isGreater, align 1
  %isEqual38 = load i1, ptr %isEqual, align 1
  br i1 %isEqual38, label %and_rhs, label %and_merge

and_rhs:                                          ; preds = %ternary_merge
  %isNotEqual39 = load i1, ptr %isNotEqual, align 1
  br label %and_merge

and_merge:                                        ; preds = %and_rhs, %ternary_merge
  %and_result = phi i1 [ false, %ternary_merge ], [ %isNotEqual39, %and_rhs ]
  %both = alloca i1, align 1
  store i1 %and_result, ptr %both, align 1
  %isEqual40 = load i1, ptr %isEqual, align 1
  br i1 %isEqual40, label %or_merge, label %or_rhs

or_rhs:                                           ; preds = %and_merge
  %isNotEqual41 = load i1, ptr %isNotEqual, align 1
  br label %or_merge

or_merge:                                         ; preds = %or_rhs, %and_merge
  %or_result = phi i1 [ true, %and_merge ], [ %isNotEqual41, %or_rhs ]
  %either = alloca i1, align 1
  store i1 %or_result, ptr %either, align 1
  ret double 2.025000e+03
}

define double @test_loops() {
entry:
  %count = alloca double, align 8
  store double 0.000000e+00, ptr %count, align 8
  br label %while_cond

while_cond:                                       ; preds = %while_body, %entry
  %count1 = load double, ptr %count, align 8
  %flt = fcmp olt double %count1, 3.000000e+00
  br i1 %flt, label %while_body, label %while_exit

while_body:                                       ; preds = %while_cond
  %count2 = load double, ptr %count, align 8
  %fadd = fadd double %count2, 1.000000e+00
  store double %fadd, ptr %count, align 8
  br label %while_cond

while_exit:                                       ; preds = %while_cond
  %repeat_counter = alloca i32, align 4
  store i32 0, ptr %repeat_counter, align 4
  br label %repeat_cond

repeat_cond:                                      ; preds = %repeat_body, %while_exit
  %counter = load i32, ptr %repeat_counter, align 4
  %repeat_cond3 = icmp slt i32 %counter, 3
  br i1 %repeat_cond3, label %repeat_body, label %repeat_exit

repeat_body:                                      ; preds = %repeat_cond
  %count4 = load double, ptr %count, align 8
  %fadd5 = fadd double %count4, 1.000000e+00
  store double %fadd5, ptr %count, align 8
  %counter6 = load i32, ptr %repeat_counter, align 4
  %inc_counter = add i32 %counter6, 1
  store i32 %inc_counter, ptr %repeat_counter, align 4
  br label %repeat_cond

repeat_exit:                                      ; preds = %repeat_cond
  %j = alloca double, align 8
  store double 0.000000e+00, ptr %j, align 8
  br label %for_cond

for_cond:                                         ; preds = %for_update, %repeat_exit
  %j7 = load double, ptr %j, align 8
  %flt8 = fcmp olt double %j7, 5.000000e+00
  br i1 %flt8, label %for_body, label %for_exit

for_body:                                         ; preds = %for_cond
  %count9 = load double, ptr %count, align 8
  %fadd10 = fadd double %count9, 1.000000e+00
  store double %fadd10, ptr %count, align 8
  br label %for_update

for_update:                                       ; preds = %for_body
  %j11 = load double, ptr %j, align 8
  %fadd12 = fadd double %j11, 1.000000e+00
  store double %fadd12, ptr %j, align 8
  br label %for_cond

for_exit:                                         ; preds = %for_cond
  %count13 = load double, ptr %count, align 8
  ret double %count13
}
