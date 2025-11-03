#include <stdio.h>

int main(void)
{
    int a = 9999;

    printf("%zu\n", sizeof a);
    printf("%zu\n", sizeof(2 + 7));
    printf("%zu\n", sizeof 3.14);
    printf("%zu\n", sizeof "asdasdqfqwwerwerwerrqerfqwefqefqgesadf");
    printf("%zu\n", sizeof 'a');
}