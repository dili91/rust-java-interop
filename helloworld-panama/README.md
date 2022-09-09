# HelloWorld Panama

To run: 

```shell
docker run -it --rm -v $PWD/helloworld-panama:/work adilisio/rust-java-interop bash
```
...and once inside the container: 
```shell
cargo build

jextract -d classes -t org.openjdk --include-function hello_world -l helloworld_panama -- lib.h

java --add-modules jdk.incubator.foreign --enable-native-access=ALL-UNNAMED -Djava.library.path=./target/debug -cp classes HelloWorldPanama.java
```