.PHONY: run build clean

run:
	java -Djava.library.path=target/debug -cp classes org.opds.api.tests.Main

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
	cargo ndk -t armeabi-v7a build --release
	cargo ndk -t arm64-v8a build --release
	cargo ndk -t x86 build --release
	mkdir -p app/jni/libs/arm64-v8a
	cp ./target/aarch64-linux-android/release/libopds_jni.so ./app/jni/libs/arm64-v8a/
	mkdir -p app/jni/libs/armeabi-v7a
	cp ./target/armv7-linux-androideabi/release/libopds_jni.so ./app/jni/libs/armeabi-v7a/
	mkdir -p app/jni/libs/x86
	cp ./target/i686-linux-android/release/libopds_jni.so ./app/jni/libs/x86/