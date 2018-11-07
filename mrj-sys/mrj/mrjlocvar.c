#include "mrjlocvar.h"

void get_cRobLB_with_range(cJSON* root, int num, int start, int end) {
	unsigned int *base;
	int count;
	if (start >= LB_COUNT || num >= CALL_NEST_NUM) {
		return;
	}
	if (end > LB_COUNT) {
		end = LB_COUNT;
	}
	base = SHARE_RES(locvar)[num].cRobLB + start;
	count = end - start;
	cJSON_AddItemToObject(root, "cRobLB", cJSON_CreateIntArray((const int *)base, count));
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(LB_COUNT));
	cJSON_AddItemToObject(root, "NestNum", cJSON_CreateNumber(CALL_NEST_NUM));
}

void get_iRobLI_with_range(cJSON* root, int num, int start, int end) {
	int i=0;
	cJSON *array;
	if (start >= LI_COUNT || num >= CALL_NEST_NUM) {
		return;
	}
	if (end > LI_COUNT) {
		end = LI_COUNT;
	}

	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateNumber(SHARE_RES(locvar)[num].iRobLI[i]));
	}
	cJSON_AddItemToObject(root, "iRobLI", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(LI_COUNT));
	cJSON_AddItemToObject(root, "NestNum", cJSON_CreateNumber(CALL_NEST_NUM));
}

void get_dRobLD_with_range(cJSON* root, int num, int start, int end) {
	double *base;
	int count;
	if (start >= LD_COUNT || num >= CALL_NEST_NUM) {
		return;
	}
	if (end > LD_COUNT) {
		end = LD_COUNT;
	}
	base = SHARE_RES(locvar)[num].dRobLD + start;
	count = end - start;
	cJSON_AddItemToObject(root, "dRobLD", cJSON_CreateDoubleArray(base, count));
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(LD_COUNT));
	cJSON_AddItemToObject(root, "NestNum", cJSON_CreateNumber(CALL_NEST_NUM));
}

void get_dRobLP_with_range(cJSON* root, int num, int start, int end) {
	cJSON *array;
	int i;
	if (start > LP_COUNT || num >= CALL_NEST_NUM) {
		return;
	}
	if (end > LP_COUNT) {
		end = LP_COUNT;
	}
	
	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateDoubleArray(SHARE_RES(locvar)[num].dRobLP[i], AXIS_COUNT));
	}
	cJSON_AddItemToObject(root, "dRobLP", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(LP_COUNT));
	cJSON_AddItemToObject(root, "NestNum", cJSON_CreateNumber(CALL_NEST_NUM));
}

void get_dRobLV_with_range(cJSON* root, int num, int start, int end) {
	cJSON *array;
	int i;
	if (start > LV_COUNT || num >= CALL_NEST_NUM) {
		return;
	}
	if (end > LV_COUNT) {
		end = LV_COUNT;
	}

	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateDoubleArray(SHARE_RES(locvar)[num].dRobLV[i], VSub_COUNT));
	}
	cJSON_AddItemToObject(root, "dRobLV", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(LV_COUNT));
	cJSON_AddItemToObject(root, "NestNum", cJSON_CreateNumber(CALL_NEST_NUM));
}

getLocVarFunc locVarTable[] = {
	&get_cRobLB_with_range,
	&get_iRobLI_with_range,
	&get_dRobLD_with_range,
	&get_dRobLP_with_range,
	&get_dRobLV_with_range,
};

cJSON* get_locvar_with_range(int datatype, int number, int start, int end) {
	getLocVarFunc gf;
	cJSON* root=NULL;
	switch (datatype) {
		case GetLocVarcRobLB:
		case GetLocVariRobLI:
		case GetLocVardRobLD:
		case GetLocVardRobLP:
		case GetLocVardRobLV:
			root = cJSON_CreateObject();
			gf = locVarTable[datatype];
			gf(root, number, start, end);
			break;
		default:
			return NULL;
	}

	return root;
}

char* mrj_get_locvar(int datatype, int number, int start, int end) {
	cJSON* root;
	char* ret;
	root = get_locvar_with_range(datatype, number, start, end);
	ret = cJSON_PrintUnformatted(root);
	cJSON_Delete(root);
	return ret;
}