# What is catlang?

catlang is a language design to replace C in all ways
catlang doesn't requires libc which means the first compiling target is bare metal

# Examples

```cat
// importing
import @cpu

@cpu.start
fun main(): None { // None can be omitted
    @cpu.mov eax, ebx
    @cpu.end
}
```

# Future

I may use this language to make a operating system and run catlang
like this(?):
```cat
fun main() {
    println("Hello, world")

    for i in 0..10 {
        match i {
            0 => println("zero"),
            1 => println("one"),
            2 => println("two"),
            3 => println("three"),
            4 => println("four"),
            5 => println("five"),
            6 => println("six"),
            7 => println("seven"),
            8 => println("eight"),
            9 => println("nine"),
            _ => println("unknown"),
        }
    }
```
