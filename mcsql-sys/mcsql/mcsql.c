#include "mcsql.h"

void mcsql_set_db_file(char* dbname) {
	strcpy(db_conn, dbname);
}