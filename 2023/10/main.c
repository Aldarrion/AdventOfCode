#include <stdio.h>
#include <string.h>

char DATA[64 * 1024];
int FILL[64 * 1024];

typedef struct
{
    int x;
    int y;
} Point;

int fringe_c;
Point FRINGE[64 * 1024];
Point FRINGE_NEXT[64 * 1024];
int WRITTEN[64 * 1024];

int width = 0;
int height = 0;

void PrintArr(int* arr, int width, int height)
{
    for (int y = 0; y < height; ++y)
    {
        for (int x = 0; x < width; ++x)
        {
            int c = arr[y * width + x];
            if (c == -1)
                printf(".");
            else
                printf("%d", c);
        }
        printf("\n");
    }

    printf("\n");
}

void PrintChars(char* arr, int width, int height)
{
    for (int y = 0; y < height; ++y)
    {
        for (int x = 0; x < width; ++x)
        {
            char c = arr[y * width + x];
            if (c == -1)
                printf(".");
            else
                printf("%c", c);
        }
        printf("\n");
    }

    printf("\n");
}

void TryAddPoint(Point* fringe, int* fringe_c, Point p, const char* allowed)
{
    if (p.x < 0 || p.x >= width || p.y < 0 || p.y >= height)
        return;

    char new = DATA[p.y * width + p.x];
    int canAdd = 0;
    const char* c = allowed;
    while (*c)
    {
        if (new == *c)
            canAdd = 1;
        ++c;
    }

    if (canAdd)
    {
        fringe[*fringe_c] = p;
        *fringe_c += 1;
    }
}

int main()
{
    FILE* file = fopen("input.txt", "rb");

    if (!file)
    {
        printf("Wrong file\n");
        return 1;
    }

    char buff[1024];

    int data_c = 0;

    while (fgets(buff, sizeof(buff), file))
    {
        if (!width)
            width = strlen(buff) - 1;

        for (int i = 0; i < width; ++i)
        {
            DATA[data_c++] = buff[i];
            if (buff[i] == 'S')
                FRINGE[fringe_c++] = (Point){ .x = i, .y = height }; 
        }

        ++height;
    }

    PrintChars(DATA, width, height);

    for (int i = 0; i < data_c; ++i)
        FILL[i] = -1;

    int found = 0;
    int current = 0;
    while (!found)
    {
        int fringeNext_c = 0;
        int written_c = 0;
        for (int i = 0; i < fringe_c; ++i)
        {
            Point p = FRINGE[i];
            int idx = p.y * width + p.x; 
            
            for (int w = 0; w < written_c; ++w)
            {
                if (WRITTEN[w] == idx)
                {
                    found = current;
                    break;
                }
            }

            if (found)
                break;

            if (FILL[idx] != -1)
                continue;

            FILL[idx] = current;
            WRITTEN[written_c++] = idx;

            char c = DATA[p.y * width + p.x];
            switch (c)
            {
                case 'S':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y - 1 }, "|F7");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y + 1 }, "|JL");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x - 1, .y = p.y }, "-FL");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x + 1, .y = p.y }, "-J7");
                    break;
                case '|':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y - 1 }, "|F7");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y + 1 }, "|JL");
                    break;
                case '-':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x - 1, .y = p.y }, "-FL");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x + 1, .y = p.y }, "-J7");
                    break;
                case '7':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x - 1, .y = p.y }, "-FL");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y + 1 }, "|JL");
                    break;
                case 'J':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y - 1 }, "|F7");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x - 1, .y = p.y }, "-FL");
                    break;
                case 'L':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y - 1 }, "|F7");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x + 1, .y = p.y }, "-J7");
                    break;
                case 'F':
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x, .y = p.y + 1 }, "|JL");
                    TryAddPoint(FRINGE_NEXT, &fringeNext_c, (Point){ .x = p.x + 1, .y = p.y }, "-J7");
                    break;
            }
        }

        memcpy(FRINGE, FRINGE_NEXT, sizeof(Point) * fringeNext_c);
        fringe_c = fringeNext_c;

        //PrintArr(FILL, width, height);

        ++current;
    }

    PrintArr(FILL, width, height);

    printf("Result: %d\n", found);
}