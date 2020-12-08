
/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

Your puzzle answer was 177.
--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?

Your puzzle answer was 34988.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdbool.h>

#define max(a, b) (a > b ? a : b)

struct ChildNode
{
    int count_;
    struct Node* node_;
};

struct Node
{
    char* name_;

    struct ChildNode* parents_;
    int parentsCount_;
    int parentsCapacity_;

    struct ChildNode* children_;
    int childrenCount_;
    int childrenCapacity_;
};

struct VisitedNode
{
    struct Node* node_;
    int count_;
};

int nodeCapacity = 1024;
int nodeCount = 0;
struct Node** nodes;

//------------------------------------------------------------------------------
struct Node* GetNode(char* name, bool canCreate)
{
    struct Node* node = NULL;
    for (int i = 0; i < nodeCount; ++i)
    {
        if (strcmp(nodes[i]->name_, name) == 0)
        {
            node = nodes[i];
            break;
        }
    }

    if (!node && canCreate)
    {
        if (nodeCount == nodeCapacity)
        {
            nodeCapacity *= 2;
            nodes = realloc(nodes, nodeCapacity * sizeof(struct Node*));
        }

        node = malloc(sizeof(struct Node));
        memset(node, 0, sizeof(struct Node));

        int nameLen = strlen(name);
        node->name_ = malloc(nameLen + 1);
        strcpy(node->name_, name);

        nodes[nodeCount++] = node;
    }

    return node;
}

//------------------------------------------------------------------------------
void AddChild(struct Node* parent, struct Node* child, int count)
{
    if (parent->childrenCount_ == parent->childrenCapacity_)
    {
        parent->childrenCapacity_ = max(parent->childrenCapacity_ * 2, 8);
        parent->children_ = realloc(parent->children_, parent->childrenCapacity_ * sizeof(struct ChildNode));
    }

    struct ChildNode* childNode = &parent->children_[parent->childrenCount_++];
    childNode->node_ = child;
    childNode->count_ = count;
}

//------------------------------------------------------------------------------
void AddParent(struct Node* child, struct Node* parent, int count)
{
    if (child->parentsCount_ == child->parentsCapacity_)
    {
        child->parentsCapacity_ = max(child->parentsCapacity_ * 2, 8);
        child->parents_ = realloc(child->parents_, child->parentsCapacity_ * sizeof(struct ChildNode));
    }

    struct ChildNode* parentNode = &child->parents_[child->parentsCount_++];
    parentNode->node_ = parent;
    parentNode->count_ = count;
    //printf("Add parent %s to %s\n", parentNode->node_->name_, child->name_);
}

//------------------------------------------------------------------------------
void Visit(struct Node* node, struct Node** visited, int* visitedCount)
{
    visited[*visitedCount] = node;
    (*visitedCount)++;

    for (int i = 0; i < node->parentsCount_; ++i)
    {
        //printf("  parent: %s\n", node->parents_[i].node_->name_);
        bool visit = true;
        for (int j = 0; j < *visitedCount; ++j)
        {
            if (node->parents_[i].node_ == visited[j])
            {
                visit = false;
                break;
            }
        }

        if (visit)
        {
            Visit(node->parents_[i].node_, visited, visitedCount);
        }
    }
}

//------------------------------------------------------------------------------
int VisitChildren(struct Node* node, struct VisitedNode* visited, int* visitedCount)
{
    int thisIdx = *visitedCount;
    visited[thisIdx].node_ = node;
    (*visitedCount)++;

    int bags = 1;
    for (int i = 0; i < node->childrenCount_; ++i)
    {
        //printf("  parent: %s\n", node->children_[i].node_->name_);
        bool visit = true;
        for (int j = 0; j < *visitedCount; ++j)
        {
            if (node->children_[i].node_ == visited[j].node_)
            {
                assert(visited[j].count_);
                bags += (node->children_[i].count_ * visited[j].count_);
                visit = false;
                break;
            }
        }

        if (visit)
        {
            bags += (node->children_[i].count_ * VisitChildren(node->children_[i].node_, visited, visitedCount));
        }
    }

    visited[thisIdx].count_ = bags;
    return bags;
}

//------------------------------------------------------------------------------
int main()
{
    nodes = malloc(nodeCapacity * sizeof(struct Node*));

    FILE* file = fopen("input.txt", "r");

    char* line = NULL;
    size_t lineLen = 0;

    const char* separator = " bags contain ";
    int separatorLen = strlen(separator);

    int read = 0;
    while ((read = getline(&line, &lineLen, file)) != -1)
    {
        //light beige bags contain 5 dark green bags, 5 light gray bags, 3 faded indigo bags, 2 vibrant aqua bags.

        char* parentEnd = strstr(line, separator);
        char parentName[64];
        memset(parentName, 0, 64);
        memmove(parentName, line, parentEnd - line);

        struct Node* parent = GetNode(parentName, true);

        char* childrenStart = parentEnd + separatorLen;

        if (strstr(line, "no other bags") != NULL)
            goto skip;

        char* childStr = strtok(childrenStart, ",");
        while (childStr != NULL)
        {
            char* nameEnd = strstr(childStr, " bag");
            *nameEnd = 0;

            char* nameStart = childStr;
            while (*(++nameStart) != ' ');
            *nameStart = 0;
            nameStart++;

            int count = atoi(childStr);

            struct Node* child = GetNode(nameStart, true);
            AddChild(parent, child, count);
            AddParent(child, parent, count);

            childStr = strtok(NULL, ",");
        }

    skip:
        free(line);
        line = NULL;
    }

    struct Node* node = GetNode("shiny gold", false);
    assert(node);

    {
        int visitedCount = 0;
        struct Node** visited = malloc(nodeCount * sizeof(struct Node*));
        free(visited);

        Visit(node, visited, &visitedCount);
        printf("Visited count:\n%d\n", visitedCount - 1);
    }

    {
        int visitedCount = 0;
        struct VisitedNode* visited = malloc(nodeCount * sizeof(struct VisitedNode));
        memset(visited, 0, nodeCount * sizeof(struct VisitedNode));

        int bagsTotal = VisitChildren(node, visited, &visitedCount);
        printf("Total bag count:\n%d\n", bagsTotal - 1);
    }

    return 0;
}