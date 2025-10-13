// Global script code
var a = 42;
var b = "hello";
c = true;

// Function definition
function main(a, b, c) {
    a = 1 + 1;
    b = 1 - 2;

    if (a > 1) {
        b = 2;
    } else {
        b = 3;
    }

    if (c) b = 4;
}

// More global script code
var d = a + 1;


// More Function
function another(a, b, c) {
    a = b && c
}