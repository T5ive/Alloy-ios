# Rust library
before-all::
	@printf "\033[1;32m==>\033[0m Building Rust library...\n"
	cargo build --release --target aarch64-apple-ios

# Target configuration
ARCHS = arm64
TARGET := iphone:clang:latest:14.0
FINALPACKAGE = 1

# Device config (change IP as needed)
DEVICE_IP ?= 1.1.1.1
DEVICE_USER ?= mobile
DEVICE_PASS ?= 1

include $(THEOS)/makefiles/common.mk

TWEAK_NAME = rgg

${TWEAK_NAME}_CFLAGS = -fobjc-arc
${TWEAK_NAME}_LDFLAGS = -force_load target/aarch64-apple-ios/release/librust_igmm.a

include $(THEOS_MAKE_PATH)/tweak.mk

clean::
	rm -rf .theos packages
	cargo clean

# We are using sshpass to avoid typing the password every time
deploy:
	$(MAKE) clean
	$(MAKE) all
	$(MAKE) package
	@printf "\033[1;32m=>\033[0m Copying package to device...\n"
	@sshpass -p "$(DEVICE_PASS)" scp packages/*.deb $(DEVICE_USER)@$(DEVICE_IP):~
	@printf "\033[1;32m>\033[0m Deploy complete.\n"
