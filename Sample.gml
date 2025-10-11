/* ============= 基本控制结构 ============= */
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

// do...until
i = 0;
do {
    result -= i;
    i += 1;
} until (i < 2);

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
