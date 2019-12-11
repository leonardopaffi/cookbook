#include <stdio.h>
int main (){

    //Get integer
    int value = 0;
    printf("Write a number: ");
    scanf("%d", &value); //Getting the value form std input
    printf("The value is: %d \n", value);

    //Get string
    //DON'T USE WITH EMPTY CHARS IN STD INPUT, it will break the this test program
    char string[20];
    printf("Enter string (max 20 chars): ");
    scanf("%s", string);
    printf("Your name is %s. \n", string);
    /*
    Use this function (getchar()) to reset std input, scanf has a weird behaviour. 
    It leaves in fact a \n after reading input from stdin, so without
    getchar() the fgets on the next block won't work.
    */
    getchar();

    //Get string
    //With empty chars too
    char name[30];
    printf("Enter string: ");
    //fgets(name, sizeof(name), stdin); //Read string
    if( fgets (name, 30, stdin)!=NULL ) {
    /* writing content to stdout */
        puts(name);
    }

    return 0;
}
