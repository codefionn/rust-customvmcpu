li $r1, %string // Load string address
li $r2, 14 // Length of string

syscalli 1 // Print to console

li $r1, 0
syscalli 0

string:
	.str "Hello, world!\n"
