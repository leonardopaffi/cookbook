#include <stdio.h>
int main(int argc, char *argv[]){
    switch(argc)
    {
    case 1:
        printf("Program name: %s \n", argv[0]);
        break;
    case 2:
        printf("Arg: %s \n", argv[1]);    
        break;       
    case 3:
        printf("Arg: %s %s \n", argv[1], argv[2]);
        break; 
    }
    return 0;
}
