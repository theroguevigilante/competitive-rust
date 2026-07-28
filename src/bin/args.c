#include <stdio.h>

int main(int argc, char** args){
    for(int i = 0; i < argc; ++i){
        printf("%s ", args[i]);
    }
    printf("\n");
}
