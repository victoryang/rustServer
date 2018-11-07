#ifndef MRJSYSVARS_H
#define MRJSYSVARS_H

#include "define.h"
#define GetSysVarcRobB 0
#define GetSysVariRobI 1
#define GetSysVardRobD 2
#define GetSysVardRobP 3
#define GetSysVardRobV 4

typedef void (*getSysVarFunc)(cJSON*, int, int);

char* mrj_get_sysvar(int datatype, int start, int end);

#endif //MRJSYSVARS_H