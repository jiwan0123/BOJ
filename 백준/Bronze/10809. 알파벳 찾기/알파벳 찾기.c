#include <stdio.h>
#include <string.h>

int main() {
    char s[100];
    int alphabet[26], len, index;
    
    for (int i = 0; i < 26; i++) {
        alphabet[i] = -1;
    }
    
    scanf("%s", s);
    
    len = strlen(s);
    for (int i = 0; i < len; i++) {
        index = s[i] - 'a';
        if (alphabet[index] == -1) {
            alphabet[index] = i;
        }
        else {
            continue;
        }
    }
    
    for (int i = 0; i < 26; i++) {
        printf("%d ", alphabet[i]);
    }
    
    return 0;
}