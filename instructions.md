# Registers 

 Name | Index | Usage
------|-------|-------
 eax  | 0     | accumulator
 ebx  | 1     | base
 ecx  | 2     | counter
 edx  | 3     | data
 eip  | 4     | instruction pointer 
 esp  | 5     | stack pointer
 erp  | 6     | return pointer
 ebp  | 7     | base pointer
 exp  | 8     | exit pointer 

## Exit pointer
When exiting.
If `exp == 0` shut down. Otherwise writes `exp` to `eip`.

# Instructions

 Name    | Opcode | Arg 0 | Arg 1 | Arg 2 | Usage													
---------|--------|-------|-------|-------|-------
 const   | 0      | src   |       |       | Loads the next word from memory into `%src`.
 mov     | 1      | src   | dst   |       | Writes `%src` to `%dst`.
 push    | 2      | src   |       |       | Stores `%src` to `@esp`. Increments `esp` by 4.
 pop     | 3      | dst   |       |       | Decrements `esp` by 4. Loads `@esp` to `%dst`.
 load    | 4      | src   | dst   | width | Loads `width` bytes of `@src` to `%dst`.
 store   | 5      | src   | dst   | width | Stores `width` bytes of `%src` to `@dst`.
 jmp     | 16     | trg   |       |       | Writes `%trg` to `eip`.
 jmpnz   | 17     | trg   | src   |       | Reads `%src`. If `%src == 0` writes `%trg` to `eip`.
 call    | 18     | trg   |       |       | Pushes `erp`. Writes `eip` to `erp`. Writes `%trg` to `eip`.
 ret     | 19     |       |       |       | Writes `erp` to `eip`. Pops `erp`.
 exit    | 20     | src   |       |       | Exits program according to data in `exp` with exitcode `%src`.
 addi    | 32     | lhs   | rhs   | dst   | Writes `%lhs + %rhs` to `%dst` as integers.
 subi    | 33     | lhs   | rhs   | dst   | Writes `%lhs - %rhs` to `%dst` as integers.
 muli    | 34     | lhs   | rhs   | dst   | Writes `%lhs * %rhs` to `%dst` as integers.
 divi    | 35     | lhs   | rhs   | dst   | Writes `%lhs / %rhs` to `%dst` as integers.
 modi    | 36     | lhs   | rhs   | dst   | Writes `%lhs % %rhs` to `%dst` as integers.
 gti     | 37     | lhs   | rhs   | dst   | Writes `%lhs > %rhs` to `%dst` as integers.
 lti     | 38     | lhs   | rhs   | dst   | Writes `%lhs < %rhs` to `%dst` as integers.
 shift   | 48     | src   | shift | dst   | Writes `%src >> %shift` to `%dst`.
 and     | 49     | src   | rhs   | dst   | Writes `%src & %rhs` to `%dst`.
 or      | 50     | src   | rhs   | dst   | Writes `%src | %rhs` to `%dst`.
 xor     | 51     | src   | rhs   | dst   | Writes `%src ^ %rhs` to `%dst`.
 addf    | 64     | lhs   | rhs   | dst   | Writes `%lhs + %rhs` to `%dst` as floating point numbers.
 subf    | 65     | lhs   | rhs   | dst   | Writes `%lhs - %rhs` to `%dst` as floating point numbers.
 mulf    | 66     | lhs   | rhs   | dst   | Writes `%lhs * %rhs` to `%dst` as floating point numbers.
 divf    | 67     | lhs   | rhs   | dst   | Writes `%lhs / %rhs` to `%dst` as floating point numbers.
 modf    | 68     | lhs   | rhs   | dst   | Writes `%lhs % %rhs` to `%dst` as floating point numbers.
 floorf  | 72     | lhs   | dst   |       | Writes `floor(%lhs)` to `%dst` as floating point numbers.
