# Overview

Stack is a minimal virtual machine used to run reloadable code on embedded machines. It is based on a RISC architecture, and has a whopping 71 instructions.

## But why?

Typically, we think of lua when we think of reloadable code from C, and for good reason. The Lua interpreter is extremely fast, and can run in tandem with C code. However, on extremely memory-limited systems, Lua may not be a good choice. Its garbage collection causes systems to slow down, and the library is not small enough in some circumstances. Because of this, I created Stack, a minimal C library that has plenty of room for custom-defined functions, and takes up minimal memory. Memory management in stack is done manually, so writing raw Stack code should only be done by compilers or very experienced individuals. 
