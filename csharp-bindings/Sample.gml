// Sample GML script for COL Runtime C# bindings
// This file demonstrates GML features that work with the current implementation

// Simple arithmetic function
function add(a, b) {
    return a + b;
}

// Subtraction function
function subtract(a, b) {
    return a - b;
}

// Multiplication function
function multiply(a, b) {
    return a * b;
}

// Division function
function divide(a, b) {
    return a / b;
}

// Function with more complex operations
function calculate(x, y) {
    return (x + y) * 2;
}

// Function that demonstrates different operations
function mathDemo(value) {
    return ((value + 10) * 2) - 5;
}

// Boolean logic function
function logicTest(a, b) {
    if (a > b) {
        return 1;
    } else {
        return 0;
    }
}

// Simple conditional function
function isPositive(x) {
    if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

// Function that returns maximum of two numbers
function max(a, b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Function that returns minimum of two numbers
function min(a, b) {
    if (a < b) {
        return a;
    } else {
        return b;
    }
}

// Square function
function square(x) {
    return x * x;
}

// Cube function
function cube(x) {
    return x * x * x;
}

// Area of rectangle
function rectangleArea(width, height) {
    return width * height;
}

// Simple absolute value function
function absolute(x) {
    if (x < 0) {
        return -x;
    } else {
        return x;
    }
}

// Utility function for testing
function testFunction() {
    return 42;
}

// Function that tests parameter passing
function sumThree(a, b, c) {
    return a + b + c;
}

// Power of 2 function (without recursion)
function powerOfTwo(exp) {
    if (exp == 0) {
        return 1;
    } else if (exp == 1) {
        return 2;
    } else if (exp == 2) {
        return 4;
    } else if (exp == 3) {
        return 8;
    } else if (exp == 4) {
        return 16;
    } else {
        return 32;
    }
}