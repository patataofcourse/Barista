ifeq ($(strip $(DEVKITPRO)),)
	$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>devkitPro)
endif

DEBUG 		?= 1
FEATURES 	?=
CARGO3DS	?= 0

STD			:= $(shell rustc --print sysroot)/rustlib/armv6k-nintendo-3ds
NM 			:= $(DEVKITARM)/bin/arm-none-eabi-nm
SMDHTOOL 	:= $(DEVKITPRO)/tools/bin/smdhtool
3DSXTOOL	:= $(DEVKITPRO)/tools/bin/3dsxtool
BANNERTOOL 	:= $(DEVKITPRO)/tools/bin/bannertool
CITRA		:= $(shell which citra 2> /dev/null || which citra-qt 2> /dev/null || which citra-appimage 2> /dev/null || true)

CARGOFLAGS  := --color=always

ifeq ($(CARGO3DS),0)

CARGO 		:= cargo
#CARGOFLAGS  += --target=armv6k-nintendo-3ds

# build STD if it doesn't exist
ifeq ($(wildcard $(STD)),)
CARGOFLAGS += -Zbuild-std
endif

else

CARGO		:= cargo 3ds

endif

# Assume flatpak
ifeq ($(CITRA),)
CITRA 		:= flatpak run org.citra_emu.citra
endif

ifeq ($(DEBUG), 1)
PROFILE 	:= debug
SYMBOLS		?= 1
CIA			?= 0
else
PROFILE 	:= release
CARGOFLAGS  += --release
SYMBOLS		?= 0
CIA			?= 1
endif

ifneq ($(FEATURES),)
CARGOFLAGS	+= --features="$(FEATURES)"
endif

BUILD		:= target/armv6k-nintendo-3ds/$(PROFILE)
DIST		:= dist/barista_$(PROFILE)
ROMFS 		:= romfs
RSF			:= app.rsf

CRATE_NAME 	:= barista
PROG_NAME 	:= Barista
PROG_DESC 	:= A launcher for Rhythm Heaven Megamix mods
PROG_AUTHOR := patataofcourse, RHModding
PROG_ICON 	:= icon.png

ifeq ($(DEBUG),0)
export RUSTFLAGS = -L$(DEVKITPRO)/libctru/lib -lctru
else
export RUSTFLAGS = -L$(DEVKITPRO)/libctru/lib -lctrud
endif

.PHONY: all clean dist plgldr check doc fmt fix test update re force
.PRECIOUS: $(BUILD)/$(CRATE_NAME).elf 

all: dist

### Main executable ###

ifneq ($(CIA), 0)
dist: $(BUILD)/$(CRATE_NAME).cia
endif

dist: $(BUILD)/$(CRATE_NAME).3dsx
	@mkdir -p $(DIST)
	@cp $(BUILD)/$(CRATE_NAME).elf $(DIST)
	@cp $(BUILD)/$(CRATE_NAME).lst $(DIST)
	@cp $(BUILD)/$(CRATE_NAME).3dsx $(DIST)
ifneq ($(CIA), 0)
	@cp $(BUILD)/$(CRATE_NAME).cia $(DIST)
else
	@rm -f $(DIST)/$(CRATE_NAME).cia
endif
	@cp $(PROG_ICON) $(DIST)/$(notdir $(PROG_ICON))

%.cia: %.elf
	@bannertool makesmdh -s $(PROG_NAME) -l $(PROG_NAME) -p $(PROG_AUTHOR) -i icon.png -o $(dir $@)icon.icn -r regionfree -f nosavebackups,visible
	@bannertool makebanner -i banner.png -a banner.wav -o $(dir $@)banner.bnr
	@makerom -f cia -o $@ -exefslogo -elf $(basename $@).elf -rsf app.rsf -ver 0 -icon $(dir $@)icon.icn -banner $(dir $@)banner.bnr

%.elf: plgldr
	@$(CARGO) build $(CARGOFLAGS)
	@$(NM) -Cn $@ > $(basename $@).lst
ifeq ($(SYMBOLS), 1)
	@cp $(basename $@).lst romfs
else
	@rm -f romfs/$(basename $(notdir $@)).lst
endif

%.3dsx: %.elf
	@$(SMDHTOOL) --create "${PROG_NAME}" "${PROG_DESC}" "${PROG_AUTHOR}" "${PROG_ICON}" $(basename $@)_.smdh
	@$(3DSXTOOL) $(basename $@).elf $(basename $@).3dsx --smdh=$(basename $@)_.smdh --romfs=$(ROMFS)

### Clean ###

clean:
	@cargo clean
	@rm -rf dist
	@rm -f romfs/barista.lst
	@cd library/plgldr && make clean --no-print-directory

re: clean all

# Like make re, but only forces rebuild of the binary
force:
	@rm -rf target/armv6k-nintendo-3ds/debug/build/barista-*
	@make all

### C libraries ###

plgldr:
	@make --no-print-directory -C library/plgldr


### Useful Cargo stuff ###

doc:
	@$(CARGO) doc --open $(CARGOFLAGS)

fmt:
	@cargo fmt --all

test: dist
	@$(CITRA) $(DIST)/$(CRATE_NAME).3dsx

check:
	@$(CARGO) clippy $(CARGOFLAGS)

update:
	@cargo update