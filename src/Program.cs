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
                    InterpretFile(File.ReadAllText(filePath).ToCharArray());
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
                        break;

                    case '<':
                        pointer--;
                        break;

                    case '+':
                        memory[pointer]++;
                        break;

                    case '-':
                        memory[pointer]--;
                        break;

                    case '.':
                        Console.WriteLine(Convert.ToChar(memory[pointer]));
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