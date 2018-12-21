#include "mrj.h"

int init_nv_ram() {
	return init_nvram(NULL);
}

int mrj_init() {
	return resource_init("/rbctrl/mcserver");
}

int mrj_get_remote_mode_status() {
	return (int)GetRemoteAccessState();
}

int mrj_get_encryption_status() {
	return GetEncryptState();
}

int mrj_get_encryption_remain_time() {
	return GetEncryptRemainTime();
}

char* mrj_get_machine_code() {
	return GetEncryptMachCode();
}