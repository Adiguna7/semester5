#include <stdio.h>

int bitAND(int x, int y)
{
    int result;

    result = ~((~x) | (~y));

    return result;
   
}

int main()
{
    int x, y;
    
    scanf("%d", &x);
    scanf("%d", &y);
    
    printf("%d", bitAND(x, y));

    return 0;
}