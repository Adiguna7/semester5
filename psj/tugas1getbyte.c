#include <stdio.h>

int getByte(int x, int n)
{
    int result = (x >> (8 * (n - 1))) & 255;

    return result;
    
}

int main()
{
    int x, n;
    
    scanf("%d", &x);
    scanf("%d", &n);
    
    printf("%d", getByte(x, n));

    return 0;
}



