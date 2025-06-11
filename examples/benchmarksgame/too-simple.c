/* The Computer Language Benchmarks Game
   https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
*/

#include<stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
    // mtime side-effect   
    FILE* f = fopen("./one", "w"); fclose(f);
  
    long n = atol(argv[1]);
    double sum = 0.0;
    double flip = -1.0;
    for (long i = 1; i <= n; i++) {    
        flip *= -1.0;        
        sum += flip / (2*i - 1);               
    }                        
    printf("%.9f\n", sum*4.0);
    
    // mtime side-effect
    f = fopen("./two", "w"); fclose(f); 
    
    return 0;      
}