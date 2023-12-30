# Aglet - Another Great Language Executing Things

### Basic Road Map (will change)

- Don't worry about objects yet.
- Don't worry about arrays yet.
- Don't worry about strings yet.
- Don't worry about floats yet.
- Don't worry about booleans yet.
- Don't worry about integers bigger than 8bit.
- Basically, for first iteration, just integers and characters.

- Functions are first class citizens.
- NO ASYNC / PARALLEL (don't think the 6502 can even do that?)
- IF based conditionals. NO CASE (at least at first)
- Range based for loops... I don't think C's for loops are good at all. Don't have to worry about this yet. will be V2

- Don't worry about self-hosting yet! that is a ways away.
- However, syntax should be simple enough that self-hosting may be possible in the future!
- Ideally the source code can be compiled in a single pass...
- Need to keep tiny memory footprint in mind!!!!!!
- Alternately, we could have the compiler

Talking points:
- Iterators as a language construct? might be cool.
- Constant VS mutable?
- How many keywords do we want? I like to keep them to a minimum.
- Strict typing is a must, it simplifies stuff so much.
Later...:
- How will we tell the OS to run some program? maybe syntax like `!['test', 'param1', 'param2', var]`? it needs to be simple, easy to add... maybe `![test param1 param2 $var]`? The latter can just be syntax sugar for `exec(['test', 'param1', 'param2', var])`

-----

Some form of timekeeping, just simple as "how long has the computer been running"
start with while loops, do for loops LATER!


NOTES FOR FIRST RUN:
>	We want functions
>
>	WHILE and IF are only 2 control structures
>
>	"set" for constants, "let" for mutable vars
>	syntax like `set x:i16 = 123`
>	Might be too similar? need to crowd test this, see if it's too easy to confuse.
>
>	Don't worry about integers bigger than 1 byte, or non-integer types.
>
>	allow inline assembly at first, and then when we generate raw bytecode, allow inline bytecode.
