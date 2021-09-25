# rust-vmcpu

Custom CPU instruction set executed with a rust program. Licensed unter GPL
Version 3.

## Build & run

### Linux

Build:

```sh
git clone https://github.com/codefionn/rust-customvmcpu
cd rust-customvmcpu
cargo build
```

Run (in same directory)

```sh
cargo run -- --register-table ./src/exampleprogramwriter/out/add_32_100.bin
```

This runs the add program which adds two numbers on register $r0.

## Instruction format

Instructions are always 32-bit long and little-endian

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

### Values of the error register

The register $err can be on of the following values:

- 0: No error
- 1: Invalid opcode
- 2: Invalid register
- 3: Invalid syscall
- 4: Invalid memory address
- 5: Read-only register
- 6: Divisor must not be zero

If a program terminates with an error, they are terminated with the error code
32000 + $err.

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
cpy $x, $y

// Load word (32-bit integer) from memory position y into x
lw $x, $y

// Store word (32-bit integer) x to memory position y
sw $x, $y

// Load half-integer from memory position y into x
lh $x, $y

// Store half-integer x to memory position y
sh $x, $y

// Load integer from memory position y into x
lb $x, $y

// Store integer x to memory position y
sb $x, $y

// Load word (32-bit integer) from memory position y into x
lwi $x, %y

// Store word (32-bit integer) x to memory position y
swi $x, %y

// Load half-integer from memory position y into x
lhi $x, %y

// Store half-integer x to memory position y
shi $x, %y

// Load integer from memory position y into x
lbi $x, %y

// Store integer x to memory position y
sbi $x, %y

// Store immediate y into x (20-bit, is two's complement)
li $x, %y

// --- Arithmetic instructions ---
// Add x and y and store result in x
add $x, $y

// Subtract y from x and store result in x
sub $x, $y

// Multiply x times y and store result in x
mul $x, $y

// Divide x through y and store result in x
// If $y error, $x will also be overwritten with 0
div $x, $y

// Add x and y and store result in x
addi $x, %y

// Subtract y from x and store result in x
subi $x, %y

// Multiply x times y and store result in x
muli $x, %y

// Divide x through y and store result in x
// If %y error, $x will also be overwritten with 0
divi $x, %y

// --- Bitshift/logical instructions ---
// Bitwise and x y and store result in x
and $x, $y

// Bitwise or x y and store result in x
or $x, $y

// Bitwise xor x y and store result xor
xor $x, $y

// Shift right logical, (>>)
srl $x, $y

// Shift left logical, (<<)
sll $x, $y

// Shift right logical, (>>) with immediate
srli $x, %y

// Shift left logical, (<<) with immediate
slli $x, %y

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
- lw: 0x01
- sw: 0x02
- lh: 0x03
- sh: 0x04
- lb: 0x05
- sb: 0x06
- li: 0x07
- add: 0x08
- sub: 0x09
- mul: 0x0A
- div: 0x0B
- and: 0x0C
- or: 0x0D
- xor: 0x0E
- not: 0x0F
- j: 0x10
- ji: 0x11
- jil: 0x12
- jzi: 0x13
- jnzi: 0x14
- jlzi: 0x15
- jgzi: 0x16
- syscalli: 0x17
- srl: 0x18
- sll: 0x19
- srli: 0x20
- slli: 0x21
- addi: 0x22
- subi: 0x23
- muli: 0x23
- divi: 0x24
- lwi: 0x25
- swi: 0x26
- lhi: 0x27
- shi: 0x28
- lbi: 0x29
- sbi: 0x30
