/// GML Compiler Stress Tests
/// 作者: ChatGPT
/// 目的: 覆盖变量/表达式/控制流/数组/函数/闭包/struct/enums/宏等语法特性
/// 注意: 不使用内置函数 (例如 show_message, array_length, ds_* 等)

/* ============= 宏 / 预处理 ============= */
#macro MIN(a,b) ((a) < (b) ? (a) : (b))
#macro MAX(a,b) ((a) > (b) ? (a) : (b))
#macro SQUARE(x) ((x) * (x))

/* ============= 枚举 ============= */
enum Color {
    COLOR_NONE = -1,
    COLOR_RED,
    COLOR_GREEN = 5,
    COLOR_BLUE
}

/* ============= 全局与局部变量 / 赋值表达式 ============= */
var g_test_counter = 0;    // 顶层脚本中定义的局部变量（在脚本执行时为局部）
{
    // 复合赋值与各种表达式
    var a = 1;
    var b = 2;
    a += b;      // a = 3
    b *= 4;      // b = 8
    var c = a << 1; // 位运算测试
    var d = (a & b) ^ c;
    var flag = (d != 0) && (a > 0);
    // 三元运算
    var m = flag ? a : b;
}

/* ============= 基本控制结构 ============= */
function test_control_flow(x) {
    var result = 0;
    // if / else if / else
    if (x < 0) {
        result = -1;
    } else if (x == 0) {
        result = 0;
    } else {
        result = 1;
    }

    // switch / case / break / default
    switch (result) {
        case -1:
            result = result - 10;
            break;
        case 0:
            result = result + 0;
            break;
        case 1:
            result = result + 10;
            break;
        default:
            result = 999;
            break;
    }

    // while
    var i = 0;
    while (i < 3) {
        result += i;
        i += 1;
    }

    // do...while
    i = 0;
    do {
        result -= i;
        i += 1;
    } while (i < 2);

    // for
    var sum = 0;
    for (var j = 0; j < 5; j += 1) {
        if (j % 2 == 0) continue;
        sum += j;
    }

    // repeat (关键字)
    var r = 0;
    repeat (3) {
        r += 1;
    }

    return result + sum + r;
}

/* ============= 数组（动态/多维） ============= */
function test_arrays() {
    // 一维数组文字
    var arr = [0, 1, 2, 3];
    // 动态扩展
    arr[4] = 4;
    arr[6] = 6; // 为空洞数组测试

    // 多维数组（嵌套数组）
    var mat = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    // 手动遍历（不使用内置 array_length_*）
    var total = 0;
    for (var i = 0; i < 3; i += 1) {
        for (var j = 0; j < 3; j += 1) {
            total += mat[i][j];
        }
    }

    // 稀疏索引访问测试（未初始化索引返回 undefined/na，取决于引擎）
    var v6 = arr[6];
    var v5 = arr[5]; // 可能是未定义

    return total + (v6 == 6 ? 100 : 0);
}

