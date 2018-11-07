#ifndef MCMANAGER_H
#define MCMANAGER_H

#include "define.h"
#include "mcsql.h"

#define BUF_SIZE 255
#define ERRNONE DB_OK
#define ERREMPTYMANAGER -1

int mcsql_manager_backup_db(char* db_dir);

int mcsql_manager_restore_db(char* db_dir, char* db_bak_name, char force);

int mcsql_manager_upgrade_db(char* db_dir, char* upgrade_pkg);

#endif //MCMANAGER_H