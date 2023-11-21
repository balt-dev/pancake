Pancake is a stack-based esoteric programming language
created by heptor42@gmail.com.

Registers X and Y start empty.
Trying to access empty registers errors.
Pushing or operating with the X and Y registers empties them.
Overdrawing the stack raises an error.
Any text after the fields of a register is treated as a comment.

Instructions

- push integer 100: 
    Push an integer. 64-bit signed with two's complement wrapping.
- push float 1.5: 
    Push a floating point. Double precision IEEE754.
- push boolean true: 
    Push a boolean.
- push character H: 
    Push a character. Must be valid ASCII.
    May be escaped with \x00 - \x7F.
- push register X: 
    Push the value in the register.
    Errors if the register is empty.
- pop X: 
    Pop a stack value into one of two registers.
- copy X: 
    Copies the value in a register and pushes it to the stack.
- length X: 
    Puts the current length of the stack, as an integer,
    into the given register.

- label LOOP:
    Sets a label to jump to.
    This instruction is executed, unconditionally,
    before execution of any other.
- branch LOOP: 
    Jumps to a label if the value on the top of the stack is true,
    popping it. Errors if the value isn't a boolean.

- compare equal:
    Compares the values in X and Y, and pushes the boolean result
    to the stack. If "equal" or "nonequal", the values may be of differing
    types, but if "less" or "greater", an error is raised if they are.
    A boolean true is larger than a boolean false,
    and characters are compared by ASCII codepoint. 
    Floats follow IEEE745 logic, meaning NaN != NaN.
- add:
    Adds X and Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- subtract:
    Subtracts X from Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- multiply:
    Multiplies X and Y, and pushes the result to the stack.
    Valid for integers and floats. Values must be the same type.
- divide:
    Divides X and Y, and pushes the quotient to the stack.
    Valid for integers and floats. Values must be the same type.
    Raises an error on integer division by 0.
- modulo:
    Divides X and Y, and pushes the modulo
    (euclidean remainder) to the stack.
    Valid for integers and floats. Values must be the same type.
    Raises an error on integer division by 0.
- negate X:
    Negates the value of the register, and leaves it in said register.
    Valid for integers and floats. 

- and: 
    Takes the bitwise or logical AND of X and Y, 
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- or: 
    Takes the bitwise or logical OR of X and Y,
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- xor:
    Takes the bitwise or logical XOR of X and Y,
    and pushes the result to the stack.
    Valid for integers and booleans. Values must be the same type.
- not X:
    Takes the bitwise NOT of the register,
    and leaves it in said register.
    Valid for integers and booleans.

- shift: 
    Shifts the integer in X to the right by
    the signed integral amount of bits in Y,
    discarding extra bits and padding with 0, and pushes the result.
    Only valid for integers.
- rotate:
    Shifts the integer in X to the right by
    the signed integral amount of bits in Y,
    wrapping extra bits to the other side, and pushes the result.
    Only valid for integers.

- cast integer X:
    Attempts to cast the value in the register to the given type,
    leaving it in place. Pushes a boolean returning whether or not
    the cast successfully completed.
- reinterpret integer X:
    Reinterprets the value in the register as the given type,
    without touching the bits.
    Only valid for integers and floats.

- input character X: 
    Grabs a value from stdin as text, and puts it into this register.
    If reading fails, an error is raised.
- read character X:
    Grabs a value from stdin as bytes, and puts it into this register.
    If reading fails, an error is raised.
- output X:
    Outputs the value in the register to stdout as text.
    If writing fails, an error is raised.
- write X:
    Outputs the value in the register to stdout as bytes.
    If writing fails, an error is raised.

- random integer X:
    Puts a random valid value of the specified type into the register.
    For floats, this may be any value in the half-open range [0.0, 1.0).
- break
    Immediately halts the program.
- *...
    A comment. Lasts until the end of the line.