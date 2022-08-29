lwi $r3, %int_array_len // Length of int_array

li $r1, 0 // Result
li $r2, 0 // Index
// For-Loop
loop:
	li $r4, %int_array // Address
	cpy $r6, $r2 // Index * 4
	muli $r6, 4
	add $r4, $r6 // Address + Index
	lw $r7, $r4 // Load num from array
	add $r1, $r7 // Add to result
	addi $r2, 1
	// Check if index < array.length
	cpy $r5, $r3
	sub $r5, $r2
	jnzi $r5, %loop

end:
	syscalli 0

int_array_len:
	.i32 11
int_array:
	.i32 0
	.i32 1
	.i32 2
	.i32 3
	.i32 4
	.i32 5
	.i32 6
	.i32 7
	.i32 8
	.i32 9
	.i32 10
