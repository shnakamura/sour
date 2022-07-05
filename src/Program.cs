using System.Text;

namespace BrainfuckInterpreter
{
    public static class Program
    {
        private static byte[] memory = Array.Empty<byte>();
        private static int pointer;

        public static void Main(string[] args)
        {
            if (!args.Any())
            {
                Console.WriteLine("Could not find any files.");
            }
            else
            {
                foreach (string filePath in args)
                {
                    StringBuilder builder = new();

                    builder.Append(Path.GetFileName(filePath));
                    builder.Append(' ');
                    builder.Append('=', 50 - Path.GetFileName(filePath).Length);

                    Console.WriteLine(builder);
                    Console.WriteLine();

                    InterpretFile(File.ReadAllText(filePath).ToCharArray());

                    builder = new StringBuilder();
                    builder.Append('=', 51);

                    Console.WriteLine();
                    Console.WriteLine();
                    Console.WriteLine(builder);
                    Console.WriteLine();
                }
            }

            Console.ReadKey();
        }

        private static void InterpretFile(char[] fileContent)
        {
            memory = new byte[30000];
            pointer = 0;

            for (int i = 0; i < fileContent.Length; i++)
            {
                switch (fileContent[i])
                {
                    case '>':
                        pointer++;

                        if (pointer > memory.Length)
                        {
                            pointer = 0;
                        }
                        break;

                    case '<':
                        pointer--;

                        if (pointer < 0)
                        {
                            pointer = memory.Length - 1;
                        }
                        break;

                    case '+':
                        memory[pointer]++;
                        break;

                    case '-':
                        memory[pointer]--;
                        break;

                    case '.':
                        Console.Write(Convert.ToChar(memory[pointer]));
                        break;

                    case ',':
                        memory[pointer] = Convert.ToByte(Console.ReadKey());
                        break;

                    case '[':
                        if (memory[pointer] == 0)
                        {
                            int skip = 0;
                            int ptr = i + 1;

                            while (fileContent[ptr] != ']' || skip > 0)
                            {
                                switch (fileContent[ptr])
                                {
                                    case '[':
                                        skip++;
                                        break;

                                    case ']':
                                        skip--;
                                        break;
                                }

                                ptr++;
                                i = ptr;
                            }
                        }
                        break;

                    case ']':
                        if (memory[pointer] != 0)
                        {
                            int skip = 0;
                            int ptr = i - 1;

                            while (fileContent[ptr] != '[' || skip > 0)
                            {
                                switch (fileContent[ptr])
                                {
                                    case '[':
                                        skip--;
                                        break;

                                    case ']':
                                        skip++;
                                        break;
                                }

                                ptr--;
                                i = ptr;
                            }
                        }
                        break;
                }
            }
        }
    }
}