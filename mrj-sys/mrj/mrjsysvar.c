#include "mrjsysvar.h"

void get_cRobB_with_range(cJSON* root, int start, int end) {
	unsigned int *base;
	int count;
	if (start > B_COUNT) {
		return;
	}
	if (end > B_COUNT) {
		end = B_COUNT;
	}
	base = SHARE_RES(sysvar).cRobB + start;
	count = end - start;
	cJSON_AddItemToObject(root, "cRobB", cJSON_CreateIntArray((const int *)base, count));
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(B_COUNT));
}

void get_iRobI_with_range(cJSON* root, int start, int end) {
	int i=0;
	cJSON *array;
	if (start > I_COUNT) {
		return;
	}
	if (end > I_COUNT) {
		end = I_COUNT;
	}

	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateNumber(SHARE_RES(sysvar).iRobI[i]));
	}
	cJSON_AddItemToObject(root, "iRobI", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(I_COUNT));
}

void get_dRobD_with_range(cJSON* root, int start, int end) {
	double *base;
	int count;
	if (start > D_COUNT) {
		return;
	}
	if (end > D_COUNT) {
		end = D_COUNT;
	}
	base = SHARE_RES(sysvar).dRobD + start;
	count = end - start;
	cJSON_AddItemToObject(root, "dRobD", cJSON_CreateDoubleArray(base, count));
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(D_COUNT));
}

void get_dRobP_with_range(cJSON* root, int start, int end) {
	cJSON *array;
	int i;
	if (start > P_COUNT) {
		return;
	}
	if (end > P_COUNT) {
		end = P_COUNT;
	}
	
	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateDoubleArray(SHARE_RES(sysvar).dRobP[i], AXIS_COUNT));
	}
	cJSON_AddItemToObject(root, "dRobP", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(P_COUNT));
}

void get_dRobV_with_range(cJSON* root, int start, int end) {
	cJSON *array;
	int i;
	if (start > V_COUNT) {
		return;
	}
	if (end > V_COUNT) {
		end = V_COUNT;
	}

	array = cJSON_CreateArray();
	for (i=start; i<end; i++) {
		cJSON_AddItemToArray(array, cJSON_CreateDoubleArray(SHARE_RES(sysvar).dRobV[i], VSub_COUNT));
	}
	cJSON_AddItemToObject(root, "dRobV", array);
	cJSON_AddItemToObject(root, "TotalSize", cJSON_CreateNumber(V_COUNT));
}

getSysVarFunc sysVarTable[] = {
	&get_cRobB_with_range,
	&get_iRobI_with_range,
	&get_dRobD_with_range,
	&get_dRobP_with_range,
	&get_dRobV_with_range,
};

cJSON* get_sysvar_with_range(int datatype, int start, int end) {
	getSysVarFunc gf;
	cJSON* root=NULL;
	switch (datatype) {
		case GetSysVarcRobB:
		case GetSysVariRobI:
		case GetSysVardRobD:
		case GetSysVardRobP:
		case GetSysVardRobV:
			root = cJSON_CreateObject();
			gf = sysVarTable[datatype];
			gf(root, start, end);
			break;
		default:
			return NULL;
	}

	return root;
}

char* mrj_get_sysvar(int datatype, int start, int end) {
	cJSON* root;
	char* ret;
	root = get_sysvar_with_range(datatype, start, end);
	ret = cJSON_PrintUnformatted(root);
	cJSON_Delete(root);
	return ret;
}