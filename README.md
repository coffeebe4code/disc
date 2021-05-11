---
# About

The DISC language is a lisp adjacent language. The language itself has many features that most developers are accustomed to in the modern era, as well as several new ones.
This project has goals that will ensure success and adoption of the language. The goals are...

- **Tooling is first class**. The language server, the cli, the parser, everything for a developer to be successful will all be included in decisions and apart of the main project.
- **Make the fastest interpreted language available**. Tools will be able to output optimal performance suggestions for interpreted scripts. The language is designed around executing scripts as fast as possible.
- **Provide the best embedded support**. Embedded systems require demanding performance use of their hardware. Supporting embedded systems will force the tooling, to be as efficient as possible. Thus, further helping the second goal.
- **Documentation is key**. Making documentation another high priority, will ensure that any newcomers to the language get the most up to date features,knowledge, and tutorials for easy use.
- **Make it easy to use**. The first class tooling, the documentation, and clear goals will make this tool and language easy to use.

---
What does disc stand for?

- **D**ynamically (typed)
- **I**nterpreted
- **S**trongly (typed) 
- **C**ompiled language.

The interpreter and compiler for disc can give you options to control the four pillars of static and dynamic type analysis, weak and strong typing.

Binaries are not cross-platform, Scripts are cross-platform. Developers with disc will be able to choose to create minified scripts that can be ran on any machine, compiled binaries, or both. Many developers work on different systems, and the tools output a binary that is specific to x86-64, riscv32, arm32, the list goes on.
In the business world for CICD, many developers would work on windows, but then pipelines or servers could be linux based, or if the company is fancy, they might even have their developers be on mac. We need to make the code that is developed, portable. 

There are two main factions in the software building community. 

One side cares strictly about the correctness of the code, and has no issues enforcing the strictest type checking. More time is spent receiving feedback from the compiler. This reduces the feedback of runtime behavior, but insures correctness, and often, fewer bugs.

While the other side, has no problems with implicit casts, doesn't type check and allows you to run code. When working with javascript, I find its lack of typing nice at times when I just want to see the code run, inspect breakpoints, variables in memory, etc. Actually running the code, and seeing the actual behavior has its benefits. 

Disc has the most powerful type handling, and allows much more granular control over the four pillars of analysis.

