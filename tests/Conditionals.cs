using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace Test;

public class Conditionals
{
    public static void Main()
    {
        Console.WriteLine("Starting Conditional Tests...");

        TestBasicIfElse();
        TestNestedIfElse();
        TestComparisonOperators();
        TestLogicalOperators();
        TestForLoops();
        TestWhileLoops();
        TestDoWhileLoops();
        TestForeachLoops();
        TestSwitchStatements();
        TestTernaryOperator();
        TestComplexConditions();
        TestBreakAndContinue();
        TestNullChecks();
        TestStringComparisons();
        TestBooleanLogic();

        Console.WriteLine("All Conditional Tests Completed!");
    }

    public static void TestBasicIfElse()
    {
        Console.WriteLine("Testing Basic If-Else...");

        int a = 10;
        int b = 5;
        bool result = false;

        if (a > b)
        {
            result = true;
        }
        else
        {
            result = false;
        }

        Debug.Assert(result == true, "Basic if-else failed: a > b should be true");

        if (a < b)
        {
            result = true;
        }
        else
        {
            result = false;
        }

        Debug.Assert(result == false, "Basic if-else failed: a < b should be false");

        Console.WriteLine("Basic If-Else tests passed!");
    }

    public static void TestNestedIfElse()
    {
        Console.WriteLine("Testing Nested If-Else...");

        int score = 85;
        string grade = "";

        if (score >= 90)
        {
            grade = "A";
        }
        else if (score >= 80)
        {
            if (score >= 85)
            {
                grade = "B+";
            }
            else
            {
                grade = "B";
            }
        }
        else if (score >= 70)
        {
            grade = "C";
        }
        else
        {
            grade = "F";
        }

        Debug.Assert(grade == "B+", "Nested if-else failed: score 85 should be B+");

        Console.WriteLine("Nested If-Else tests passed!");
    }

    public static void TestComparisonOperators()
    {
        Console.WriteLine("Testing Comparison Operators...");

        int x = 10;
        int y = 20;

        Debug.Assert(x < y, "Less than comparison failed");
        Debug.Assert(y > x, "Greater than comparison failed");
        Debug.Assert(x <= y, "Less than or equal comparison failed");
        Debug.Assert(y >= x, "Greater than or equal comparison failed");
        Debug.Assert(x != y, "Not equal comparison failed");
        Debug.Assert(!(x == y), "Equal comparison failed");

        x = 20;
        Debug.Assert(x == y, "Equal comparison failed");
        Debug.Assert(x <= y, "Less than or equal (equal case) failed");
        Debug.Assert(x >= y, "Greater than or equal (equal case) failed");

        Console.WriteLine("Comparison Operators tests passed!");
    }

    public static void TestLogicalOperators()
    {
        Console.WriteLine("Testing Logical Operators...");

        bool a = true;
        bool b = false;

        Debug.Assert(a && !b, "Logical AND failed");
        Debug.Assert(a || b, "Logical OR failed");
        Debug.Assert(!b, "Logical NOT failed");
        Debug.Assert(!(a && b), "Complex logical expression failed");

        int x = 5;
        int y = 10;
        int z = 15;

        Debug.Assert(x < y && y < z, "Chained logical AND failed");
        Debug.Assert(x > y || y < z, "Mixed logical operators failed");
        Debug.Assert(!(x > y) && (y < z), "Complex logical with NOT failed");

        Console.WriteLine("Logical Operators tests passed!");
    }

    public static void TestForLoops()
    {
        Console.WriteLine("Testing For Loops...");

        int sum = 0;
        for (int i = 1; i <= 5; i++)
        {
            sum += i;
        }
        Debug.Assert(sum == 15, "Basic for loop failed: sum should be 15");

        // Nested for loops
        int product = 1;
        for (int i = 1; i <= 3; i++)
        {
            for (int j = 1; j <= 2; j++)
            {
                product *= i;
            }
        }
        Debug.Assert(product == 36, "Nested for loop failed: product should be 36");

        // For loop with step
        int count = 0;
        for (int i = 0; i < 10; i += 2)
        {
            count++;
        }
        Debug.Assert(count == 5, "For loop with step failed: count should be 5");

        Console.WriteLine("For Loops tests passed!");
    }

