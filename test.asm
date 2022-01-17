main:
	mov esp %15

	// move stack to allocate space for program
	const 4096u eax
	addi esp eax esp

	// save ptr to start of program
	push %15

	// load program path
	const "test.prog" ebx
	addi ebp ebx ebx

	// write length to ebp
	const 4u eax
	addi ebx eax eax

	// write *char to eax
	load ebx ebx 4
	
	// load read call to ecx
	const 1u ecx	

	push erp
	// call read
	call ecx
	pop erp

	// load parse_program
	const parse_program ecx
	addi ebp ecx ecx

	push erp
	push eax
	push ebx
	call ecx
	pop erp

	pop eax
	subi %15 eax ebx

	const 0u ecx

	push erp
	call ecx
	pop erp

	// exit program
	exit eax

parse_program:
	pop ecx
	pop ebx

	mov esp %13
	const 1024u edx
	addi esp edx esp

	mov esp %14
	const 136u edx
	addi esp edx esp
	store %13 %14 4

parse_program::loop:	
	// skip whitespace

	// load leading_whitespace
	const leading_whitespace edx
	addi ebp edx edx	

	// save state
	push ebx
	push ecx
	// save erp
	push erp
	// push args
	push ebx
	push ecx
	// call leading_whitespace
	call edx
	// restore erp
	pop erp
	// restore state
	pop ecx
	pop ebx

	// remove from start of string
	addi ebx eax ebx
	subi ecx eax ecx

	// load parse_line
	const parse_line edx
	addi ebp edx edx

	// save state
	push ebx
	push ecx
	// save erp
	push erp
	// push args
	push ebx
	push ecx
	// call parse_line
	call edx
	// restore erp
	pop erp
	// restore state
	pop ecx
	pop ebx

	push eax

	// load parse_stmt
	const parse_stmt edx
	addi ebp edx edx

	// save state
	push ebx
	push ecx
	// save erp
	push erp
	// push args
	push ebx
	push eax
	push %14
	// call parse_stmt
	call edx
	// restore erp
	pop erp
	// restore state
	pop ecx
	pop ebx

	pop eax

	// remove from start of string
	addi ebx eax ebx
	subi ecx eax ecx
	
	const parse_program::loop edx
	addi ebp edx edx
	jmpnz edx ecx

	const 1024u edx
	subi esp edx esp

	const 136u edx
	subi esp edx esp
	ret

parse_stmt:
	pop %14
	pop ebx
	pop eax

	const parse_stmt::not_empty ecx
	addi ebp ecx ecx
	jmpnz ecx ebx
	ret

parse_stmt::not_empty:
	// check if last character is ':'
	const 1u ecx
	subi ebx ecx ecx
	addi eax ecx ecx
	load ecx ecx 1
	const 58u edx
	eq ecx edx ecx
	const parse_stmt::label edx
	addi ebp edx edx
	jmpnz edx ecx

parse_stmt::expr:
	const parse_expr ecx
	addi ebp ecx ecx

	// store erp
	push erp
	// push args
	push eax
	push ebx
	push %14
	// call parse_expr
	call ecx
	// restore erp
	pop erp

	ret

parse_stmt::label:
	const 4u ecx
	addi %14 ecx ecx
	const 0u edx
	store edx ecx 4

	const copy ecx
	addi ebp ecx ecx

	// save ebx
	push ebx
	// save erp
	push erp
	// push args
	push eax
	push %15
	push ebx
	// call copy
	call ecx
	// restore erp
	pop erp
	// restore ebx
	pop ebx

	// increment ptr
	addi %15 ebx %15

	const 10u eax
	store eax %15 1
	const 1u eax
	addi %15 eax %15
	
	ret

parse_expr:
	pop %14
	pop ecx
	pop ebx

	// store state
	push ebx
	push ecx
	// store erp
	push erp
	// push args
	push ebx
	push ecx
	// call not_whitespace
	const not_whitespace eax
	addi ebp eax eax
	call eax
	// restore erp
	pop erp
	// restore state
	pop ecx
	pop ebx
	
	ret

not_whitespace:
	pop ebx
	pop eax

	const 0u ecx

