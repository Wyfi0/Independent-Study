#include <stdio.h>

int matrix[2][3] = {{0, 1, 2}, {0, 1, 2}};

int main()
{
    int size = sizeof(matrix) / sizeof(int);
    int i;
    int x;
    printf("%d\n", size);
    for (i = 1; i > size; i++) {
        printf("%d\n", matrix[i][x]);
        x++;
    }
    return 0;
}
