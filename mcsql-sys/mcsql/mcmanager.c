#include "mcmanager.h"

int execute(struct db_manager* mgr) {
	set_cmds(mgr, ELIBOT_BAK_BACKUP_PARAMS);
	void* output = malloc(sizeof(char) * BUF_SIZE);
	return mgr->execute(mgr, output);
}

int mcsql_manager_backup_db(char* db_dir) {
	db_manager* mgr = (db_manager*)malloc(sizeof(db_manager));
    if(mgr == NULL) {
        return ERREMPTYMANAGER;
    }
    
    new_backup_db_manager(db_conn, db_dir, mgr);
    return execute(mgr);
}

int mcsql_manager_restore_db(char* db_dir, char* db_bak_name, char force) {
	db_manager* mgr = (db_manager*)malloc(sizeof(db_manager));
    if(mgr == NULL) {
        return ERREMPTYMANAGER;
    }

    new_restore_db_manager(db_conn, db_dir, db_bak_name, mgr, force);
    return execute(mgr);
}

int mcsql_manager_upgrade_db(char* db_dir, char* upgrade_pkg) {
	db_manager* mgr = (db_manager*)malloc(sizeof(db_manager));
    if(mgr == NULL) {
        return ERREMPTYMANAGER;
    }

    new_upgrade_db_manager(db_conn, db_dir, upgrade_pkg, mgr);
    return execute(mgr);
}