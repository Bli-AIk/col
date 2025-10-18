// Test various expressions and statements
var x = 10;
var y = 5;

// Test arithmetic and assignment
x += y;
y -= 2;
x *= 2;
y /= 3;

// Test bitwise operations
var a = 15;
var b = 7;
var c = a & b;
var d = a | b;
var e = a ^ b;
var f = ~a;

// Test increment/decrement
var i = 0;
++i;
i++;
--i;
i--;

// Test ternary operator
var max = x > y ? x : y;

// Test comparison and logical operations
var isEqual = x == y;
var isNotEqual = x != y;
var isGreater = x > y;
var both = isEqual && isNotEqual;
var either = isEqual || isNotEqual;

function test_loops() {
    // Test while loop
    var count = 0;
    while (count < 3) {
        count = count + 1;
    }
    
    // Test repeat loop
    repeat (3) {
        count = count + 1;
    }
    
    // Test for loop
    for (var j = 0; j < 5; j = j + 1) {
        count = count + 1;
    }
    
    return count;
}

return 2025;