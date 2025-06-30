namespace Test;

public class HelloWorld
{
    public static void Main()
    {
        System.Console.WriteLine("And now, math!");
        System.Console.WriteLine("Addition: " + Add(5, 3));
        System.Console.WriteLine("Subtraction: " + Subtract(5, 3));
        System.Console.WriteLine("Multiplication: " + Multiply(5, 3));
        System.Console.WriteLine("Division: " + Divide(6, 3));
        System.Console.WriteLine("Modulus: " + Modulus(5, 3));
        System.Console.WriteLine("Power: " + Power(2, 3));
    }

    public static int Add(int a, int b)
    {
        return a + b;
    }

    internal static int Subtract(int a, int b)
    {
        return a - b;
    }

    protected static int Multiply(int a, int b)
    {
        return a * b;
    }

    private static int Divide(int a, int b)
    {
        return a / b;
    }

    public static int Modulus(int a, int b)
    {
        return a % b;
    }

    public static int Power(int a, int b)
    {
        int result = 1;
        for (int i = 0; i < b; i++)
        {
            result *= a;
        }
        return result;
    }
}
