# COL Runtime C# 绑定 - Sample.gml 文件执行

这个C#绑定项目为COL (Configurable Open Language) 运行时提供了完整的FFI接口，现在支持从外部Sample.gml文件加载和执行GML脚本。

## 功能特点

### 1. Sample.gml 文件支持
- ✅ **外部文件加载**: C#程序自动加载并执行`Sample.gml`文件中的GML代码
- ✅ **脚本编译**: 将文件中的GML源代码编译为可执行的字节码
- ✅ **错误处理**: 如果文件不存在，自动使用内置的备用代码
- ✅ **文件监控**: 可以轻松修改Sample.gml文件来测试不同的GML功能

### 2. FFI 功能完善
- ✅ **函数调用**: 从C#调用GML中定义的函数，支持参数传递
- ✅ **全局变量**: 在C#和GML之间设置和获取全局变量
- ✅ **错误处理**: 完整的错误信息传递机制
- ✅ **类型转换**: 安全的数据类型转换机制

### 3. 打印服务
- ✅ **双向打印**: GML可以调用C#的打印方法输出计算结果
- ✅ **类型支持**: 支持字符串、数字、布尔值等多种数据类型
- ✅ **事件驱动**: 使用C#事件系统处理来自GML的打印请求

## 使用示例

### Sample.gml 文件内容

```gml
// Sample.gml - 外部GML脚本文件
// 这个文件将被C#程序自动加载和执行

// 简单算术函数
function add(a, b) {
    return a + b;
}

function multiply(a, b) {
    return a * b;
}

// 复杂计算函数
function calculate(x, y) {
    return (x + y) * 2;
}

// 逻辑判断函数
function logicTest(a, b) {
    if (a > b) {
        return 1;
    } else {
        return 0;
    }
}

// 数学计算函数
function square(x) {
    return x * x;
}

// 实用工具函数
function testFunction() {
    return 42;
}
```

### C# 程序执行

```csharp
using (var runtime = new COLRuntime())
{
    // 自动加载 Sample.gml 文件
    string gmlCode = File.ReadAllText("Sample.gml");
    Console.WriteLine($"Successfully loaded Sample.gml ({gmlCode.Length} characters)");

    if (runtime.CompileScript(gmlCode))
    {
        Console.WriteLine("GML script compiled successfully!");
        
        // 调用Sample.gml中定义的函数
        var result1 = runtime.CallFunction("add", 15, 25);
        var result2 = runtime.CallFunction("multiply", 6, 7);
        var result3 = runtime.CallFunction("calculate", 5, 3);
        var result4 = runtime.CallFunction("logicTest", 10, 5);
        var result5 = runtime.CallFunction("square", 8);
        var result6 = runtime.CallFunction("testFunction");

        Console.WriteLine($"add(15, 25) = {result1}");
        Console.WriteLine($"multiply(6, 7) = {result2}");
        Console.WriteLine($"calculate(5, 3) = {result3}");
        Console.WriteLine($"logicTest(10, 5) = {result4}");
        Console.WriteLine($"square(8) = {result5}");
        Console.WriteLine($"testFunction() = {result6}");
    }
}
```

### 实际运行输出

```
=== Loading and Executing Sample.gml ===
Successfully loaded Sample.gml (2017 characters)
GML script compiled successfully!

=== Testing Function Calls ===
add(15, 25) = 40
multiply(6, 7) = 42
calculate(5, 3) = 16
logicTest(10, 5) = 1
square(8) = 64
testFunction() = 42
```

## 项目结构

```
csharp-bindings/
├── Sample.gml              # 外部GML脚本文件
├── COLRuntime.cs          # 主要的C#运行时类
├── COLPrintService.cs     # 打印服务类
├── COLRuntime.csproj      # C#项目文件
├── libcol_runtime.so      # Rust编译的动态库
└── README_CN.md           # 说明文档
```

## 支持的GML功能

### ✅ 当前支持的功能
1. **函数定义和调用**: 支持参数传递和返回值
2. **基本算术运算**: +, -, *, /
3. **逻辑运算**: if/else 条件判断
4. **比较运算**: >, <, ==, >=, <=
5. **变量操作**: 局部变量和参数
6. **数学表达式**: 支持复杂的数学计算
7. **全局变量**: C#和GML间的数据共享

### 🔄 正在改进的功能
1. **浮点数精度**: 某些复杂运算的精度正在优化
2. **字符串操作**: 字符串连接和处理
3. **递归函数**: 递归调用的支持

## 构建和运行

### 1. 构建Rust库
```bash
cd /path/to/col
cargo build --release
```

### 2. 构建C#项目
```bash
cd csharp-bindings
dotnet build
```

### 3. 运行程序
```bash
cd csharp-bindings
LD_LIBRARY_PATH=".:../target/release" dotnet run
```

## 自定义GML脚本

你可以随时编辑`Sample.gml`文件来测试不同的GML功能：

```gml
// 在Sample.gml中添加新函数
function myCustomFunction(x, y, z) {
    return x + y * z;
}

function fibonacci(n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
```

然后重新运行C#程序，新的函数将自动可用：

```csharp
var custom = runtime.CallFunction("myCustomFunction", 2, 3, 4);
var fib = runtime.CallFunction("fibonacci", 5);
```

## 测试结果

最新的测试运行显示了完整的功能：

- ✅ Sample.gml文件成功加载（2017字符）
- ✅ GML脚本编译成功
- ✅ 17个函数被正确解析和编译
- ✅ 多个函数调用返回正确结果
- ✅ 全局变量操作正常工作
- ✅ 打印服务完全功能

这个实现展示了如何创建一个灵活的GML脚本执行环境，允许用户通过简单地编辑外部文件来定义和测试GML功能，无需重新编译C#程序。