#ifndef MRJ_H
#define MRJ_H

#include "define.h"

int init_nv_ram();
int mrj_init();

int mrj_get_remote_mode_status();

int mrj_get_encryption_status();

int mrj_get_encryption_remain_time();

char* mrj_get_machine_code();

#endif //MRJ_H