not_whitespace::loop:
	// load character
	addi eax ecx edx
	load edx edx 1

	// char > 32
	const 32u %9	
	gti edx %9 %10

	// char < 127
	const 127u %9
	lti edx %9 %11

	xor %10 %11 %10

	// if false jump to end
	const not_whitespace::end %9
	addi ebp %9 %9
	jmpnz %9 %10

	// increment ecx
	const 1u edx
	addi ecx edx ecx

	// check if string is over
	subi ebx ecx edx

	// if false jump to loop
	const not_whitespace::loop %9	
	addi ebp %9 %9
	jmpnz %9 edx

not_whitespace::end:
	mov ecx eax
	ret

parse_call:
	const 0u eax
	ret

parse_label:
	ret

// get number of characters to next line
parse_line:
	pop ebx
	pop eax

	const 0u ecx

parse_line::loop:
	eq ebx ecx %9
	const parse_line::end edx
	addi ebp edx edx
	jmpnz edx %9

	// load character
	addi eax ecx edx
	load edx edx 1

	// load '\n'
	const 10u %9

	eq edx %9 edx

	// end if char != \n
	const parse_line::end %9
	addi ebp %9 %9
	jmpnz %9 edx

	// increment ecx
	const 1u edx
	addi ecx edx ecx

	subi ebx ecx %9
	const parse_line::loop edx
	addi ebp edx edx
	jmpnz edx %9

parse_line::end:
	mov ecx eax
	ret


// gets length of leading whitespace in string
leading_whitespace:
	pop ebx
	pop eax

	const 0u ecx

leading_whitespace::loop:
	// load character
	addi eax ecx edx
	load edx edx 1

	// char > 32
	const 32u %9	
	gti edx %9 %10

	// char < 127
	const 127u %9
	lti edx %9 %11

	// 32 > char > 127
	and %10 %11 %10

	// if false jump to end
	const leading_whitespace::end %9
	addi ebp %9 %9
	jmpnz %9 %10

	// increment ecx
	const 1u edx
	addi ecx edx ecx

	// check if string is over
	subi ebx ecx edx

	// if false jump to loop
	const leading_whitespace::loop %9	
	addi ebp %9 %9
	jmpnz %9 edx

leading_whitespace::end:
	mov ecx eax
	ret

copystr:
	pop ebx
	pop eax

	// load length
	load eax ecx 4

	// load string start
	const 4u edx
	addi eax edx eax

	const copy edx
	addi ebp edx edx

	// save erp
	push erp
	// push args
	push eax
	push ebx
	push ecx
	// call copy
	call edx
	// restore erp
	pop erp

	ret

copy:
	pop ecx
	pop ebx
	pop eax

copy::loop:
	// decrement ecx
	const 1u edx
	subi ecx edx ecx

	// load byte
	addi eax ecx edx
	load edx %9 1

	// write byte
	addi ebx ecx edx
	store %9 edx 1

	// loop if ecx > 0
	const copy::loop edx
	addi ebp edx edx
	jmpnz edx ecx

	// return
	ret


// compares strings
// inputs
//  0. string a
//  1. string b
strcmpl:
	pop ebx	
	pop eax

	// load lengths
	load eax ecx 4
	load ebx edx 4

	// load not_eq
	const strcmpl::not_eq %9
	addi ebp %9 %9

	// compare lengths
	xor ecx edx ecx
	// if not equal jump to not_eq
	jmpnz %9 ecx

	// get *char
	const 4u ecx
	addi eax ecx eax
	addi ebx ecx ebx

	// load strcmp	
	const strcmp ecx
	addi ebp ecx ecx

	// save erp
	push erp
	// push args
	push eax
	push ebx
	push edx
	// call strcmp
	call ecx
	// load erp
	pop erp

	ret

strcmpl::not_eq:
	const 0u eax
	ret


// compares ptrs
// inputs
//  0. *char a
//  1. *char b
//  2. u32 length
strcmp:
	pop ecx
	pop ebx
	pop eax

strcmp::loop:
	// decrement ecx
	const 1u edx
	subi ecx edx ecx

	// read A char at ecx
	addi eax ecx edx
	load edx %9 1

	// read B char at ecx
	addi ebx ecx edx
	load edx %10 1

	// load label
	const strcmp::not_eq edx
	addi ebp edx edx

	// compare A char and B char
	xor %9 %10 %9
	// if not equal jump to not_eq
	jmpnz edx %9

	// load loop
	const strcmp::loop edx
	addi ebp edx edx

	// if ecx > 0 repeat
	jmpnz edx ecx

	// return true
	const 1u eax
	ret

strcmp::not_eq:
	// return false
	const 0u eax
	ret
