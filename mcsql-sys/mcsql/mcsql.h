#ifndef MCSQL_H
#define MCSQL_H

#include "mcquery.h"

extern char db_conn[128];

void mcsql_set_db_file(char* dbname);

#endif //MCSQL_H