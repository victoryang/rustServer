CC = arm-linux-gnueabihf-gcc
AR = arm-linux-gnueabihf-ar
SHARED = -shared
RCS = rcs

INCLUDE = /root/mcserver/include/
LIB = /usr/arm-linux-gnueabihf/lib/

CPPFLAGS += -I$(INCLUDE) -fPIC -Wall
LDFLAGS += -L$(LIB)

.PHONY: all

all: libmcsql.a

libmcsql.a: mcquery.o mcmanager.o mcsql.o mcsqlmapper.o
	$(AR) $(RCS) $@ $^

test: test_main.c mcsql.o mcquery.o mcmanager.o
	$(CC) $(CPPFLAGS) $(LDFLAGS) -o $@ $^ -lsqlitedb

mcsql.o: mcsql.c
	$(CC) $(CPPFLAGS) $(LDFLAGS) -o $@ -c $^ -lsqlitedb

mcquery.o: mcquery.c
	$(CC) $(CPPFLAGS) $(LDFLAGS) -o $@ -c $^ -lsqlitedb

mcmanager.o: mcmanager.c
	$(CC) $(CPPFLAGS) $(LDFLAGS) -o $@ -c $^ -lsqlitedb

mcsqlmapper.o: mcsqlmapper.c
	$(CC) $(CPPFLAGS) $(LDFLAGS) -o $@ -c $^ -lsqlitedb

clean:
	rm -f *.o libmcsql.a