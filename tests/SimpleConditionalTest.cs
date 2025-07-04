using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace Test;

public class ConditionalSimple
{
    public static void Main()
    {
        bool a = true;
        bool b = false;

        Assert(a && !b, "Logical AND failed");
        Assert(a || b, "Logical OR failed");
        Assert(!b, "Logical NOT failed");
    }

    public static void Assert(bool condition, string message)
    {
        if (!condition)
        {
            Console.WriteLine("Assertion failed: " + message);
        }
    }
}
