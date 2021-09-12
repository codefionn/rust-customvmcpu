# rust-vmcpu

Custom CPU Instruction set executed with a rust program

## Instruction format

Instructions are always 32-bit long

```
0-7: Opcode (8-bit)

// Register 1 + Register 2/Immediate
8-11: Register 1 (4-bit)
12-31: Register 2 / Immediate value (21-bit)

// Only register/immediate
8-31

// Register 1 + Register 2 + Imediate
8-11: Register 1 (4-bit)
12-15: Register 2 (4-bit)
16-31: Immediate (16-bit)
```

## Registers

Registeres start with $. The maximum number of registers are 16 (4-bit).

```
// General purpose Registers (32-bit)
$r0
$r1
$r2
$r3
$r4
$r5
$r6
$r7

// Stack pointer
$sp

// Instructions pointer
$ip

// Return instruction pointer (Return-adress)
$ra

// Error register (store error codes)
$err
```

## Immediates

Immediates are integers. Also constants (like jump points) can be used as
immediates starting with the % sign.

Example:

```
program;
	li $r0, %program
```

Also in the assembler code representation, arithmetics can be used to
manipulate the immediate. Supported operations:

```
// Addition
%x + %y

// Subtraction
%x - %y

// Multiplication
%x * %y

// Division (floor)
%x / %y

// Negative
-

// Brackets
%x + (%x * %y)
(%x * %y) + %x

```

Note: there's no operator precendence, but operations can be wrapped in curly
brackets ().

## Instructions

```
// --- Memory operations ---
// Copy from y to x (mov)
cpy $x $y

// Load from memory position y into x
ld $x $y

// Store x to memory position y
st $x $y

// Store immediate y into x
li $x, %y

// --- Arithmetic instructions ---
// Add x and y and store result in x
add $x $y

// Subtract y from x and store result in x
sub $x $y

// Multiply x times y and store result in x
mul $x $y

// Divide x through y and store result in x
div $x $y

// Bitwise and x y and store result in x
and $x $y

// Bitwise or x y and store result in x
or $x $y

// Bitwise xor x y and store result xor
xor $x $y

// Bitwise not x and store result in x
not $x

// --- Flow-control operations ---
// Uncoditional jump to x
j $x

// Unconditional jump to immediate x
ji %x

// Copy $ip+4 (next instruction) to %ra and
// do unconditional jump to immediate x (jump and link instruction)
// should be used to call a procedure/"function"
jil %x

// Jump to y, if x is zero
jzi $x, %y

// Jump to y, if x is not zero
jnzi $x, %y

// Jump to y, if x is less than zero (x must be a twos-complement)
jlzi $x, %y

// Jump to y, if x is greater than zero (x must be a twos-complement)
jgzi $x, %y

// Syscall immediate value
syscalli %x
```

## System calls

The following system calls are supported

- 0: Exit program (r1 is status value), writes $ip+4 to $ra

## Opcodes

- cpy: 0x00
- ld: 0x01
- st: 0x02
- li: 0x03
- add: 0x04
- sub: 0x05
- mul: 0x06
- div: 0x07
- and: 0x08
- or: 0x09
- xor: 0x0A
- not: 0x0B
- j: 0x0C
- ji: 0x0D
- jil: 0x0E
- jzi: 0x0F
- jnzi: 0x10
- jlzi: 0x11
- jgzi: 0x12
- syscalli: 0x13
