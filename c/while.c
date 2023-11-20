#include <stdio.h>

int i;
int array[] = {10, 15, 20, 25, 30, 35, 40, 45, 50};

int main()
{
    while (i < 5) {
        printf("%d\n", i);
        i++;
    }
    printf("\n");
    for (i = 0; i < 10; i++) {
        printf("%d\n", array[i]);
    }

    return 0;
}
