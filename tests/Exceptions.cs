using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace Test;

public class Exceptions
{
    public static void Main()
    {
        try
        {
            Console.WriteLine("Testing exception handling...");
            TestExceptionHandling();
        }
        catch (Exception ex)
        {
            Console.WriteLine("Caught an exception: " + ex.Message);
        }
        finally
        {
            Console.WriteLine("Finally block executed.");
        }
    }

    private static void TestExceptionHandling()
    {
        throw new InvalidOperationException("This is a test exception.");
    }
}
