using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

namespace COL.Runtime
{
    /// <summary>
    /// C# bindings for the COL (Configurable Open Language) runtime
    /// </summary>
    public class COLRuntime : IDisposable
    {
        // Native library name - adjust for your platform
        private const string LibraryName = "col_runtime";

        #region Native Enums and Structures

        public enum COLResult
        {
            Success = 0,
            ErrorCompilation = 1,
            ErrorExecution = 2,
            ErrorInvalidHandle = 3,
            ErrorInvalidParameter = 4
        }

        [StructLayout(LayoutKind.Explicit)]
        public struct COLValue
        {
            [FieldOffset(0)]
            public double Number;
            
            [FieldOffset(0)]
            public int Boolean;
            
            [FieldOffset(0)]
            public IntPtr StringPtr;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct COLVariant
        {
            public int ValueType; // 0=number, 1=boolean, 2=string, 3=null
            public COLValue Value;
        }

        #endregion

        #region Native Function Imports

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr col_compile_script([MarshalAs(UnmanagedType.LPStr)] string source);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_call_function(
            IntPtr script,
            [MarshalAs(UnmanagedType.LPStr)] string functionName,
            IntPtr args,
            int argCount,
            ref COLVariant result);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_set_global_variable(
            IntPtr script,
            [MarshalAs(UnmanagedType.LPStr)] string varName,
            ref COLVariant value);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_get_global_variable(
            IntPtr script,
            [MarshalAs(UnmanagedType.LPStr)] string varName,
            ref COLVariant result);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void col_destroy_script(IntPtr script);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void col_free_string(IntPtr stringPtr);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_initialize();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void col_shutdown();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr col_get_last_error();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr col_get_script_error(IntPtr script);

        // Print functionality imports
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void PrintCallback([MarshalAs(UnmanagedType.LPStr)] string message);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void col_register_print_callback(PrintCallback callback);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_print([MarshalAs(UnmanagedType.LPStr)] string message);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_print_number(double value);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        private static extern COLResult col_print_boolean(int value);

        #endregion

        #region Private Fields

        private IntPtr scriptHandle = IntPtr.Zero;
        private bool disposed = false;

        #endregion

        #region Public Properties and Methods

        /// <summary>
        /// Initialize the COL runtime (call once per application)
        /// </summary>
        public static COLResult Initialize()
        {
            return col_initialize();
        }

        /// <summary>
        /// Shutdown the COL runtime (call once per application)
        /// </summary>
        public static void Shutdown()
        {
            col_shutdown();
        }

        /// <summary>
        /// Get the last error message from the native runtime
        /// </summary>
        public static string GetLastError()
        {
            IntPtr errorPtr = col_get_last_error();
            if (errorPtr == IntPtr.Zero)
                return null;
            
            return Marshal.PtrToStringAnsi(errorPtr);
        }

        /// <summary>
        /// Register a print callback function
        /// </summary>
        public static void RegisterPrintCallback(PrintCallback callback)
        {
            col_register_print_callback(callback);
        }

        /// <summary>
        /// Send a print message to the COL runtime
        /// </summary>
        public static COLResult Print(string message)
        {
            return col_print(message);
        }

        /// <summary>
        /// Send a number value to the COL runtime print system
        /// </summary>
        public static COLResult Print(double value)
        {
            return col_print_number(value);
        }

        /// <summary>
        /// Send a boolean value to the COL runtime print system
        /// </summary>
        public static COLResult Print(bool value)
        {
            return col_print_boolean(value ? 1 : 0);
        }

        /// <summary>
        /// Compile GML source code
        /// </summary>
        public bool CompileScript(string gmlSource)
        {
            if (disposed)
                throw new ObjectDisposedException(nameof(COLRuntime));

            // Destroy existing script if any
            if (scriptHandle != IntPtr.Zero)
            {
                col_destroy_script(scriptHandle);
                scriptHandle = IntPtr.Zero;
            }

            scriptHandle = col_compile_script(gmlSource);
            return scriptHandle != IntPtr.Zero;
        }

        /// <summary>
        /// Call a function in the compiled script
        /// </summary>
        public object CallFunction(string functionName, params object[] args)
        {
            if (disposed)
                throw new ObjectDisposedException(nameof(COLRuntime));
            
            if (scriptHandle == IntPtr.Zero)
                throw new InvalidOperationException("No script compiled");

            var result = new COLVariant();
            var callResult = col_call_function(scriptHandle, functionName, IntPtr.Zero, 0, ref result);

            if (callResult != COLResult.Success)
                throw new Exception($"Function call failed: {callResult}");

            return ConvertFromCOLVariant(result);
        }

        /// <summary>
        /// Set a global variable in the script
        /// </summary>
        public bool SetGlobalVariable(string varName, object value)
        {
            if (disposed)
                throw new ObjectDisposedException(nameof(COLRuntime));
            
            if (scriptHandle == IntPtr.Zero)
                return false;

            var variant = ConvertToCOLVariant(value);
            var result = col_set_global_variable(scriptHandle, varName, ref variant);
            
            return result == COLResult.Success;
        }

