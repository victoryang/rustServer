CC = arm-linux-gnueabihf-gcc
AR = arm-linux-gnueabihf-ar
SHARED = -shared
RCS = rcs

INCLUDE = /root/mcserver/include/

CPPFLAGS += -I$(INCLUDE) -fPIC -Wall

.PHONY: all

all: libmcsql.a

libmcsql.a: mcquery.o mcmanager.o mcsql.o mcsqlmapper.o
	$(AR) $(RCS) $@ $^

test: test_main.c mcsql.o mcquery.o mcmanager.o
	$(CC) $(CPPFLAGS) -o $@ $^ -lsqlitedb

mcsql.o: mcsql.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lsqlitedb

mcquery.o: mcquery.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lsqlitedb

mcmanager.o: mcmanager.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lsqlitedb

mcsqlmapper.o: mcsqlmapper.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lsqlitedb

clean:
	rm -f *.o libmcsql.a