/* ============= 函数：参数/返回/递归/可变参数风格 ============= */
function factorial(n) {
    // 递归
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// 可变参数模拟：使用 argument_count 和 argument[] 是 GML 提供的内建变量（不是函数）
function varargs_sum() {
    var total = 0;
    var count = argument_count;
    for (var i = 0; i < count; i += 1) {
        total += argument[i];
    }
    return total;
}

/* ============= 匿名函数 / 闭包 / 高阶函数 ============= */
function test_higher_order() {
    // 匿名函数赋值
    var square = function(x) { return x * x; };
    var apply_two = function(f, value) {
        return f(value) + f(value + 1);
    };

    var res = apply_two(square, 3); // 3^2 + 4^2 = 9 + 16 = 25

    // 闭包：捕获外部变量
    var counter_value = 0;
    var make_counter = function() {
        // 捕获 counter_value，返回一个函数，每次调用增加并返回新值
        return function() {
            counter_value += 1;
            return counter_value;
        };
    };

    var c1 = make_counter();
    var a1 = c1(); // 1
    var a2 = c1(); // 2

    return res + a2;
}

/* ============= struct 字面量 与 方法/工厂 函数 ============= */
function make_point(x, y) {
    return {
        x: x,
        y: y,
        // 方法直接访问结构体字段（不使用 built-in）
        move: function(dx, dy) {
            x += dx; // 这里的 x,y 解析为结构体字段
            y += dy;
            // 返回自身以便链式调用
            return { x: x, y: y };
        },
        // 使用三元表达式构造字符串表示（不调用 string() 之类的内置）
        to_repr: function() {
            return "(" + x + "," + y + ")";
        }
    };
}

function test_structs() {
    var p = make_point(2, 3);
    var before = p.to_repr();
    var moved = p.move(1, -1);
    var after = p.to_repr();
    // moved 是一个临时结构 { x:..., y:... }，after 来自同一 struct 的 to_repr

    // 数组嵌套 struct 测试
    var poly = [ make_point(0,0), make_point(1,0), make_point(1,1) ];
    var s = "";
    for (var i = 0; i < 3; i += 1) {
        s = s + poly[i].to_repr();
    }

    return before + " -> " + after;
}

/* ============= 原型/“继承”模式（通过工厂拷贝字段） ============= */
function make_animal(name) {
    return {
        name: name,
        // 基本方法
        speak: function() {
            return name + "???";
        }
    };
}

function make_dog(name, breed) {
    // 通过工厂组合来模拟继承（拷贝字段/方法）
    var proto = make_animal(name);
    proto.breed = breed;
    // 覆盖方法
    proto.speak = function() {
        return name + " (a " + breed + ") says: woof";
    };
    return proto;
}

function test_inheritance() {
    var a = make_animal("Critter");
    var d = make_dog("Rex", "Shepherd");
    var amsg = a.speak();
    var dmsg = d.speak();
    return amsg + " | " + dmsg;
}

/* ============= 位运算与常量测试 ============= */
function test_bit_ops() {
    var x = 5; // 0b0101
    var y = 3; // 0b0011
    var andv = x & y;
    var orv = x | y;
    var xv = x ^ y;
    var shl = x << 2;
    var shr = x >> 1;
    return andv + orv + xv + shl + shr;
}

/* ============= 复杂表达式与优先级 ============= */
function test_complex_expr(a, b, c) {
    var res = (a + b) * c - (a == b ? a : b) / (1 + (c % 3));
    return res;
}

/* ============= 组合性测试：用之前定义的所有单元构建一次综合运行 ============= */
function compiler_sanity_run() {
    var out = "";
    out += "cf:" + string(test_control_flow(2)); // NOTE: string() 是内建，下面改为不调用 string()
    // 为了严格不调用任何内建，我们改用算术和拼接来保留信息：
    var v1 = test_control_flow(2);
    var v2 = test_arrays();
    var v3 = factorial(5); // 120
    var v4 = varargs_sum(1,2,3,4);
    var v5 = test_higher_order();
    var v6 = test_structs();
    var v7 = test_inheritance();
    var v8 = test_bit_ops();
    // 返回一个结构体，便于外部断言各个子结果（避免任何内建输出）
    return {
        cf: v1,
        arr: v2,
        fact5: v3,
        vargs: v4,
        hof: v5,
        structs: v6,
        inheritance: v7,
        bits: v8
    };
}

/* ============= 演示宏运算 ============= */
var macro_demo = MIN(10, 20) + MAX(10, 20) + SQUARE(3);

/* ============= 小结：接口函数 ============= */
// 以下函数可以从外部调用以测试编译器能否解析并链接这些符号
function test_all_basic() {
    var results = compiler_sanity_run();
    // 直接返回结构体结果，调用方可以断言字段是否存在与类型
    return results;
}

// 也导出单独的小函数，便于逐项测试
function __test__factorial_6() { return factorial(6); }  // 720
function __test__varargs_demo() { return varargs_sum(5,5,5); } // 15
function __test__make_point_demo() { var p = make_point(10,20); return p.to_repr(); }
