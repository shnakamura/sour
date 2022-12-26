namespace BrainfuckInterpreter;

public sealed class Interpreter
{
    public readonly string FilePath;
    public readonly byte[] Memory;

    private int memoryPointer;

    public Interpreter(string filePath) {
        if (string.IsNullOrEmpty(filePath)) {
            throw new ArgumentNullException(nameof(filePath));
        }

        Memory = new byte[30000];
        FilePath = filePath;
    }

    public void Interpret() {
        char[] fileContent = File.ReadAllText(FilePath).ToCharArray();

        for (int i = 0; i < fileContent.Length; i++) {
            switch (fileContent[i]) {
                case '>':
                    memoryPointer++;

                    if (memoryPointer > Memory.Length) {
                        memoryPointer = 0;
                    }

                    break;

                case '<':
                    memoryPointer--;

                    if (memoryPointer < 0) {
                        memoryPointer = Memory.Length - 1;
                    }

                    break;

                case '+':
                    Memory[memoryPointer]++;

                    break;

                case '-':
                    Memory[memoryPointer]--;

                    break;

                case '.':
                    Console.Write(Convert.ToChar(Memory[memoryPointer]));

                    break;

                case ',':
                    Memory[memoryPointer] = Convert.ToByte(Console.ReadKey());

                    break;

                case '[':
                    if (Memory[memoryPointer] == 0) {
                        int skip = 0;
                        int ptr = i + 1;

                        while (fileContent[ptr] != ']' || skip > 0) {
                            switch (fileContent[ptr]) {
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
                    if (Memory[memoryPointer] != 0) {
                        int skip = 0;
                        int ptr = i - 1;

                        while (fileContent[ptr] != '[' || skip > 0) {
                            switch (fileContent[ptr]) {
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