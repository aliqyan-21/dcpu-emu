## DCPU Assembly Spec

The specifications of how assembly will be written for this DCPU-16 implementation. It could grow as commands grow.

> **Note:** There is no official asm doc for dcpu, this is what I have made for this implementation

> Whole assembly is case insensitive

### Basic Instruction

**Syntax:** `cmd <b> <a>`

> here, `b` = destination and `a` = source

### Special Instruction

**Syntax:** `cmd <a>`

---

### Values and Addressing Modes

#### 1. Registers

* **`reg`**: Any general-purpose register (`a`, `b`, `c`, `x`, `y`, `z`, `i`, `j`) or special registers where permitted (`pc`, `sp`, `ex`).

> `set a b` ; copy value from register b to register a

#### 2. Memory Locations (Pointers)

* **`[val]`**: Value at RAM address `val`.

> `set [0x1000] a` ; store value of register a into memory address 0x1000

* **`[reg]`**: Value at RAM address contained in register.

> `set [i] 0x00ff` ; store 0x00ff at the memory address pointed to by register i

* **`[reg + val]` / `[val + reg]`**: Offset memory access.

> `set a [i + 4]` ; load value from address (i + 4) into register a

#### 3. Stack Operations

* **`push` / `pop`**: Stack pointer push/pop automation.

> `set push a` ; push register a onto the stack (decrements sp first)

> `set a pop` ; pop value from stack into register a (increments sp after)

> `peek` / `[sp]` ; reference top of stack without changing sp

#### 4. Literals & Constants

* **`val`**: Short literals (`0` through `31`) embedded directly without extra words.

> `add a 5` ; adds literal 5 directly to a

* **`val`**: Next-word literals/values (hex, decimal, or character codes) requiring an inline extra word.

> `set a 0xbeef` ; assign 0xbeef to register a

---

### Labels and Directives

#### 1. Labels

* case insensitive
* defined with a colon prefix (`:label_name`). Used as jump destinations or targets for data.

```assembly
:loop
  add a 1
  ife a 10
    set pc loop    ; jump back to loop target

```

#### 2. Data Directives

* `.dat`
* used to inline raw words or strings into the compiled binary output.

```assembly
message:
  .dat "Hello", 0x00   ; character array terminated by null

```

### Reference

* [DCPU-16 Examples Repository by Markus Pearson / James Tauber](https://github.com/jtauber/DCPU-16-Examples/)
