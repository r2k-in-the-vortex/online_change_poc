This is a proof-of-concept for PLC code online change in Rust
Written by Rainer Kordmaa, MIT licence, do whatever you want with it.

demonstrates only function pointers,

for variable access, rust should probably also copy memory contents to it's own memory to persist data across online change
and handle cases if variables are added/removed between verisons

to create libraries
```
clang -shared project1.ll -o project1.so
clang -shared project2.ll -o project2.so
```

to run proof of concept
```
$ cargo run
   Compiling online_change_poc v0.1.0 (/workspaces/rusty/examples/online_change_poc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.49s
     Running `/workspaces/rusty/target/debug/online_change_poc`
First call result before got handover to rust: 42
Second call result after got handover to rust: 42
Third call result after loading new version of library: 67
```

same project IR can also work standalone
```
vscode ➜ /online_change_poc $ clang -static project1.ll -o project1
vscode ➜ /online_change_poc $ ./project1
vscode ➜ /online_change_poc $ echo $?
42
```