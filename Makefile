ifeq ($(strip $(DEVKITPRO)),)
	$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>devkitPro)
endif

DEBUG 		?= 1
FEATURES 	?=

NM 			:= $(DEVKITARM)/bin/arm-none-eabi-nm
SMDHTOOL 	:= $(DEVKITPRO)/tools/bin/smdhtool
3DSXTOOL	:= $(DEVKITPRO)/tools/bin/3dsxtool
BANNERTOOL 	:= $(DEVKITPRO)/tools/bin/bannertool

ifeq ($(DEBUG), 1)
PROFILE 	:= debug
CARGOFLAGS 	:=
SYMBOLS		?= 1
else
PROFILE 	:= release
CARGOFLAGS  := --release
SYMBOLS		?= 0
endif
ifneq ($(FEATURES),)
CARGOFLAGS	+= --features=$(FEATURES)
endif

CARGOFLAGS  += --color=always

TARGET		:= target/armv6k-nintendo-3ds/$(PROFILE)
DIST		:= dist/barista_$(PROFILE)
ROMFS 		:= romfs
RSF			:= app.rsf

CRATE_NAME 	:= barista
PROG_NAME 	:= Barista
PROG_DESC 	:= A launcher for Rhythm Heaven Megamix mods
PROG_AUTHOR := patataofcourse, RHModding
PROG_ICON 	:= icon.png

# Prepend devkitarm bin to PATH, in case there is another arm-none-eabi-gcc installed
export PATH := $(DEVKITARM)/bin:$(PATH)

.PHONY: all clean dist plgldr check doc fmt test update

all: dist

### Main executable ###

ifneq ($(DEBUG), 1)
dist: $(TARGET)/$(CRATE_NAME).cia
endif

dist: $(TARGET)/$(CRATE_NAME).3dsx $(TARGET)/$(CRATE_NAME).elf $(TARGET)/$(CRATE_NAME).smdh
	@mkdir -p $(DIST)
	@cp $(TARGET)/$(CRATE_NAME).elf $(DIST)
	@cp $(TARGET)/$(CRATE_NAME).lst $(DIST)
	@cp $(TARGET)/$(CRATE_NAME).3dsx $(DIST)
ifneq ($(DEBUG), 1)
	@cp $(TARGET)/$(CRATE_NAME).cia $(DIST)
endif
	@cp $(PROG_ICON) $(DIST)/$(notdir $(PROG_ICON))

%.cia: %.elf
	@bannertool makesmdh -s Barista -l Barista -p "patataofcourse, RHModding" -i icon.png -o $(dir $@)icon.icn -r regionfree -f nosavebackups,visible
	@bannertool makebanner -i banner.png -a banner.wav -o $(dir $@)banner.bnr
	@makerom -f cia -o $@ -exefslogo -elf $(basename $@).elf -rsf app.rsf -ver 0 -icon $(dir $@)icon.icn -banner $(dir $@)banner.bnr

%.elf: plgldr
	@cargo 3ds build $(CARGOFLAGS)
	@$(NM) -Cn $@ > $(basename $@).lst
ifeq ($(SYMBOLS), 1)
	@cp $(basename $@).lst romfs
else
	@rm -f romfs/$(basename $(notdir $@)).lst
endif

%.smdh:
	@$(SMDHTOOL) --create "${PROG_NAME}" "${PROG_DESC}" "${PROG_AUTHOR}" "${PROG_ICON}" $(basename $@).smdh

%.3dsx: %.elf %_.smdh
	@$(3DSXTOOL) $(basename $@).elf $(basename $@).3dsx --smdh=$(basename $@)_.smdh --romfs=$(ROMFS)

### Clean

clean:
	@echo "clean ..."
	@rm -rf target
	@rm -rf dist
	@rm -f romfs/barista.lst
	@cd library/plgldr && make clean --no-print-directory

### C libraries ###

plgldr:
	@make --no-print-directory -C library/plgldr


### Useful Cargo stuff ###

doc:
	@cargo 3ds doc --open

fmt:
	@cargo 3ds fmt

test: dist
	@cargo 3ds run

check:
	@cargo 3ds check --features=$(FEATURES)

update:
	@cargo 3ds update