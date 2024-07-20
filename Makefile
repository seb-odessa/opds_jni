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

targets:
	cargo ndk -t armeabi-v7a build --release
	cargo ndk -t arm64-v8a build --release
	cargo ndk -t x86_64 build --release