        /// <summary>
        /// Get a global variable from the script
        /// </summary>
        public object GetGlobalVariable(string varName)
        {
            if (disposed)
                throw new ObjectDisposedException(nameof(COLRuntime));
            
            if (scriptHandle == IntPtr.Zero)
                return null;

            var result = new COLVariant();
            var getResult = col_get_global_variable(scriptHandle, varName, ref result);

            if (getResult != COLResult.Success)
                return null;

            return ConvertFromCOLVariant(result);
        }

        /// <summary>
        /// Get the last error message from this script
        /// </summary>
        public string GetScriptError()
        {
            if (disposed)
                throw new ObjectDisposedException(nameof(COLRuntime));

            if (scriptHandle == IntPtr.Zero)
                return null;

            IntPtr errorPtr = col_get_script_error(scriptHandle);
            if (errorPtr == IntPtr.Zero)
                return null;

            return Marshal.PtrToStringAnsi(errorPtr);
        }

        #endregion

        #region Helper Methods

        private COLVariant ConvertToCOLVariant(object value)
        {
            var variant = new COLVariant();

            switch (value)
            {
                case null:
                    variant.ValueType = 3; // null
                    break;
                case double d:
                    variant.ValueType = 0; // number
                    variant.Value.Number = d;
                    break;
                case float f:
                    variant.ValueType = 0; // number
                    variant.Value.Number = f;
                    break;
                case int i:
                    variant.ValueType = 0; // number
                    variant.Value.Number = i;
                    break;
                case bool b:
                    variant.ValueType = 1; // boolean
                    variant.Value.Boolean = b ? 1 : 0;
                    break;
                case string s:
                    variant.ValueType = 2; // string
                    variant.Value.StringPtr = Marshal.StringToHGlobalAnsi(s);
                    break;
                default:
                    throw new ArgumentException($"Unsupported type: {value.GetType()}");
            }

            return variant;
        }

        private object ConvertFromCOLVariant(COLVariant variant)
        {
            switch (variant.ValueType)
            {
                case 0: // number
                    return variant.Value.Number;
                case 1: // boolean
                    return variant.Value.Boolean != 0;
                case 2: // string
                    if (variant.Value.StringPtr != IntPtr.Zero)
                    {
                        string result = Marshal.PtrToStringAnsi(variant.Value.StringPtr);
                        col_free_string(variant.Value.StringPtr);
                        return result;
                    }
                    return null;
                case 3: // null
                default:
                    return null;
            }
        }

        #endregion

        #region IDisposable Implementation

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!disposed)
            {
                if (scriptHandle != IntPtr.Zero)
                {
                    col_destroy_script(scriptHandle);
                    scriptHandle = IntPtr.Zero;
                }
                disposed = true;
            }
        }

        ~COLRuntime()
        {
            Dispose(false);
        }

