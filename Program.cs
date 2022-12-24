using System.Text;

namespace BrainfuckInterpreter;

public static class Program
{
    public static void Main(string[] args) {
        if (!args.Any()) {
            Console.WriteLine("Could not find any files.");

            return;
        }

        Console.WriteLine("OUTPUT");
        Console.WriteLine();
            
        foreach (string filePath in args) {
            
            StringBuilder builder = new();

            builder.Append(Path.GetFileName(filePath));
            builder.Append(' ');
            builder.Append('=', 50 - Path.GetFileName(filePath).Length);

            Console.WriteLine(builder);
            Console.WriteLine();
            Console.WriteLine();

            new Interpreter(filePath).Interpret();

            builder = new StringBuilder();
            builder.Append('=', 51);

            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine();
            Console.WriteLine(builder);
            Console.WriteLine();
        }

        Console.ReadKey();
    }
}