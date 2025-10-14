; ModuleID = 'main_module'
source_filename = "main_module"

define double @main() {
entry:
  %x = alloca double, align 8
  store double 5.000000e+00, ptr %x, align 8
  ret double 0.000000e+00
}

define double @test_func(double %0) {
entry:
  %a = alloca double, align 8
  store double %0, ptr %a, align 8
  %a1 = load double, ptr %a, align 8
  %fadd = fadd double %a1, 3.000000e+00
  ret double %fadd
}