    public static void TestWhileLoops()
    {
        Console.WriteLine("Testing While Loops...");

        int i = 0;
        int sum = 0;
        while (i < 5)
        {
            sum += i;
            i++;
        }
        Debug.Assert(sum == 10, "While loop failed: sum should be 10");
        Debug.Assert(i == 5, "While loop failed: i should be 5");

        // While loop with complex condition
        int x = 1;
        int iterations = 0;
        while (x < 100 && iterations < 10)
        {
            x *= 2;
            iterations++;
        }
        Debug.Assert(x == 128, "Complex while loop failed: x should be 128");
        Debug.Assert(iterations == 7, "Complex while loop failed: iterations should be 7");

        Console.WriteLine("While Loops tests passed!");
    }

    public static void TestDoWhileLoops()
    {
        Console.WriteLine("Testing Do-While Loops...");

        int i = 0;
        int count = 0;
        do
        {
            count++;
            i++;
        } while (i < 3);

        Debug.Assert(count == 3, "Do-while loop failed: count should be 3");

        // Do-while that executes only once
        int value = 10;
        int executions = 0;
        do
        {
            executions++;
            value++;
        } while (value < 10);

        Debug.Assert(executions == 1, "Do-while loop failed: should execute at least once");
        Debug.Assert(value == 11, "Do-while loop failed: value should be 11");

        Console.WriteLine("Do-While Loops tests passed!");
    }

    public static void TestForeachLoops()
    {
        Console.WriteLine("Testing Foreach Loops...");

        int[] numbers = { 1, 2, 3, 4, 5 };
        int sum = 0;

        foreach (int num in numbers)
        {
            sum += num;
        }
        Debug.Assert(sum == 15, "Foreach loop failed: sum should be 15");

        string[] words = { "Hello", "World", "Test" };
        string combined = "";

        foreach (string word in words)
        {
            combined += word;
        }
        Debug.Assert(combined == "HelloWorldTest", "Foreach with strings failed");

        // Foreach with List
        List<int> list = new List<int> { 10, 20, 30 };
        int product = 1;

        foreach (int item in list)
        {
            product *= item;
        }
        Debug.Assert(product == 6000, "Foreach with List failed: product should be 6000");

        Console.WriteLine("Foreach Loops tests passed!");
    }

    public static void TestSwitchStatements()
    {
        Console.WriteLine("Testing Switch Statements...");

        // Basic switch
        int day = 3;
        string dayName = "";

        switch (day)
        {
            case 1:
                dayName = "Monday";
                break;
            case 2:
                dayName = "Tuesday";
                break;
            case 3:
                dayName = "Wednesday";
                break;
            case 4:
                dayName = "Thursday";
                break;
            case 5:
                dayName = "Friday";
                break;
            default:
                dayName = "Weekend";
                break;
        }
        Debug.Assert(dayName == "Wednesday", "Basic switch failed");

        // Switch with fall-through
        char grade = 'B';
        string message = "";

        switch (grade)
        {
            case 'A':
            case 'B':
                message = "Good";
                break;
            case 'C':
                message = "Average";
                break;
            case 'D':
            case 'F':
                message = "Poor";
                break;
            default:
                message = "Invalid";
                break;
        }
        Debug.Assert(message == "Good", "Switch with fall-through failed");

        // Switch with string
        string fruit = "apple";
        int calories = 0;

        switch (fruit)
        {
            case "apple":
                calories = 95;
                break;
            case "banana":
                calories = 105;
                break;
            case "orange":
                calories = 62;
                break;
            default:
                calories = 0;
                break;
        }
        Debug.Assert(calories == 95, "String switch failed");

        Console.WriteLine("Switch Statements tests passed!");
    }

