// --- Day 11: Cosmic Expansion ---
// 
// You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.
// 
// He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.
// 
// Maybe you can help him with the analysis to speed things up?
// 
// The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:
// 
// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....
// 
// The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.
// 
// Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.
// 
// In the above example, three columns and two rows contain no galaxies:
// 
//    v  v  v
//  ...#......
//  .......#..
//  #.........
// >..........<
//  ......#...
//  .#........
//  .........#
// >..........<
//  .......#..
//  #...#.....
//    ^  ^  ^
// 
// These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:
// 
// ....#........
// .........#...
// #............
// .............
// .............
// ........#....
// .#...........
// ............#
// .............
// .............
// .........#...
// #....#.......
// 
// Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:
// 
// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// ............6
// .............
// .............
// .........7...
// 8....9.......
// 
// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)
// 
// For example, here is one of the shortest paths between galaxies 5 and 9:
// 
// ....1........
// .........2...
// 3............
// .............
// .............
// ........4....
// .5...........
// .##.........6
// ..##.........
// ...##........
// ....##...7...
// 8....9.......
// 
// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:
// 
//     Between galaxy 1 and galaxy 7: 15
//     Between galaxy 3 and galaxy 6: 17
//     Between galaxy 8 and galaxy 9: 5
// 
// In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.
// 
// Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
// 
// Your puzzle answer was 9274989.
// --- Part Two ---
// 
// The galaxies are much older (and thus much farther apart) than the researcher initially estimated.
// 
// Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.
// 
// (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)
// 
// Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
// 
// Your puzzle answer was 357134560737.


#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

typedef struct
{
    int x;
    int y;
} Point;

int DATA[1024 * 1024];
int DATA_FINAL[1024 * 1024];

char NO_GALAXY_COL[1024];
char NO_GALAXY_ROW[1024];

typedef long long int i64;

int min(int a, int b)
{
    return a > b ? b : a;
}

int max(int a, int b)
{
    return a < b ? b : a;
}

int main()
{
    FILE* file = fopen("input.txt", "rb");

    memset(NO_GALAXY_COL, 1, sizeof(NO_GALAXY_COL));
    memset(NO_GALAXY_ROW, 1, sizeof(NO_GALAXY_ROW));

    char buff[1024];
    int data_c = 0;
    int width = 0;
    int height = 0;

    Point galaxies[1024 * 64];
    int galaxies_c = 0;

    while (fgets(buff, sizeof(buff), file))
    {
        if (!width)
        {
            width = strlen(buff) - 1;
        }
        
        int noGalaxy = 1;
        for (int i = 0; i < width; ++i)
        {
            if (buff[i] == '#')
            {
                noGalaxy = 0;
                NO_GALAXY_COL[i] = 0;
                galaxies[galaxies_c++] = (Point){ .x = i, .y = height };
            }

            DATA[data_c++] = buff[i];
        }

        NO_GALAXY_ROW[height] = noGalaxy;
        ++height;
    }

    printf("width: %d, height: %d\n", width, height); 

    i64 dist_sum = 0;
    i64 dist_mul = 1000000; // Set to 2 for part 1
    for (int i = 0; i < galaxies_c; ++i)
    {
        for (int j = i; j < galaxies_c; ++j)
        {
            Point a = galaxies[i];
            Point b = galaxies[j];

            i64 dist = 0;

            int from_x = min(a.x, b.x);
            int to_x = max(a.x, b.x);
            for (int x = from_x; x < to_x; ++x)
            {
                if (NO_GALAXY_COL[x])
                    dist += dist_mul;
                else
                    ++dist;
            }

            int from_y = min(a.y, b.y);
            int to_y = max(a.y, b.y);
            for (int y = from_y; y < to_y; ++y)
            {
                if (NO_GALAXY_ROW[y])
                    dist += dist_mul;
                else
                    ++dist;
            }

            //printf("%d, %d: %lld\n", i + 1, j + 1, dist);

            dist_sum += dist;
        }
    }

    printf("Result: %lld\n", dist_sum);

    return 0;
}