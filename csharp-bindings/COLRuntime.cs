using System;
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
    /// Example usage of the COL runtime
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
                using (var runtime = new COLRuntime())
                {
                    // Compile a simple GML script
                    string gmlCode = @"
                        var x = 5;
                        function test_func(a) {
                            return a + 3;
                        }
                    ";

                    if (runtime.CompileScript(gmlCode))
                    {
                        Console.WriteLine("Script compiled successfully!");

                        // Try to call a function
                        try
                        {
                            var result = runtime.CallFunction("test_func");
                            Console.WriteLine($"Function result: {result}");
                        }
                        catch (Exception ex)
                        {
                            Console.WriteLine($"Function call failed: {ex.Message}");
                        }

                        // Try to set and get variables
                        runtime.SetGlobalVariable("myVar", 42.0);
                        var value = runtime.GetGlobalVariable("myVar");
                        Console.WriteLine($"Global variable myVar: {value}");
                    }
                    else
                    {
                        Console.WriteLine("Failed to compile script");
                        string error = COLRuntime.GetLastError();
                        if (error != null)
                            Console.WriteLine($"Error: {error}");
                    }
                }
            }
            finally
            {
                // Shutdown the runtime
                COLRuntime.Shutdown();
            }
        }
    }
}