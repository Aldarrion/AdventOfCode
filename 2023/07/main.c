#define _CRT_SECURE_NO_WARNINGS

#include <stdlib.h>
#include <stdio.h>

typedef long long int i64;

i64 ParseNum(char** c)
{
    i64 num = 0;
    while (**c == ' ')
        ++(*c);

    while (**c >= '0' && **c <= '9')
    {
        num = num * 10 + (**c - '0');
        ++(*c);
    }
    return num;
}

typedef enum
{
    HIGH,
    PAIR,
    TWO_PAIR,
    THREE,
    FULLHOUSE,
    FOUR,
    FIVE,

    COUNT
} HandType;

typedef struct
{
    HandType type;
    char cards[5];
    i64 bid;
    int id;
} Hand;

int hand_c;
Hand hands[4 * 1024];

void IdentifyHand(Hand* h)
{
    int matches[5] = {};
    for (int i = 0; i < 5; ++i)
    {
        for (int j = 0; j < 5; ++j)
        {
            if (h->cards[i] == h->cards[j])
                ++matches[i];
        }
    }

    int match3 = 0;
    int match2 = 0;

    for (int i = 0; i < 5; ++i)
    {
        if (matches[i] == 5)
        {
            h->type = FIVE;
            return;
        }
        else if (matches[i] == 4)
        {
            h->type = FOUR;
            return;
        }

        if (matches[i] == 3)
        {
            ++match3;
        }
        if (matches[i] == 2)
        {
            ++match2;
        }
    }

    if (match3 && match2)
        h->type = FULLHOUSE;
    else if (match3)
        h->type = THREE;
    else if (match2 == 4)
        h->type = TWO_PAIR;
    else if (match2 == 2)
        h->type = PAIR;
    else
        h-> type = HIGH;
}

int HandCmp(const void* x, const void* y)
{
    const Hand* a = x;
    const Hand* b = y;

    if (a->type > b->type)
        return 1;
    else if (a->type < b->type)
        return -1;

    for (int i = 0; i < 5; ++i)
    {
        if (a->cards[i] > b->cards[i])
            return 1;
        else if (a->cards[i] < b->cards[i])
            return -1;
    }

    return a->id < b->id ? -1 : 1;
}

int main()
{
    FILE* file = fopen("input.txt", "rb");
    if (!file)
    {
        printf("Wrong file");
        return 1;
    }

    char buff[1024];
    while (fgets(buff, sizeof(buff), file))
    {
        Hand* h = &hands[hand_c++];
        for (int i = 0; i < 5; ++i)
        {
            if (buff[i] >= '0' && buff[i] <= '9')
                h->cards[i] = buff[i] - '0';
            else if (buff[i] == 'T')
                h->cards[i] = 10;
            else if (buff[i] == 'J')
                h->cards[i] = 11;
            else if (buff[i] == 'Q')
                h->cards[i] = 12;
            else if (buff[i] == 'K')
                h->cards[i] = 13;
            else if (buff[i] == 'A')
                h->cards[i] = 14;
        }

        char* c = buff + 5;
        h->bid = ParseNum(&c);
        h->id = hand_c - 1;

        IdentifyHand(h);
    }

    qsort(hands, hand_c, sizeof(Hand), &HandCmp);

    i64 result = 0;
    for (int i = 0; i < hand_c; ++i)
    {
        printf("%d\n", hands[i].id);
        result += hands[i].bid * (i + 1);
    }

    printf("Result: %lld\n", result);

    return 0;
}