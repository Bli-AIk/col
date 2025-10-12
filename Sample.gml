/* ============= 基本算术和逻辑表达式 ============= */
{
    // 整数运算
    var basic_add = (1 + 2);       // 3
    var basic_sub = (5 - 3);       // 2
    var basic_mul = (4 * 6);       // 24
    var basic_div = (12 / 4);      // 3
    var basic_mod = (10 % 3);      // 1

    // 小数运算
    var basic_fadd = (1.5 + 2.3);  // 3.8
    var basic_fsub = (5.0 - 2.7);  // 2.3
    var basic_fmul = (1.2 * 3.0);  // 3.6
    var basic_fdiv = (7.5 / 2.5);  // 3.0
    var basic_fcombo = (1.5 + 2.5) * 2.0 / 3.0; // 2.666...

    // 组合运算与括号优先级
    var basic_combo = (1 + 2) * (3 - 1) / 2; // 3

    // 比较运算
    var basic_eq = (1 == 1);        // true
    var basic_neq = (1 != 2);       // true
    var basic_lt = (1 < 2);         // true
    var basic_gt = (2 > 1);         // true
    var basic_le = (2 <= 2);        // true
    var basic_ge = (3 >= 2);        // true

    // 逻辑运算
    var basic_andv = (true && false);   // false
    var basic_orv = (true || false);    // true
    var basic_notv = !true;             // false

    // 复合逻辑与比较
    var basic_complex_logic = ((1 + 2) > 2) && ((5 % 2) == 1); // true
}
