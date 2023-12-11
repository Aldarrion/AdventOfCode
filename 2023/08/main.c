// --- Day 8: Haunted Wasteland ---
// 
// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.
// 
// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.
// 
// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
// 
// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.
// 
// This format defines each node of the network individually. For example:
// 
// RL
// 
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// 
// Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.
// 
// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:
// 
// LLR
// 
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// 
// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
// 
// Your puzzle answer was 13301.
// --- Part Two ---
// 
// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!
// 
// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.
// 
// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.
// 
// For example:
// 
// LR
// 
// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)
// 
// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:
// 
//     Step 0: You are at 11A and 22A.
//     Step 1: You choose all of the left paths, leading you to 11B and 22B.
//     Step 2: You choose all of the right paths, leading you to 11Z and 22C.
//     Step 3: You choose all of the left paths, leading you to 11B and 22Z.
//     Step 4: You choose all of the right paths, leading you to 11Z and 22B.
//     Step 5: You choose all of the left paths, leading you to 11B and 22C.
//     Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
// 
// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
// 
// Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
// 
// Your puzzle answer was 7309459565207.
// 
// Both parts of this puzzle are complete! They provide two gold stars: **


#define _CRT_SECURE_NO_WARNINGS

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stddef.h>
#include <inttypes.h>

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
    int starts[512];
    int ends[512];
    int starts_c = 0;
    int ends_c = 0;

    while (fgets(buff, sizeof(buff), file))
    {
        //Direction* dir = &dirs[dirs_c++];
        int from = 0;
        Value value = {};
        memcpy(&from, buff, 3); 
        memcpy(&value.left, buff + 7, 3);
        memcpy(&value.right, buff + 12, 3);

        if (buff[2] == 'A')
        {
            printf("start: %.*s\n", 3, (char*)&from);
            starts[starts_c++] = from;
        }
        else if (buff[2] == 'Z')
        {
            printf("end: %.*s\n", 3, (char*)&from);
            ends[ends_c++] = from;
        }

        hmput(dirs, from, value);
    }

    if (starts_c != ends_c)
    {
        printf("start end don't match\n");
        return 1;
    }

    i64 foundSteps[32] = {};
    i64 foundInterval[32] = {};
    int foundIdx[32] = {};
    int converged[32] = {};
    int allConverged = 0;

    i64 lastMatch[8]= {};

    int matches_c = 0;
    typedef struct
    {
        int i;
        int end;
        i64 steps;
    } Match;
    Match matches[512] = {};

    i64 jump = 1;
    i64 steps = 0;
    while (1)
    {
        int allMatching = 1;
        char inst = instructions[steps++ % (i64)instructions_c];
        
        for (int i = 0; i < starts_c; ++i)
        {
            int pos = starts[i];
            Value val = hmget(dirs, pos);

            if (inst == 'L')
                pos = val.left;
            else
                pos = val.right;

            starts[i] = pos;

            int found = 0;
            for (int j = 0; j < ends_c; ++j)
            {
                if (pos == ends[j])
                {
                    if (!allConverged)
                    {
                        for (int m = 0; m < matches_c; ++m)
                        {
                            if (i == matches[m].i
                                && j == matches[m].end
                                && (steps % (i64)instructions_c) == (matches[m].steps % (i64)instructions_c))
                            {
                                foundInterval[i] = steps - matches[m].steps;
                                converged[i] = 1;
                            }
                        }

                        if (!converged[i])
                            matches[matches_c++] = (Match){ .i = i, .steps = steps, .end = j };
                    }
                    
                    lastMatch[i] = steps;

                    found = 1;
                    break;
                }
            }

            if (!found)
                allMatching = 0;
        }

        int all = 1;
        for (int i = 0; i < starts_c; ++i)
        {
            if (!converged[i])
            {
                printf("not done: %d\n", i);
                all = 0;
            }
        }
        if (!allConverged && all)
        {
            allConverged = 1;
            printf("-- all converged\n");
            jump = 1;
            for (int i = 0; i < starts_c; ++i)
            {
                printf("%lld\n", foundInterval[i]);
                //jump *= foundInterval[i];
            }
        }

        if (allMatching || allConverged)
            break;
    }

    printf("Step 2\n");

    while (1)
    {
        int allMatching = 1;

        int min_i = 0;
        i64 minSteps = INT64_MAX;
        for (int i = 0; i < starts_c; ++i)
        {
            if (lastMatch[i] < minSteps)
            {
                min_i = i;
                minSteps = lastMatch[i];
            }
        }

        lastMatch[min_i] += foundInterval[min_i];

        for (int i = 0; i < starts_c; ++i)
        {
            if (lastMatch[i] != lastMatch[0])
                allMatching = 0;
        }

        if (allMatching)
        {
            steps = lastMatch[0];
            break;
        }
    }
    
    printf("Result: %lld\n", steps);

    return 0;
}