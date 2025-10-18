using System;
using System.Runtime.InteropServices;

namespace COL.Runtime
{
    /// <summary>
    /// Print service for COL runtime that allows GML scripts to output data
    /// </summary>
    public class COLPrintService
    {
        private static COLRuntime.PrintCallback? _currentCallback;

        /// <summary>
        /// Register this service as the print handler for COL runtime
        /// </summary>
        public static void RegisterPrintService()
        {
            _currentCallback = PrintMessage;
            COLRuntime.RegisterPrintCallback(_currentCallback);
        }

        /// <summary>
        /// Internal print callback method
        /// </summary>
        private static void PrintMessage(string message)
        {
            OnPrintReceived?.Invoke(message);
        }

        /// <summary>
        /// Event fired when a print message is received from GML
        /// </summary>
        public static event Action<string>? OnPrintReceived;

        /// <summary>
        /// Print a message from C# to the COL runtime
        /// </summary>
        public static void Print(string message)
        {
            Console.WriteLine($"[COL Print] {message}");
            OnPrintReceived?.Invoke(message);
        }

        /// <summary>
        /// Print a number value
        /// </summary>
        public static void Print(double value)
        {
            Print(value.ToString());
        }

        /// <summary>
        /// Print a boolean value
        /// </summary>
        public static void Print(bool value)
        {
            Print(value ? "true" : "false");
        }

        /// <summary>
        /// Print an object by converting it to string
        /// </summary>
        public static void Print(object obj)
        {
            Print(obj?.ToString() ?? "null");
        }

        /// <summary>
        /// Clear any registered callbacks
        /// </summary>
        public static void UnregisterPrintService()
        {
            _currentCallback = null;
            OnPrintReceived = null;
        }
    }
}