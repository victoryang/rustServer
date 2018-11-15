CC = arm-linux-gnueabihf-gcc
AR = arm-linux-gnueabihf-ar
SHARED = -shared
RCS = rcs
INCLUDE = /root/mcserver/include/

CPPFLAGS += -I$(INCLUDE) -fPIC -Wall

.PHONY: all

all: libmrj.a

libmrj.a: mrj.o mrjnv.o mrjresource.o mrjplc.o mrjsysvar.o mrjlocvar.o cJSON.o
	$(AR) $(RCS) $@ $^

test: main.c mrj.o mcnv.o mcresource.o mcvars.o cJSON.o
	$(CC) $(CPPFLAGS) -o $@ $^ -lm -lrobresource

mrj.o: mrj.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

mrjnv.o: mrjnv.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

mrjresource.o: mrjresource.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

mrjplc.o: mrjplc.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

mrjsysvar.o: mrjsysvar.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

mrjlocvar.o: mrjlocvar.c
	$(CC) $(CPPFLAGS) -o $@ -c $^ -lm -lrobresource

cJSON.o: cJSON.c
	$(CC) -fPIC -Wall -o $@ -c $^

clean:
	rm -f *.o libmrj.a