---
# Contents
- [About](#about)
- [Introduction](#introduction)
- [Installation](#installation)
- [Beginners](#beginners)
- [Language](#language)
	- [Program and Scope](#program-and-scope)
	- [Types](#types)
	- [Properties](#properties)
	- [Variables](#variables)
	- [Functions](#functions)
	- [Identifiers](#identifiers)
	- [Builtins](#builtins)
	- [Enums](#enums)
	- [Interfaces and Generics](#interfaces-and-generics)
	- [Yielded Types](#enums)
	- [Assembly](#assembly)
	- [Types List](#types-list)
	- [Builtins List](#builtins-list)
	- [Valid Syntax](#valid-syntax)
- [Tutorials](#tutorials)
	- [My First Project](#my-first-project)
  - [Interfaces](#interfaces-tutorial)
- [Default Behavior](#default-behavior)
- [Projects](#projects)
- [Embedded](#embedded)
	- [Script Based](#script-based)
	- [Linux Based](#linux-based)
	- [Freestanding](#freestanding)
- [Minifier](#minifier)
- [Linter](#linter)
- [Compiler](#compiler)
- [Library](#library)

---
# Introduction


---
# Installation 

---
# Beginners

---
# Language

#### Program and Scope
A valid program is a program which syntax can be parsed by the `runner` `minifier` `linter` and `compiler`.
Every open parenthesis `(` must have a closing parenthesis `)`.
This will be referred to as a scope. Every `()` is a scope. A file is an implicit scope.
Every scope must have an expression. This can be achieved many ways.

- A declaration `@`
- A builtin `%`
- Another scope `()`
- Evaluation of a valid expression

examples of valid programs. Each line a separate program
```
() ; evaluates to nil
(()) ; nil evaluates to nil
(%+ 5 5) ; evaluates to 10
("valid") ; evaluates to "valid"
(@l myvar "valid") ; sets a variable on the owned scope, and evaluates to nil
```
examples of invalid programs.
```
( ; unexpected opening scope
()) ; unexpected closing scope
(,anything) ; unexpected identifier ,
```
Some scopes are owned scopes. Owned scopes maintain state. [types](#types), [functions](#functions), [generics](#interfaces-and-generics), and files are an owned scope.

See [Valid Syntax](#valid-syntax) for complete reference on valid program and scope syntax.

---
### Types
Everything that is evaluated has a type. Types are defined using the declaration `@` [identifier](#identifiers).

Type definition.
```
(@t computer 
  :mouse "")
```
`:mouse` is a [property](#properties).

Types can also have constructor arguments.
```
(@t computer (mouseName)
  :mouse mouseName)
```
You can strongly type constructor arguments and properties.
```
(@t computer 'string (mouseName)
  :mouse 'string mouseName)
```
Types can also have no properties, these are `type definitions`.
```
(@t special-string 'string)
```

Declaring a [variable](#variables) by type name. Types are referenced by the type indication `'` [identifier](#identifiers).
```
(@l mycomputer1 'computer)
(@l mycomputer2 'computer (:mouse "Gaming Mouse"))
```

For a complete list of types provided in the language see [types list](#types-list)

See [Valid Syntax](#valid-syntax) for complete reference on valid types syntax.

---
#### Variables
Variables are state that is stored on the owned [scope](#program-and-scope). Variables are defined using the declaration `@` [identifiers](#identifiers)
```
(@l myvar) 
```
You may use either of these declarations.
```
@l
@let
```
---
string examples:
``` 
(@l mystring1 "")
(@l mystring2 "this is a string.")
(@l mystring3 'string "strongly typed string")
```

---
number examples:
```
(@l mynum1 0)
(@l mynum2 0.0)
(@l mynum3 'sint -500) ; signed integer
```

All of the above are valid ways to declare a number. All numbers without a specific type will be 64 bit floating point. 
It is possible to use other types of numbers, see the complete [types list](#types-list)

---
list examples:
```
(@l mylist1 [])
(@l mylist3 [1 2 3])
(@l mylist4 'list ["my" "list" "four"])
(@l mylist5 ["my" "list" 5])
```

---
nil examples:
```
(@l myvar1)
(@l myvar2 ())
(@l myvar3 'nil)
(@l myvar4 'nil (()))
``` 
nil is both a type, and an evaluation. The first example is of type any. As it could later be assigned. The evaluation of an unassigned variable is nil.
See [Default Behavior](#default-behavior) to understand how this execution could impact your project.
  
See [Valid Syntax](#valid-syntax) for complete reference on valid type syntax.

---
#### Properties
Properties exist only on [types](#types). Since types are an owned [scope](#program-and-scopes), properties can access other properties in the same type.
```
(@t computer ()
  :monitors []
  :gpus []
  :peripheral-count (count (add :monitors :gpus)))
```

See [Valid Syntax](#valid-syntax) for complete reference on valid property syntax.

---
#### Identifiers
Identifiers are prepended to text. They have special meaning which can quickly inform the `parser` `linter` `compiler` what to expect next.
```
@ # $ % & * ; :  , ... . () ' " ? / | _
```
`@` - used for defining/declaring.

`;` - used for comments, everything up until the next line is immediately ignored. multi-line comments are up to your ide tooling to insert ; on every line to be commented

`#` - used for preprocessor commands.

`()` - scoping block. A file is an implicit scope block.

`:` - indicates properties on a type.

`,` - hook identifier. Used for implementing interfaces.

`%` - builtin identifier. These are functions that are embedded in the language.

`'` - type indication identifier.

`$` - reflection identifier.

`...` - variadic identifer.

`_` - rest identifier. Used in case matching 

`& * ? / | .` - are reserved for now.

---
#### Functions
Functions have their own [scope](#program-and-scope). They are declared with the declaration `@` [identifier](#identifiers)
```
(@f add (x y)
  (%+ x y)) 
```
You may use either of these declarations.
```
@f
@fn
@function
@func
```
Functions are used to manipulate [types](#types).
A function is valid if it evaluates to any type.
Functions are evaluated in this context `(operator operand operand)`

See [Valid Syntax](#valid-syntax) for complete reference on valid function syntax.

---
#### Builtins

Some functions are built into the language. Here is a list of all.
```
%ffi - used for calling functions local to the Operating System.
%thread - used for creating and working with threads.
%proc - used for creating a process.
%do - used for serially executing functions.
%if - used for conditionallity executing one of several blocks of code.
%while - loops while a condition is true.
%1while - executes the block of code at least once even if condition is false.
%loop - executes consuming an implicit iterator.
%arrow- allows for a function to be projected onto its parameters.
%main - special function which tells the program where to start
%+ - addition.
%- - subtraction.
%% - modulo.
%/ - divide.
%* - multiplication.
%< - less than.
%> - greater than.
%| - or.
%& - and.
%[ - shift left.
%] - shift right.
%~ - not.
%^ - xor.
%! - falsey. works the same as `not` on bits, but handles nil for types.
%= - equality. same as xor on bits, but handles nil for types.
```
Every builtin has an evaluation. Builtins are specific implementations for the arch and OS, but are guaranteed to behave the same way on any of the supported.

---
#### Enums
An enumeration is a [type](#types) which evaluates to a different type.
You define an enum with the declaration `@` [identifier](#identifiers)
```
(@e Directions 
  'NORTH
  'SOUTH
  'EAST
  'WEST)
```
Any of these declarations are valid.
```
@e
@enum
```
Defining an enum.
```
(@l current-direction1 'NORTH)
(@l current-direction2 'Direction'NORTH)
```
When executing code with enums, all possible outcomes must be defined.
Here is an example using the [builtin](#builtins) `%match`.
```
(@f turn-clockwise 'Direction (myparam)
  (%match myparam
    'NORTH myparam 'EAST 
    'SOUTH myparam 'WEST
    'EAST myparam 'SOUTH
    'WEST myparam 'NORTH))
```
See [functions](#functions), and [match](#match)


See [Valid Syntax](#valid-syntax) for complete reference on valid enum syntax.

---
#### Interfaces and Generics
Interfaces describe contracts that a [type](#types) must implement in order for them to be used with a generic.
You declare an interface with the declaration `@` [identifer](#identifiers)
```
(@i debug-it (param1)
  :debug param1)

(@i debug-it-strong 'string (param1)
  :debug param1)
```
The first interface can take any type in the `constructor args` and ensures that they get assigned to the `:debug` property.
The second interface specifies that this interface is strictly for `strings` in `:debug` property. 

See [Valid Syntax](#valid-syntax) for a complete reference on valid interface syntax. 

`constructor args` is used loosely here, as there is never going to be an instance of the interface.
Either of these declarations are valid:
```
@i
@interface
```
In order for a type to implement the interface, you must use the hook `,` [identifier](#identifiers).
```
(@t computer ,debug-it
  :mouse ""
  :debug "It's a computer")
```
The `computer` type must implement the `:debug` property. 

Generics are special [functions](#functions) which take interfaces and parameters in its constructor arguments. The interfaces do not need to be named.
The properties on the contract can be used by name directly. 
```
(@g print-out (,debug-it)
  (printf :debug))
```
The `print-out` function must use at least one `property` on the interface. See [Valid Syntax](#valid-syntax) for a complete reference on valid generics syntax.

Either of these declarations are valid:
```
@generic
@g
```
#### Yielded Types
**WIP**

```
(@e Directions 
  'CENTER
  'EAST
  'WEST)

(@e TimesOfDay
  'NIGHT
  'AFTERNOON
  'EVENING)

(@t panel
  :direction 'Directions'EAST)
```
```
(@y rotate-solar-panel 'EAST 'AFTERNOON (solarPanel sunLocation)
  (set-angle solarPanel 0))
(@y rotate-solar-panel 'WEST 'NIGHT (solarPanel sunLocation)
  (set-angle solarPanel 45)
```
Every combination for `rotate-solar-panel` must be defined.
so then the usage might look something like this. `@d` for dependent types.
```
(@d update-panels 'rotate-solar-panel 'Direction 'SunLocation (panel sunlocation)
  (rotate-solar-panel panel sunlocation))
```
and it's potential usage
```
(@l mypanel 'panel)
(%while (%= 1 1) 
  (update-panels mypanel (get-sun-location))
```
---
#### Assembly

The assembly syntax follows the new `asm!` syntax for rust as closely as possible. Writing assembly like this is probably one of the most pleasant experiences in writing assembly.
Here is an example.

** WIP **

// the implication of taking a type, and using some sort of reflection around 'reg and inout, and the fact you need to mark the command with #asm indicates this entire scoped block is unrelated to the rest of the language.

// might as well just use (reg), 'reg, or reg?
```
(add2 #asm (x)
  (%addi x x num)
  (%inout ('reg) x) ** wip** <--- what does it mean to take a 'type? is this the instantiation of this type like the rest of the language?
  (%c num 2))
```
First, the function must be marked with the preprocessor command `#asm`. Next we can use the builtins that are made for this specific arch.

`%addi` takes a `dst`, `src`, and `imm`.

`%inout` tells the compiler to use any register it chooses with ('reg), and then x is both an `in` and `out`. The compiler will successfully deduce that it can leave x in the same register, and it will be clobbered. The compiler keeps a list of all registers which are currently in use. if the `%out` or `%inout` is not specified for the variable/register, the compiler will put which ever register contained the variable x, back in the available pool to draw from.

With proper usage of the `%in %out %inout %inlateout %lateout` You are able to squeeze out the best performance possible.

---
#### Types List

---
#### Bultins List

---
#### Valid Syntax

---
# Tutorials

---
# Projects

---
# Embedded

---
# Minifier

---
# Linter

---
# Compiler

---
# Library

---
