#ifndef MCQUERY_H
#define MCQUERY_H

#include "mcsql.h"

char* mcsql_arc_get_all();

char* mcsql_arc_get_params(int32_t file_no, char* group);

char* mcsql_bookprogram_get_all();

char* mcsql_enum_get_all();

char* mcsql_extaxis_get_all();

char* mcsql_interference_get_all();

char* mcsql_ios_get_all(char* group, char* lang, int32_t auth, int32_t tech);

char* mcsql_metadata_get_all(char* lang);

char* mcsql_params_get_params();

char* mcsql_params_get_valid_param_by_id(char* md_id);

char* mcsql_params_get_valid_param_by_group(char* group);

char* mcsql_ref_get_all();

char* mcsql_toolframe_get_all();

char* mcsql_toolframe_get_by_toolno(int32_t tool_no);

char* mcsql_userframe_get_all();

char* mcsql_userframe_get_by_userno(int32_t user_no);

char* mcsql_zeropoint_get_all();

#endif //MCQUERY_H