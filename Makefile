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

#NOCAPTURE := --nocapture

#----------------------------------------------[here are our rules]

default: $(DEFAULT)

ANDROID_NDK_HOME := /usr/local/share/android-ndk

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

	@echo "Locating test binary"
	BINARY=$$(find target/armv7-linux-androideabi/debug/deps -type f -name "$(ACTIVE_PACKAGE)-*" ! -name "*.d" ! -name "*.rlib" | head -n 1) && \
	echo "Pushing $$BINARY to Android device" && \
	adb push "$$BINARY" /data/local/tmp/test_binary

	@echo "Executing test binary on Android device"
	adb shell chmod +x /data/local/tmp/test_binary
	adb shell /data/local/tmp/test_binary $(NOCAPTURE)

vendor:
	cargo vendor

rustup_add_targets:
	rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
