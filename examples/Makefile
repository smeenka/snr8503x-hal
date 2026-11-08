# SNR8503M firmware Makefile
# Target: Cortex-M0 (LKS32MC03x via SNR8503M re-brand)

# -----------------------------------------------------------------------------
# Build directories
# -----------------------------------------------------------------------------
BUILD_DIR := target
BINARY   := blinky

# -----------------------------------------------------------------------------
# Flashing (OpenOCD)
# -----------------------------------------------------------------------------
# The SNR8503x (LKS32MC03x) internal flash isn't supported by mainline
# OpenOCD. Build the fork with the vendor flash driver first:
#   cd /your/data/location
#   git clone https://github.com/andrewthecat/openocd-snr8503 
#   cd openocd-snr8503 && ./bootstrap && ./configure && make
# Running straight out of the build tree (no `make install`) below, so its
# .cfg files live in the source tree's tcl/, not an installed share dir —
# OPENOCD_SCRIPTS must point there via -s or OpenOCD can't find
# interface/*.cfg or target/*.cfg. If you `sudo make install` instead, set
# OPENOCD=openocd and OPENOCD_SCRIPTS= (empty) to use the installed scripts.

OPENOCD_DIR       :=/data/tools/jtag/openocd-snr8503
OPENOCD           := $(OPENOCD_DIR)/src/openocd
OPENOCD_SCRIPTS   := $(OPENOCD_DIR)/tcl
OPENOCD_INTERFACE := interface/cmsis-dap.cfg
OPENOCD_TARGET    := target/snr8503x.cfg
OPENOCD_FLAGS     := $(if $(OPENOCD_SCRIPTS),-s $(OPENOCD_SCRIPTS)) \
                     -f $(OPENOCD_INTERFACE) -f $(OPENOCD_TARGET)

# Flash size matches the FLASHSIZE default in snr8503x.cfg (32 KB).
OPENOCD_FLASHSIZE := 0x8000
READBACK_BIN      := $(TARGET)_readback.bin
BUILD_DIR         := target

# -----------------------------------------------------------------------------
# Targets
# -----------------------------------------------------------------------------
.PHONY: restore verify erase backup gdbserver help dirs

$(BUILD_DIR)/$(BINARY).hex: src/bin/$(BINARY).rs
	cargo build   --release   --bin $(BINARY)
	cargo objcopy --release  --bin $(BINARY) -- -O ihex $(BUILD_DIR)/$(BINARY).hex

FORCE: ;

flash: $(BUILD_DIR)/$(BINARY).hex
	$(OPENOCD) $(OPENOCD_FLAGS) \
	  -c "program $(BUILD_DIR)/$(BINARY).hex verify reset  exit"

restore: $(READBACK_BIN)
	$(OPENOCD) $(OPENOCD_FLAGS) \
	  -c "program $(READBACK_BIN) verify reset exit"

# Compare the chip's current flash contents against $(TARGET).hex without
# programming anything.
verify: $(BUILD_DIR)/$(BINARY).hex
	$(OPENOCD) $(OPENOCD_FLAGS) \
	  -c "init" -c "reset halt" \
	  -c "flash verify_image $(BUILD_DIR)/$(BINARY).hex" \
	  -c "exit"

# Erase the whole chip (32 KB) without programming.
erase:
	@echo "Clock must be on 5 Khz"
	$(OPENOCD) $(OPENOCD_FLAGS) \
	  -c "init" -c "reset halt" -c "snr8503x mass_erase 0" -c "reset" 

# Dump the IC's existing firmware to $(READBACK_BIN) (e.g. to back up a stock
# dump before first re-flash). Size defaults to the full 32 KB flash.
backup:
	@mkdir -p $(dir $(READBACK_BIN))
	$(OPENOCD) $(OPENOCD_FLAGS) \
	  -c "init" -c "reset halt" \
	  -c "dump_image $(READBACK_BIN) 0x0 $(OPENOCD_FLASHSIZE)" \
	  -c "reset" -c "exit"
	@echo "Firmware read back to $(READBACK_BIN)"

# Drop into an interactive OpenOCD session (telnet on :4444, GDB on :3333).
gdbserver:
	$(OPENOCD) $(OPENOCD_FLAGS)

help:
	@echo "make flash      - flash $(BINARY).hex via OpenOCD + SWD (OPENOCD_INTERFACE=$(OPENOCD_INTERFACE))"
	@echo "make backup     - dump the IC's existing firmware to \$$(READBACK_BIN)"
	@echo "make restore    - restore the IC's backup firmware from \$$(READBACK_BIN)"
	@echo "make verify     - compare the IC's current flash against $(TARGET).hex"
	@echo "make erase      - mass-erase the chip via OpenOCD"
	@echo "make gdbserver  - start OpenOCD (GDB :3333, telnet :4444) without programming"

-include $(DEPS)
