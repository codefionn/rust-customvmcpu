jil %plus_one
jil %plus_one

syscalli 0

plus_one:
	li $r1, 1
	add $r0, $r1
	j $ra
