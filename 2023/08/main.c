#define _CRT_SECURE_NO_WARNINGS

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define STB_DS_IMPLEMENTATION
#include "stb_ds.h"

typedef long long int i64;

typedef struct
{
    int left;
    int right;
} Value;

typedef struct
{
    int key;
    Value value;
} Direction;

int main()
{
    FILE* file = fopen("input.txt", "rb");
    if (!file)
    {
        printf("Wrong file");
        return 1;
    }

    char instructions[4 * 1024];

    char* line = fgets(instructions, sizeof(instructions), file);
    int instructions_c = strlen(instructions);
    --instructions_c;
    printf("instructions: %d\n", instructions_c);

    char buff[1024];
    line = fgets(buff, sizeof(buff), file);

    //int dirs_c = 0;
    //Direction dirs[1024];
    Direction* dirs = NULL;

    while (fgets(buff, sizeof(buff), file))
    {
        //Direction* dir = &dirs[dirs_c++];
        int from = 0;
        Value value = {};
        memcpy(&from, buff, 3); 
        memcpy(&value.left, buff + 7, 3);
        memcpy(&value.right, buff + 12, 3);

        hmput(dirs, from, value);
    }

    int pos = 0;
    int end = 0;
    memcpy(&pos, "AAA", 3);
    memcpy(&end, "ZZZ", 3);
    int steps = 0;
    while (1)
    {
        char inst = instructions[steps++ % instructions_c];
        Value val = hmget(dirs, pos);

        printf("pos: %.*s", 3, (char*)&pos);

        if (inst == 'L')
            pos = val.left;
        else
            pos = val.right;

        printf(" %c -> %.*s\n", inst, 3, (char*)&pos);

        if (pos == end)
            break;
    }
    
    printf("Result: %d\n", steps);

    return 0;
}