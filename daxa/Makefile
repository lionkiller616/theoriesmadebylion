# Makefile for Daxa Assembly Project
# Target: Linux x86-64

NASM=nasm
LD=ld
RM=rm -f

# NASM flags: -f elf64 (output format), -g (debug symbols), -F dwarf (dwarf debug format)
NASMFLAGS=-f elf64 -g -F dwarf -w+all -w-error=numval-cast

# LD flags: Link against libc for printf, etc.
# -I/lib64/ld-linux-x86-64.so.2 specifies the interpreter
# If building completely static without libc, flags would be different (and printf unavailable)
LDFLAGS= -dynamic-linker /lib64/ld-linux-x86-64.so.2 -lc -m elf_x86_64

# Source directories
SRCDIRS := boot core compression schema format utils cli
# Find all .asm files in source directories
SOURCES := $(foreach dir,$(SRCDIRS),$(wildcard $(dir)/*.asm))
OBJECTS := $(SOURCES:%.asm=build/obj/%.o)

# Output executable
TARGET_EXE=build/output/daxa

# Create object file directories
OBJDIRS := $(patsubst %,build/obj/%,$(SRCDIRS))

all: $(TARGET_EXE)

$(TARGET_EXE): $(OBJECTS)
	@echo "LD  $@"
	@$(LD) $(LDFLAGS) -o $@ $^

build/obj/%.o: %.asm | $(OBJDIRS)
	@echo "NASM $<"
	@$(NASM) $(NASMFLAGS) -o $@ $< -l $(@:.o=.lst)

# Rule to create .d (dependency) files
build/obj/%.d: %.asm | $(OBJDIRS)
	@echo "DEP $<"
	@$(NASM) -M $(NASMFLAGS) $< > $@

# Create object directories
$(OBJDIRS):
	@mkdir -p $@

# Include dependency files if they exist
# -include $(OBJECTS:.o=.d)

clean:
	@echo "CLEAN"
	@$(RM) $(TARGET_EXE) $(OBJECTS) $(OBJECTS:.o=.lst) $(OBJECTS:.o=.d)
	@$(RM) -r build/obj/* 2>/dev/null || true # avoid error if dirs don't exist
	@$(RM) -r build/output/* 2>/dev/null || true

# Phony targets
.PHONY: all clean

# Message
info:
	@echo "Daxa Assembly Project"
	@echo "Targets: all, clean"
	@echo "Source files:"
	@echo "$(SOURCES)"
	@echo "Object files:"
	@echo "$(OBJECTS)"
	@echo "Executable: $(TARGET_EXE)"

run_sample_info: $(TARGET_EXE)
	@echo "\n--- Running Daxa Info (Conceptual) ---"
	./$(TARGET_EXE) info examples/sample.daxa