

# The OPDS JNI

### Cross compilation


```
cargo install cargo-ndk
export NDK_HOME=~/Android/android-ndk-r27
cargo ndk -t armeabi-v7a b --release
cargo ndk -t arm64-v8a b --release
```