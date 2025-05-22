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
ACTIVE_PACKAGE   := basic-android-integration

#NOCAPTURE := --nocapture

#----------------------------------------------[here are our rules]

default: $(DEFAULT)

ANDROID_NDK_HOME := /usr/local/share/android-ndk

android:
	ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(NDK) $(NDK_TARGETS) -o $(ACTIVE_PACKAGE) $(BUILD) $(FEATURES)

test:
	RUST_LOG=trace RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE_PACKAGE) -- $(NOCAPTURE)

vendor:
	cargo vendor

rustup_add_targets:
	rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
