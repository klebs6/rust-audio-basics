.PHONY: test android vendor

#RUSTFLAGS  := "-Awarnings -Z time-passes"
RUSTFLAGS   := -Awarnings RUST_BACKTRACE=1

#CARGO      := env CARGO_MSG_LIMIT=15 CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 
CARGO       := MAKEFLAGS= env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 

BUILD       := build --verbose --release
RUN         := run
TEST        := test
NDK         := ndk
NDK_TARGETS := -t armeabi-v7a -t arm64-v8a

#-----------------------------------------------------[this section lets us configure the make invocation]
DEFAULT          := android
DEFAULT          := test
ACTIVE_PACKAGE   := basic-android-integration

NOCAPTURE := --nocapture

#----------------------------------------------[here are our rules]

default: $(DEFAULT)

ANDROID_NDK_HOME := /home/loko/Android/Sdk/ndk/27/

android:
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) \
					 RUSTFLAGS=$(RUSTFLAGS) \
					 $(CARGO) $(NDK) $(NDK_TARGETS) -o $(ACTIVE_PACKAGE) \
					 $(BUILD) $(FEATURES)

test:
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) \
	RUSTFLAGS=$(RUSTFLAGS) \
	RUST_LOG=trace \
	 $(CARGO) $(NDK) -t armeabi-v7a build --package $(ACTIVE_PACKAGE) --tests
    ## $(CARGO) $(NDK) -t arm64-v8a build --package $(ACTIVE_PACKAGE) --tests

	@echo "Locating test binary"
	BINARY=$$(find target/armv7-linux-androideabi/debug/deps -type f -name "$(subst -,_,$(ACTIVE_PACKAGE))-*" ! -name "*.d" ! -name "*.rlib" | head -n 1); \
	##BINARY=$$(find target/aarch64-linux-android/debug/deps -type f -name "$(subst -,_,$(ACTIVE_PACKAGE))-*" ! -name "*.d" ! -name "*.rlib" | head -n 1); \
	if [ -z "$$BINARY" ]; then \
		echo "Error: Test binary not found."; \
		echo "Searched for pattern: '$(subst -,_,$(ACTIVE_PACKAGE))-*' in target/armv7-linux-androideabi/debug/deps/"; \
		echo "Directory contents:"; \
		ls -l target/armv7-linux-androideabi/debug/deps/; \
		exit 1; \
	fi; \
	echo "Found test binary: $$BINARY"; \
	echo "Pushing $$BINARY to Android device" && \
	adb push "$$BINARY" /data/local/tmp/test_binary

	@echo "Executing test binary on Android device"
	adb shell chmod +x /data/local/tmp/test_binary
	# Pass RUST_LOG and RUST_BACKTRACE for better debugging on device
	adb shell RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=1 /data/local/tmp/test_binary $(NOCAPTURE)

vendor:
	cargo vendor

rustup_add_targets:
	rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
