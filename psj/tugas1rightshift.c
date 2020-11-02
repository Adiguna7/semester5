#include <stdio.h>

int logicalShift(int x, int n)
{
    int result = x;
    for(int i = 0; i < n; i++){
        result = result / 2;
    }
    return result;
}

int main()
{
    int x, n;
    
    scanf("%d", &x);
    scanf("%d", &n);
    
    printf("%d", logicalShift(x, n));

    return 0;
}