    public static void TestTernaryOperator()
    {
        Console.WriteLine("Testing Ternary Operator...");

        int a = 10;
        int b = 5;

        int max = a > b ? a : b;
        Debug.Assert(max == 10, "Ternary operator failed: max should be 10");

        string result = a > b ? "a is greater" : "b is greater";
        Debug.Assert(result == "a is greater", "Ternary with strings failed");

        // Nested ternary
        int x = 15;
        string category = x > 20 ? "high" : x > 10 ? "medium" : "low";
        Debug.Assert(category == "medium", "Nested ternary failed");

        Console.WriteLine("Ternary Operator tests passed!");
    }

    public static void TestComplexConditions()
    {
        Console.WriteLine("Testing Complex Conditions...");

        int age = 25;
        bool hasLicense = true;
        bool hasInsurance = true;
        double income = 50000;

        bool canRentCar = age >= 21 && hasLicense && hasInsurance && income > 30000;
        Debug.Assert(canRentCar, "Complex condition failed: should be able to rent car");

        // Complex nested conditions
        int score = 85;
        bool hasBonus = false;
        bool isPremium = true;

        if ((score > 80 && isPremium) || (score > 90 && !hasBonus))
        {
            hasBonus = true;
        }
        Debug.Assert(hasBonus, "Complex nested condition failed");

        Console.WriteLine("Complex Conditions tests passed!");
    }

    public static void TestBreakAndContinue()
    {
        Console.WriteLine("Testing Break and Continue...");

        // Break in for loop
        int sum = 0;
        for (int i = 1; i <= 10; i++)
        {
            if (i > 5)
                break;
            sum += i;
        }
        Debug.Assert(sum == 15, "Break in for loop failed: sum should be 15");

        // Continue in for loop
        int evenSum = 0;
        for (int i = 1; i <= 10; i++)
        {
            if (i % 2 != 0)
                continue;
            evenSum += i;
        }
        Debug.Assert(evenSum == 30, "Continue in for loop failed: evenSum should be 30");

        // Break in while loop
        int count = 0;
        int value = 1;
        while (true)
        {
            if (value > 100)
                break;
            value *= 2;
            count++;
        }
        Debug.Assert(count == 7, "Break in while loop failed");

        Console.WriteLine("Break and Continue tests passed!");
    }

    public static void TestNullChecks()
    {
        Console.WriteLine("Testing Null Checks...");

        string text = null;
        bool isNull = text == null;
        Debug.Assert(isNull, "Null check failed: text should be null");

        text = "Hello";
        bool isNotNull = text != null;
        Debug.Assert(isNotNull, "Not null check failed: text should not be null");

        // Null coalescing
        string result = text ?? "default";
        Debug.Assert(result == "Hello", "Null coalescing failed");

        text = null;
        result = text ?? "default";
        Debug.Assert(result == "default", "Null coalescing with null failed");

        Console.WriteLine("Null Checks tests passed!");
    }

    public static void TestStringComparisons()
    {
        Console.WriteLine("Testing String Comparisons...");

        string str1 = "Hello";
        string str2 = "Hello";
        string str3 = "hello";

        Debug.Assert(str1 == str2, "String equality failed");
        Debug.Assert(str1 != str3, "String inequality failed");

        bool ignoreCase = string.Equals(str1, str3, StringComparison.OrdinalIgnoreCase);
        Debug.Assert(ignoreCase, "Case-insensitive comparison failed");

        Console.WriteLine("String Comparisons tests passed!");
    }

    public static void TestBooleanLogic()
    {
        Console.WriteLine("Testing Boolean Logic...");

        bool p = true;
        bool q = false;

        // De Morgan's laws
        Debug.Assert(!(p && q) == (!p || !q), "De Morgan's law 1 failed");
        Debug.Assert(!(p || q) == (!p && !q), "De Morgan's law 2 failed");

        // Truth tables
        Debug.Assert(p && p == p, "Boolean identity failed");
        Debug.Assert(p || p == p, "Boolean identity failed");
        Debug.Assert(p && !p == false, "Boolean contradiction failed");
        Debug.Assert(p || !p == true, "Boolean tautology failed");

        Console.WriteLine("Boolean Logic tests passed!");
    }
}
