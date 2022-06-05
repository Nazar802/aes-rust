# aes-rust
Rust project that implements AES block cipher.

Available key lengths are 128, 192 and 256.
The input string has no upper limit, but it would not be safe to encrypt strings that are too large. It would occupy a lot of memory.

Compile the binary by running 
```
make
```

To run the program cd into target/release or copy the binary whereever you want to, make it executable if needed, and run the command like this:
```
aes -l <key length> -in "input"

OR

aes --length <key length> --input "input"
```


Feel free to clone and make any changes you wish.