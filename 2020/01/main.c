
/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?

Your puzzle answer was 436404.
--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

Your puzzle answer was 274879808.
*/

#include <stdio.h>
#include <stdlib.h>

static int input[] =
{
    1130,
    1897,
    1850,
    1218,
    1198,
    1761,
    1082,
    1742,
    1821,
    1464,
    1834,
    1413,
    1917,
    1746,
    1954,
    1942,
    1560,
    1227,
    1852,
    1976,
    1773,
    1404,
    1824,
    1011,
    1532,
    1306,
    1819,
    1739,
    1540,
    1973,
    1436,
    1196,
    1176,
    1856,
    1332,
    1617,
    1895,
    1749,
    1718,
    1536,
    1811,
    113,
    1008,
    1908,
    1799,
    1914,
    1603,
    1782,
    1980,
    1228,
    1838,
    2006,
    1953,
    1846,
    1903,
    1470,
    1774,
    1599,
    1446,
    1324,
    1054,
    1952,
    1928,
    1997,
    1764,
    1943,
    1932,
    1615,
    1428,
    1036,
    721,
    1097,
    1998,
    1033,
    1892,
    1904,
    1803,
    1825,
    1370,
    1836,
    1853,
    1963,
    1469,
    1385,
    246,
    1987,
    1153,
    178,
    1790,
    1927,
    1139,
    1865,
    1804,
    1974,
    1235,
    1681,
    1185,
    2009,
    1894,
    1141,
    1203,
    1808,
    1867,
    1274,
    1891,
    1779,
    1342,
    1920,
    851,
    1994,
    1975,
    1979,
    1880,
    1647,
    1365,
    448,
    1119,
    1256,
    1212,
    1268,
    1878,
    1805,
    1889,
    1870,
    1906,
    1959,
    1898,
    1305,
    1559,
    1088,
    1845,
    1783,
    1841,
    1864,
    1961,
    1267,
    1437,
    1823,
    801,
    1579,
    1538,
    1745,
    1972,
    1259,
    1899,
    1517,
    1940,
    1543,
    1882,
    1933,
    1240,
    1608,
    1263,
    1429,
    1197,
    1508,
    1631,
    1988,
    1350,
    1638,
    1800,
    1999,
    1822,
    1776,
    1896,
    1610,
    1831,
    1921,
    1535,
    1526,
    1491,
    1876,
    1476,
    1945,
    1702,
    1900,
    1814,
    1289,
    1992,
    1859,
    1967,
    1966,
    1283,
    2002,
    1195,
    1066,
    1924,
    1968,
    1835,
    1971,
    1977,
    1430,
    1844,
    1465,
    1595,
    1957,
    1472,
    219,
    1851,
    1955
};

int CmpFun(const void* va, const void* vb)
{
    const int* a = va;
    const int* b = vb;
    if (*a < *b)
        return -1;
    else if (*a > *b)
        return 1;
    else
        return 0;
}

const int DESIRED_SUM = 2020;
const int inputSize = sizeof(input) / sizeof(int);

int FindSumOfTwo(int desired, int** first, int** last)
{
    while (*first != *last)
    {
        int sum = **first + **last;
        if (sum == desired)
        {
            return 0;
        }
        else if (sum < desired)
        {
            ++(*first);
        }
        else if (sum > desired)
        {
            --(*last);
        }
    }

    return 1;
}

int main()
{
    qsort(input, inputSize, sizeof(int), &CmpFun);

    {
        int* first = input;
        int* last = input + inputSize - 1;

        int res = FindSumOfTwo(DESIRED_SUM, &first, &last);
        if (res == 0)
            printf("Result for two:\n%d\n", *first * *last);
        else
            printf("Error, sum of two not found\n");
    }

    int sumOfThreeRes = 0;
    for (int* zero = input; zero != input + inputSize - 2; ++zero)
    {
        int* first = zero + 1;
        int* last = input + inputSize - 1;
        const int desired = DESIRED_SUM - *zero;
        int sumOfThreeRes = FindSumOfTwo(desired, &first, &last);
        if (sumOfThreeRes == 0)
        {
            printf("Result for three:\n%d\n", *zero * *first * *last);
            break;
        }
    }

    if (sumOfThreeRes != 0)
        printf("Error, sum of three not found");

    return 0;
}
