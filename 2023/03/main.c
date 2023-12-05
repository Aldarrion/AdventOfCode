// --- Day 3: Gear Ratios ---
// 
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
// 
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
// 
// "Aaah!"
// 
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
// 
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
// 
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
// 
// Here is an example engine schematic:
// 
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// 
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
// 
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
// 
// Your puzzle answer was 540212.
// --- Part Two ---
// 
// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
// 
// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
// 
// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
// 
// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
// 
// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
// 
// Consider the same engine schematic again:
// 
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// 
// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
// 
// What is the sum of all of the gear ratios in your engine schematic?
// 
// Your puzzle answer was 87605697.


#include <stdio.h>
#include <string.h>

typedef enum {
    NONE,
    PART,
    NUMBER,
} Type;

typedef struct {
    Type type;
    int n;
    int id;
    char c;
} Entry;

Entry DATA[1024 * 1024];

int main()
{
    FILE* file = fopen("input.txt", "rb");

    char buff[1024];
    int width = 0;
    int height = 0;

    int dst = 0;
    while (fgets(buff, sizeof(buff), file))
    {
        if (!width)
        {
            width = strlen(buff) - 1;
        }
        
        for (int i = 0; i < width; ++i)
        {
            char c = buff[i];
            if (c == '.')
            {
                DATA[dst + i] = (Entry){ .type = NONE, .n = 0, .c = 0 }; 
            }
            else if (c >= '0' && c <= '9')
            {
                int mul = 1;
                int num_start = i;
                int num = 0;
                while (i < width && c >= '0' && c <= '9')
                {
                    num = mul * num + (c - '0');
                    mul = 10;
                    ++i;
                    c = buff[i];
                }

                for (int j = num_start; j < i; ++j)
                    DATA[dst + j] = (Entry){ .type = NUMBER, .n = num, .c = 0, .id = dst + num_start };
                
                --i;
            }
            else
            {
                DATA[dst + i] = (Entry) { .type = PART, .n = 0, .c = c };
            }
        }
        
        for (int d = 0; d < width; ++d)
        {
            if (DATA[dst + d].type == NUMBER)
                printf("d: %d\n", DATA[dst + d].n);
            else if (DATA[dst + d].type == PART)
                printf("d: %c\n", DATA[dst + d].c);
            else
                printf("d: .\n");
        }
        printf("---\n");

        dst += width;
        ++height;
    }

    printf("W: %d, H: %d, dst: %d\n", width, height, dst);

    int ids_i = 0;
    int ids[64];

    int nums_i = 0;
    int nums[64];

    long long int sum = 0;
    long long int ratioSum = 0;
    for (int y = 0; y < height; ++y)
    {
        for (int x = 0; x < width; ++x)
        {
            Entry e = DATA[y * width + x];
            if (e.type == PART)
            {
                printf("%c\n", e.c);
                ids_i = 0;
                nums_i = 0;
                for (int ny = -1; ny < 2; ++ny)
                {
                    for (int nx = -1; nx < 2; ++nx)
                    {
                        if (nx == 0 && ny == 0)
                            continue;
                        
                        if (x + nx < 0
                            || x + nx >= width
                            || y + ny < 0
                            || y + ny >= height)
                            continue;

                        Entry data = DATA[(y + ny) * width + x + nx];
                        //printf("x: %d y: %d, n: %d, id: %d\n", x + nx, y + ny, data.n, data.id);
                        if (data.type == NUMBER)
                        {
                            int unique = 1;
                            for (int i = 0; i < ids_i; ++i)
                            {
                                if (data.id == ids[i])
                                    unique = 0;
                            }

                            if (unique)
                            {
                                nums[nums_i++] = data.n;
                                sum += data.n;
                                printf("%c: %d\n", e.c, data.n);
                                ids[ids_i++] = data.id;
                            }
                        }
                    }
                }

                if (e.c == '*' && nums_i == 2)
                    ratioSum += nums[0] * nums[1];
            }
            else if (e.type == NUMBER)
            {
                //printf(" %d ", e.n);
            }
            else
            {
                //printf(".");
            }
        }

        //printf("\n");
    }

    printf("Result: %lld\n", sum);
    printf("Result2: %lld\n", ratioSum);

    return 0;
}