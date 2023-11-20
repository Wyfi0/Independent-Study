#include <stdio.h>
#include <stdbool.h>
int num = 5;
int num2 = 5;

int main() {
    printf("Hello World\n");
    printf("yoooo\n");
    printf("%d\n", num);

    if (num != 0) {
        printf("num isnt zero!\n");
    } else {
        printf("num be zero\n");
    }
    (num2 < 100) ? printf("good\n") : printf("bad\n");
    return 0;
}
