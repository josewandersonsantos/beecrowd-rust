#include <stdio.h>
int main(void)
{
    char out[50]= "AMO FAZER EXERCICIO NO URI";
    printf("<%s>\n", out);
    printf("<%30s>\n", out);
    printf("<%.20s>\n", out);
    printf("<%-20s>\n", out);
    printf("<%-30s>\n", out);
    printf("<%.30s>\n", out);
    printf("<%30.20s>\n", out);
    printf("<%-30.20s>\n", out);

    return 0;
}