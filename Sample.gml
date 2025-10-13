// With repeat the given statement is repeated the number of times indicated by the rounded value of the expression.

/*
repeat (<expression>)
{
    <statement>;
    <statement>;
    ...
}
*/
repeat (5)
{
    a++;
    ++b;

    a++
    ++b

    if (a == 1)
        break;
    else
        continue;

    c--;
    --c;

    c--
    --c
}

while(a != 1)
{
    a = a + 1;


    if (b == 1)
        break;
    else
        continue;
}


// A do statement is another way of iterating over one or more statements multiple times, and is really a "do... until" statement as you cannot have one without the other since you are telling GameMaker to do something until a specific expression returns true.
/*
do
{
    <statement>;
    <statement>;
    ...
}
until (<expression>);
*/

do
{
    a = a + 1;

    if (b == 1)
        break;
    else
        continue;
}
until (a != 1);


for (var i = 0; i < 10; i += 1)
{
    i++;

    if (b == 1)
        break;
    else
        continue;
}


