.PHONY: run build clean
run:
	java -Djava.library.path=target/debug -cp classes Main

build:
	cargo build
	# javac -d classes -h java java/Wrapper.java
	javac -d classes java/*.java

clean:
	rm classes/*.class
