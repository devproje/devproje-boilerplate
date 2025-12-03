CC = bun
OUTDIR = ./dist
TARGET = ../public

.PHONY: all init build copyto clean

all: build copyto

init:
	@$(CC) install

build:
	@$(CC) run build

copyto:
	@echo "Copying static frontend dist..."
	@cp -r $(OUTDIR) $(TARGET)

clean:
	@rm -rf $(OUTDIR)

