#include "mcsql.h"

char db_conn[128];

void mcsql_set_db_file(char* dbname) {
	strcpy(db_conn, dbname);
}