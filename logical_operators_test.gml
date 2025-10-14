// Test for logical operators: type safety and short-circuit evaluation
// This test demonstrates that the compiler now correctly:
// 1. Handles i1 types without incorrect double casting
// 2. Implements short-circuit evaluation for && and ||

function demonstrate_fixes() {
    var x = 5;
    var y = 3;
    
    // Basic boolean comparisons - these create i1 types
    var isEqual = (x == y);        // false (i1 type)
    var isNotEqual = (x != y);     // true (i1 type)
    var isGreater = (x > y);       // true (i1 type)
    
    // Before fix: these would incorrectly load i1 as double
    // After fix: these correctly use i1 types throughout
    var and_result = isEqual && isNotEqual;   // false && true = false
    var or_result = isEqual || isNotEqual;    // false || true = true
    
    // Complex chained logical operations
    var complex = (x > y) && (y > 0) && (x < 10);  // true && true && true = true
    
    // Short-circuit demonstration with side effects
    var counter = 0;
    
    // This should NOT increment counter (short-circuit)
    var test1 = false && ((counter = counter + 1) > 0);
    
    // This SHOULD increment counter 
    var test2 = true && ((counter = counter + 10) > 0);
    
    // This should NOT increment counter by 100 (short-circuit)
    var test3 = true || ((counter = counter + 100) > 0);
    
    // This SHOULD increment counter by 1000
    var test4 = false || ((counter = counter + 1000) > 0);
    
    return counter;  // Should return 1010 (0 + 10 + 1000)
}

// Global test variables
var a = 7;
var b = 3;
var result = demonstrate_fixes();