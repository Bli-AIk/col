# COL C# Bindings

This directory contains C# bindings for the COL (Configurable Open Language) runtime.

## Files

- `COLRuntime.cs` - Main C# binding class with P/Invoke declarations
- `COLRuntime.csproj` - Project file for building the C# bindings
- `README.md` - This file

## Building

1. Make sure you have the COL native library built:
   ```bash
   cd .. && cargo build --lib
   ```

2. Build the C# project:
   ```bash
   dotnet build
   ```

## Usage

```csharp
using COL.Runtime;

// Initialize the runtime
COLRuntime.Initialize();

try
{
    using (var runtime = new COLRuntime())
    {
        // Compile GML code
        string gmlCode = "var x = 5; function test() { return x * 2; }";
        if (runtime.CompileScript(gmlCode))
        {
            // Call functions
            var result = runtime.CallFunction("test");
            Console.WriteLine($"Result: {result}");
            
            // Set/get variables
            runtime.SetGlobalVariable("myVar", 42.0);
            var value = runtime.GetGlobalVariable("myVar");
        }
    }
}
finally
{
    COLRuntime.Shutdown();
}
```

## Platform Notes

- On Linux: Make sure `libcol_runtime.so` is in your library path
- On Windows: Make sure `col_runtime.dll` is accessible
- On macOS: Make sure `libcol_runtime.dylib` is accessible

You may need to copy the native library to your output directory or set appropriate environment variables.