using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace Test;

public class Switch
{
    public static void Main()
    {
        for (int i = 0; i < 5; i++)
        {
            switch (i)
            {
                case 0:
                    Console.WriteLine("Zero");
                    break;
                case 1:
                    Console.WriteLine("One");
                    break;
                case 2:
                    Console.WriteLine("Two");
                    break;
                case 3:
                    Console.WriteLine("Three");
                    break;
                case 4:
                    Console.WriteLine("Four");
                    break;
                default:
                    Console.WriteLine("Unknown number " + i);
                    break;
            }
        }
    }
}
