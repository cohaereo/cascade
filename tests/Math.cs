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
        System.Console.WriteLine("Fibonacci: " + Fibonacci(9));

        const int iterations = 500_000_000;
        long result = 0;
        for (int i = 1; i <= iterations; i++)
        {
            // Some intentionally expensive integer math
            result += (i * 1234567) / (i % 97 + 1);
            result ^= (result << 3);
            result %= 1_000_000_007;
        }

        System.Console.WriteLine("Random complex math: " + result);
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

    public static int Fibonacci(int n)
    {
        if (n <= 1)
            return n;
        int a = 0, b = 1;
        for (int i = 2; i <= n; i++)
        {
            int temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }

}
