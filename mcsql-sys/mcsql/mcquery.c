#include "mcquery.h"

static char* mcsql_db_query(db_query_req* req) {
    cJSON* root = db_query(req);

    char *ret = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);

    return ret;
}

char* mcsql_arc_get_all() {
    const char *q_id = ELIBOT_ARC_GET_ALL_PARAMS;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_arc_get_params(int32_t file_no, char* group) {
    const char *q_id = ELIBOT_ARC_GET_PARAMS;

    sql_parameter sql_params[] = {
            {name:"file_no", value:{ int_value: file_no}, type:DB_TYPE_INT32},
            {name:"group", value:{ string_value: group}, type:DB_TYPE_TEXT},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 2
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_bookprogram_get_all() {
    const char *q_id = ELIBOT_BOOKPROGRAM_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_enum_get_all() {
    const char *q_id = ELIBOT_ENUM_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_extaxis_get_all() {
    const char *q_id = ELIBOT_EXTAXIS_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_interference_get_all() {
    const char *q_id = ELIBOT_INTERFERENCE_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_ios_get_all(char* group, char* lang, int32_t auth, int32_t tech) {
    const char *q_id = ELIBOT_IO_GET_VALID_IOS_BY_GROUP;

    sql_parameter sql_params[] = {
            {name:"group", value:{ string_value: group}, type:DB_TYPE_TEXT},
            {name:"lang", value:{ string_value: lang}, type:DB_TYPE_TEXT},
            {name:"auth", value:{ int_value: auth}, type:DB_TYPE_INT32},
            {name:"tech", value:{ int_value: tech}, type:DB_TYPE_INT32},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 4
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_metadata_get_all(char* lang) {
    const char *q_id = ELIBOT_METADATA_GET_ALL;

    sql_parameter sql_params[] = {
            {name:"lang", value:{ string_value: lang}, type:DB_TYPE_TEXT},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 1
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_operation_record_get_all(int32_t created_time, int32_t start, int32_t page_size) {
    const char *q_id = ELIBOT_RECORD_GET_ALL;

    sql_parameter sql_params[] = {
            {name:"created_time", value:{ int_value: created_time}, type:DB_TYPE_INT32},
            {name:"start", value:{int_value: start}, type:DB_TYPE_INT32},
            {name:"pageSize", value:{int_value: page_size}, type: DB_TYPE_INT32},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 3
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_params_get_params() {
    const char *q_id = ELIBOT_PARAMS_GET_PARAMS;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_params_get_valid_param_by_id(char* md_id) {
    const char *q_id = ELIBOT_PARAMS_GET_VALID_PARAM_BY_ID;

    sql_parameter sql_params[] = {
            {name:"md_id", value:{ string_value: md_id}, type:DB_TYPE_TEXT},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 1
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_params_get_valid_param_by_group(char* group) {
    const char *q_id = ELIBOT_PARAMS_GET_VALID_PARAMS_BY_GROUP;

    sql_parameter sql_params[] = {
            {name:"group", value:{ string_value: group}, type:DB_TYPE_TEXT},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 1
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_ref_get_all() {
    const char *q_id = ELIBOT_REF_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_QUERY
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_toolframe_get_all() {
    const char *q_id = ELIBOT_COMMON_GET_ALL_TOOLFRAMES;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_toolframe_get_by_toolno(int32_t tool_no) {
    const char *q_id = ELIBOT_COMMON_GET_TOOLFRAMES;

    sql_parameter sql_params[] = {
            {name:"tool_no", value:{ int_value: tool_no}, type:DB_TYPE_INT32},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 1
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_userframe_get_all() {
    const char *q_id = ELIBOT_USER_FRAME_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_userframe_get_by_userno(int32_t user_no) {
    const char *q_id = ELIBOT_USER_FRAME_GET_BY_USER_NO;

    sql_parameter sql_params[] = {
            {name:"user_no", value:{ int_value: user_no}, type:DB_TYPE_INT32},
    };

    db_query_req_parameter q_params = {
            params: sql_params,
            param_size: 1
    };

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_CUSTOM_OBJECT
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:&q_params,
            page:NULL,
    };

    return mcsql_db_query(&req);
}

char* mcsql_zeropoint_get_all() {
    const char *q_id = ELIBOT_ZEROPOINT_GET_ALL;

    db_query_req_option opt = {
            type_handle_mode:DB_QUERY_MODE_STANDARD
    };

    db_query_req req = {
            query_id:(char *)q_id,
            conn_str:db_conn,
            option:&opt,
            parameter:NULL,
            page:NULL,
    };

    return mcsql_db_query(&req);
}