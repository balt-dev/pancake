Pancake is a stack-based esoteric programming language
created by heptor42@gmail.com.

Registers X and Y start empty.
Trying to access empty registers errors.
Pushing or operating with the X and Y registers empties them.
Overdrawing the stack raises an error.
Any text after the fields of a register is treated as a comment.

All instructions have any amount of whitespace before them.
Labels can be marked by lines without whitespace, and their names must not
have whitespace in them.

Any arithmetic instructions will silently overflow at the bounds of their types.

Instructions

- push integer 100
    Push an integer. Big-endian, 64-bit, signed, with two's complement wrapping.
- push float 1.5
    Push a floating point. Double precision IEEE754. NaN and +/- Infinity are pushable using this.
- push boolean true
    Push a boolean.
- push character 'H'
    Push a character. Stored as an 8-bit unsigned integer, which is not necessarily
    valid ASCII, but don't expect good results for I/O if it isn't kept that way.
    Any ASCII whitespace cannot be written out, and has to be pushed with the below instruction.
- push character #00
	Push a character with an arbitrary numerical value. Hexadecimal.
- push register X
    Push the value in the register.
    Errors if the register is empty.
- pop X
    Pop a stack value into one of two registers, or _ to discard.
- copy X
    Copies the value in this register to the other.
- length X
    Puts the current length of the stack, as an integer,
    into the given register.
- swap X 0
    Swaps the value in this regsiter
    with the value in at the value at the index
    of the top of the stack minus the given value.

- jump LOOP
    Always jumps to a label.
- branch LOOP
    Jumps to a label if the value on the top of the stack is true,
    popping it. Errors if the value isn't a boolean.
- goto X
	Jumps to the instruction index specified by the integer in
	the specified register. Exits if the value is negative or larger
	than the program.

- call LOOP
	Does the same as branching, but additionally pushes the current
	instruction index to the stack as an integer.
	Meant to be used for function calls.
- return
	Pops an integer off of the stack and jumps to that instruction index plus one.
	Meant to be used for function calls. Exits if the value is negative
	or larger than the program.

- compare equal
    Compares the values in X and Y, and pushes the boolean result
    to the stack. If "equal" or "unequal", the values may be of differing
    types, but if "less" or "greater", an error is raised if they are.
    A boolean true is larger than a boolean false,
    and characters are compared by ASCII codepoint. 
    Floats follow IEEE745 logic, meaning NaN != NaN.
- add
    Adds X and Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- subtract
    Subtracts X from Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- multiply
    Multiplies X and Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- divide
    Divides X and Y, and pushes the quotient to the stack.
    Valid for integers and floats. Values must be the same type.
    Raises an error on integer division by 0.
- modulo
    Divides X and Y, and pushes the modulo
    (Euclidean remainder) to the stack.
    Valid for integers and floats. Values must be the same type.
    Raises an error on integer division by 0.
- negate X
    Negates the value of the register, and leaves it in said register.
    Valid for integers and floats. 

- and
    Takes the bitwise or logical AND of X and Y, 
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- or
    Takes the bitwise or logical OR of X and Y,
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- xor
    Takes the bitwise or logical XOR of X and Y,
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- not X
    Takes the bitwise NOT of the register,
    and leaves it in said register.
    Valid for integers and booleans.

- shift
    Logically right shifts the integer in X to the right by
    the signed integral amount of bits in Y,
    discarding extra bits and padding with 0, and pushes the result.
    Only valid for integers.
- rotate
    Shifts the integer in X to the right by
    the signed integral amount of bits in Y,
    wrapping extra bits to the other side, and pushes the result.
    Only valid for integers.

- cast integer X
    Attempts to cast the value in the register to the given type,
    leaving it in place. Pushes a boolean returning whether
    the cast successfully completed.

    You can cast booleans into integers and characters,
    integers into booleans (!= 0), floats, and characters (% u8),
    floats into integers (saturating at bounds and 0 for NaN),
    and characters into booleans (!= 0) and integers.

- reinterpret integer X
    Reinterprets the value in the register as the given type,
    without touching the bits.

    You can reinterpret a boolean as an integer or a character,
    an integer as a float (q_rsqrt, anyone?),
    a float as an integer,
    and a character as an integer.

- input character X
    Grabs a value from stdin as text, and puts it into this register.
    If reading fails, an error is raised.
    If the given stdin string couldn't be parsed as a value,
    the input is re-queried until it can be.
- read character X
    Grabs a value from stdin as bytes, and puts it into this register.
    If reading fails, an error is raised.
- output X
    Outputs the value in the register to stdout as text.
    If writing fails, an error is raised.
    Writing characters prints them escaped, including things like newlines.
    If you don't want this, use write.
- write X
    Outputs the value in the register to stdout as bytes.
    If writing fails, an error is raised.

- random integer X
    Puts a random valid value of the specified type into the register.
    For floats, this may be any value in the half-open range [0.0, 1.0).
- break
    Immediately halts the program.
- drop X
	Empties the register.
- debug
	Prints out the current state of the interpreter to stderr.
	May do nothing on platforms where there is none.
- *...
    A comment. Lasts until the end of the line.