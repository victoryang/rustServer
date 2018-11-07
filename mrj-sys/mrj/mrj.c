#include "mrj.h"

int init_nv_ram() {
	return init_nvram(NULL);
}

int mrj_init() {
	return resource_init("/rbctrl/mcserver");
}