        #endregion
    }

    /// <summary>
    /// Example usage of the COL runtime with print functionality
    /// </summary>
    public class Program
    {
        public static void Main()
        {
            // Initialize the runtime
            if (COLRuntime.Initialize() != COLRuntime.COLResult.Success)
            {
                Console.WriteLine("Failed to initialize COL runtime");
                return;
            }

            try
            {
                // Register print service
                COLPrintService.RegisterPrintService();
                
                // Subscribe to print events
                COLPrintService.OnPrintReceived += (message) =>
                {
                    Console.WriteLine($"GML Output: {message}");
                };

                using (var runtime = new COLRuntime())
                {
                    // Test C# side printing
                    Console.WriteLine("=== Testing C# Print Service ===");
                    COLPrintService.Print("Hello from C#!");
                    COLPrintService.Print(42.5);
                    COLPrintService.Print(true);
                    COLPrintService.Print(new { name = "test", value = 123 });

                    Console.WriteLine("\n=== Loading and Executing Sample.gml ===");

                    // Read GML code from Sample.gml file
                    string gmlCode;
                    try
                    {
                        gmlCode = File.ReadAllText("Sample.gml");
                        Console.WriteLine($"Successfully loaded Sample.gml ({gmlCode.Length} characters)");
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine($"Failed to read Sample.gml: {ex.Message}");
                        Console.WriteLine("Using fallback GML code...");
                        gmlCode = @"
                            var x = 5;
                            var y = 10;
                            function add(a, b) {
                                return a + b;
                            }
                            function multiply(a, b) {
                                return a * b;
                            }
                        ";
                    }

                    if (runtime.CompileScript(gmlCode))
                    {
                        Console.WriteLine("GML script compiled successfully!");

                        // Test global variable operations
                        Console.WriteLine("\n=== Testing Global Variables ===");
                        
                        // Set some variables from C#
                        runtime.SetGlobalVariable("csharpValue", 123.456);
                        runtime.SetGlobalVariable("fromCsharp", "Greetings from C#!");
                        runtime.SetGlobalVariable("csharpBoolean", true);
                        runtime.SetGlobalVariable("pi", 3.14159);

                        var csharpValue = runtime.GetGlobalVariable("csharpValue");
                        var fromCsharp = runtime.GetGlobalVariable("fromCsharp");
                        var csharpBoolean = runtime.GetGlobalVariable("csharpBoolean");
                        var pi = runtime.GetGlobalVariable("pi");

                        Console.WriteLine($"csharpValue: {csharpValue}");
                        Console.WriteLine($"fromCsharp: {fromCsharp}");
                        Console.WriteLine($"csharpBoolean: {csharpBoolean}");
                        Console.WriteLine($"pi: {pi}");

                        // Test function calls
                        Console.WriteLine("\n=== Testing Function Calls ===");
                        try
                        {
                            // Test basic arithmetic functions
                            var addResult = runtime.CallFunction("add", 15, 25);
                            Console.WriteLine($"add(15, 25) = {addResult}");

                            var subtractResult = runtime.CallFunction("subtract", 20, 8);
                            Console.WriteLine($"subtract(20, 8) = {subtractResult}");

                            var multiplyResult = runtime.CallFunction("multiply", 6, 7);
                            Console.WriteLine($"multiply(6, 7) = {multiplyResult}");

                            var divideResult = runtime.CallFunction("divide", 24, 4);
                            Console.WriteLine($"divide(24, 4) = {divideResult}");

                            // Test more complex functions
                            var calculateResult = runtime.CallFunction("calculate", 5, 3);
                            Console.WriteLine($"calculate(5, 3) = {calculateResult}");

                            var mathDemoResult = runtime.CallFunction("mathDemo", 10);
                            Console.WriteLine($"mathDemo(10) = {mathDemoResult}");

                            // Test logic functions
                            var logicResult1 = runtime.CallFunction("logicTest", 10, 5);
                            var logicResult2 = runtime.CallFunction("logicTest", 3, 8);
                            Console.WriteLine($"logicTest(10, 5) = {logicResult1}");
                            Console.WriteLine($"logicTest(3, 8) = {logicResult2}");

                            var isPositiveResult1 = runtime.CallFunction("isPositive", 5);
                            var isPositiveResult2 = runtime.CallFunction("isPositive", -3);
                            Console.WriteLine($"isPositive(5) = {isPositiveResult1}");
                            Console.WriteLine($"isPositive(-3) = {isPositiveResult2}");

                            // Test min/max functions
                            var maxResult = runtime.CallFunction("max", 10, 15);
                            var minResult = runtime.CallFunction("min", 10, 15);
                            Console.WriteLine($"max(10, 15) = {maxResult}");
                            Console.WriteLine($"min(10, 15) = {minResult}");

                            // Test mathematical functions
                            var squareResult = runtime.CallFunction("square", 8);
                            Console.WriteLine($"square(8) = {squareResult}");

                            var cubeResult = runtime.CallFunction("cube", 4);
                            Console.WriteLine($"cube(4) = {cubeResult}");

                            var rectangleAreaResult = runtime.CallFunction("rectangleArea", 5, 8);
                            Console.WriteLine($"rectangleArea(5, 8) = {rectangleAreaResult}");

                            var absoluteResult1 = runtime.CallFunction("absolute", -15);
                            var absoluteResult2 = runtime.CallFunction("absolute", 15);
                            Console.WriteLine($"absolute(-15) = {absoluteResult1}");
                            Console.WriteLine($"absolute(15) = {absoluteResult2}");

                            // Test utility functions
                            var testResult = runtime.CallFunction("testFunction");
                            Console.WriteLine($"testFunction() = {testResult}");

                            var sumThreeResult = runtime.CallFunction("sumThree", 10, 20, 30);
                            Console.WriteLine($"sumThree(10, 20, 30) = {sumThreeResult}");

                            var powerOfTwoResult = runtime.CallFunction("powerOfTwo", 4);
                            Console.WriteLine($"powerOfTwo(4) = {powerOfTwoResult}");
                        }
                        catch (Exception ex)
                        {
                            Console.WriteLine($"Function call failed: {ex.Message}");
                            string scriptError = runtime.GetScriptError();
                            if (scriptError != null)
                                Console.WriteLine($"Script error: {scriptError}");
                        }

                        // Test print functionality
                        Console.WriteLine("\n=== Testing Print Functionality ===");
                        COLRuntime.Print("Direct print from C# runtime");
                        COLRuntime.Print(2.71828);
                        COLRuntime.Print(false);

                        // Show final state of variables
                        Console.WriteLine("\n=== Final Variable States ===");
                        var finalPi = runtime.GetGlobalVariable("pi");
                        var finalMessage = runtime.GetGlobalVariable("fromCsharp");
                        Console.WriteLine($"Final pi value: {finalPi}");
                        Console.WriteLine($"Final message: {finalMessage}");
                    }
                    else
                    {
                        Console.WriteLine("Failed to compile GML script");
                        string error = COLRuntime.GetLastError();
                        if (error != null)
                            Console.WriteLine($"Error: {error}");
                            
                        string scriptError = runtime.GetScriptError();
                        if (scriptError != null)
                            Console.WriteLine($"Script error: {scriptError}");
                    }
                }
            }
            finally
            {
                // Cleanup print service
                COLPrintService.UnregisterPrintService();
                
                // Shutdown the runtime
                COLRuntime.Shutdown();
            }
        }
    }
}