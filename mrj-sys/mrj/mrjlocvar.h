#ifndef MRJLOCVAR_H
#define MRJLOCVAR_H

#include "define.h"
#define GetLocVarcRobLB 0
#define GetLocVariRobLI 1
#define GetLocVardRobLD 2
#define GetLocVardRobLP 3
#define GetLocVardRobLV 4

typedef void (*getLocVarFunc)(cJSON*, int, int, int);

char* mrj_get_locvar(int datatype, int number, int start, int end);

#endif //MRJLOCVAR_H