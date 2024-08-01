.PHONY: run build clean

LIBRARY = libopds_jni
LIBS = app/jni/libs
PROJECT = ~/AndroidStudioProjects/OpdsClient

run:
	java -Djava.library.path=LIBRARY/debug -cp classes org.opds.api.tests.Main

build:
	cargo build
	javac -h . org/opds/api/jni/Wrapper.java
	javac -d ./classes org/opds/api/models/*.java
	javac -d ./classes org/opds/api/jni/*.java
	javac -d ./classes org/opds/api/tests/*.java

clean:
	rm -rf classes/org
	rm -rf app

targets:
	cargo update
	cargo ndk -t arm64-v8a build --release
	mkdir -p $(PROJECT)/$(LIBS)/arm64-v8a
	rsync -uP  target/aarch64-linux-android/release/$(LIBRARY).so $(PROJECT)/$(LIBS)/arm64-v8a/
	cargo ndk -t armeabi-v7a build --release
	mkdir -p $(PROJECT)/$(LIBS)/armeabi-v7a
	rsync -uP  target/armv7-linux-androideabi/release/$(LIBRARY).so $(PROJECT)/$(LIBS)/armeabi-v7a/
	cargo ndk -t x86 build --release
	mkdir -p $(PROJECT)/$(LIBS)/x86
	rsync -uP  target/i686-linux-android/release/$(LIBRARY).so $(PROJECT)/$(LIBS)/x86/

