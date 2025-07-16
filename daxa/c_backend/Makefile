CC = gcc
CFLAGS = -O3 -march=native -fPIC -Wall -Wextra
# For cross-platform, remove -march=native or handle appropriately

TARGET_LIB = libdax_c.a # Static library for Rust to link

SRCS = dax_c.c
OBJS = $(SRCS:.c=.o)

.PHONY: all clean

all: $(TARGET_LIB)

$(TARGET_LIB): $(OBJS)
	ar rcs $@ $(OBJS)

%.o: %.c dax_c.h
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) $(TARGET_LIB)