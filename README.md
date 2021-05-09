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

You read that correctly. The interpreter and compiler for disc can give you options to control the four pillars of static and dynamic type analysis, weak and strong typing.

Binaries are not cross-platform, Scripts are cross-platform. Developers with disc will be able to choose to create minified scripts that can be ran on any machine, compiled binaries, or both. Many developers work on different systems, and the tools output a binary that is specific to x86-64, riscv32, arm32, the list goes on.
In the business world for CICD, many developers would work on windows, but then pipelines or servers could be linux based, or if the company is fancy, they might even have their developers be on mac. We need to make the code that is developed, portable. 

There are two factions in the software building community. 

One side cares strictly about the correctness of the code, and has no issues enforcing strict type checking. Sometimes losing a faster feedback loop on how your code actually behaves, as more time is spent fighting the compiler. 

While the other side, has no problems with implicit casts, doesn't type check and allows you to run code. When working with javascript, I find its lack of typing nice at times when I just want to see the code run, inspect breakboints, variables in memory, etc. 

We also can't forgo the safety our programs receive in strongly typed systems. Rust has certainly taken the community by storm. Disc offers many of the same features, and has more powerful type handling as you will see.

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
Every scope must have an evaluation. This can be achieved many ways.

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

---
### Types
Everything that is evaluated has a type. Types are defined using the declaration `@` [identifier](#identifiers)
```
@t
@type
```
Types are referenced by the type indication `'` [identifier](#identifiers).

Type definition.
```
(@t computer 
  :mouse "")
```
`:mouse` is a [property](#properties).

Types can also have `constructor arguments`.
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

Declaring a [variable](#variables).
```
(@l mycomputer1 'computer)
(@l mycomputer2 'computer (:mouse "Gaming Mouse"))
```

For a complete list of types in the language see [types list](#types-list)

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

---
#### Properties
Properties exist only on [types](#types). Since types are an owned [scope](#program-and-scopes), properties can access other properties in the same type.
```
(@t computer ()
  :monitors []
  :gpus []
  :peripheral-count (count (add :monitors :gpus)))
```

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
You may use the declaration `@` [identifier](#identifiers)
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
All enum evaluations must be used.
You can use `%match` in order to match on enums.
```
(@f turn-clockwise 'Direction (myparam)
  (%match myparam
    'NORTH myparam 'EAST 
    'SOUTH myparam 'WEST
    'EAST myparam 'SOUTH
    'WEST myparam 'NORTH))
```
See [functions](#functions), and [match](#match)

---
#### Interfaces and Generics
You can define an interface as follows:
```
(@i debug-it (param1)
  :debug param1)

(@i debug-it-strong 'string (param1)
  :debug param1)
```
The first interface can take any type in the `constructor args` and ensures that they get assigned to the `:debug` property.
The second interface declares that this interface is strictly for `strings` in `:debug` property.

`constructor args` is used loosely here, as there is never going to be an instance of the interface.
You may also use either of these declarations:
```
@i
@interface
```
In order to add an interface to a type, you must use the identifier `,` for `hooking` in the requirement
```
(@t computer ,debug-it
  :mouse ""
  :debug "It's a computer")
```
Notice how `,debug-it` is not in its `constructor args`. That is because, there is no variable being passed into `computer`. This is just a contract, that the `computer` type should implement the `:debug` property.

Now, you can make a generic function which will take the interface prefaced with the hook `,` identifier.
```
(@g print-out (,debug-it)
  (printf :debug))
```
This generic function states that the first variable passed to `print-out` must implement the `debug-it` interface.
You may use either of these declarations:
```
@generic
@g
```
And finally, its usage.
```
$ discd repl ./computer.di ./print-out.di
> (@l mycomp 'computer)
> (print-out mycomp)
It's a computer
```
These are a bit more complex, albeit powerful, so let's go over what is happening.

First,
we define an interface, you must declare any types it uses in its constructor.
```
(@i debug-it (o)
  :debug o)
```
The above example requires any type, and will be referenced as `o`, and that type `o` is on the `:debug` property.
```
@t computer ,debug-it
  :mouse ""
  :debug "It's a computer")
```
The above is saying that in order to implement the `debug-it` interface, you must have a `property` called `:debug`. This example correctly implements the `debug-it` interface.

```
(@g print-out (,debug-it)
  (printf :debug)) ; <-- :debug is just going to get replaced
			  with "It's a computer".  
```
Instead of print-out receiving a string, or a list, we are saying, there will be a `property` called `:debug` you can just use whatever is evaluated in that.

`printf` is one of the earliest functions in computing history. It is extremely complex internally, and lives on your system in some way or another. On linux or mac, you can enter this into your shell.
```
$ printf "hello\n"
hello
$
```
`printf` can also take in multiple arguments and format the input string with some evaluation.
```
$ printf "hello %s\n" "world"
hello world
$ 
```
So `printf` can take a list of strings. We can edit `debug-it` interface to represent this.
```
(@i debug-it 'string 'string (format input)  
  :debug 'list [format input])

(@g print-out (,debug-it)
  (printf :debug))

(@t computer ,debug-it
  :mouse ""
  :debug 'list ["mouse key is %s\n" :mouse])
```
types are special in that they have access to their `properties` anywhere within the declaration scope. This is why the list in `:debug` can access `:mouse`

`printf` is robust. How do we ensure that we use printf as it is intended? Since you are in the `dynamic` and `weak` tutorial, we are going to let `printf` do the heavy lifting. Sure, you might accidentally use printf incorrectly, pass it 20 arguments? 30 arguments? Will it break? See the `strong` and `static` tutorial to learn about how to do all this safely.

Here is the full look at generics and interfaces in action.
```
(@i debug-it (o)
  :debug o)

(@g print-out (,debug-it)
  (printf :debug))

(@t computer (,debug-it)
  :mouse ""
  :debug "It's a computer")

(@t car ,debug-it
  :make ""
  :model ""
  :debug ["car => :make %s and :model %s\n" :make :model])
```
printf is doing a bit of work here, as we are passing it a string in one instance, and a list in another.
 
Here is the usage. `full-example` contains all the code from the previous code block.
```
$ discd repl ./full-example.di
> (@l mycomp 'computer)
> (print-out mycomp
It's a computer
> (@l mycar ('car :make "Ford" :model "RS200"))
> (print-out mycar)
car => :make Ford and :model RS200
>
```

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

## Tooling Mindset
The `discr` or "disc runner" is the absolute minimum required cli necessary in order to run scripts. No repl, no validators, nothing. The way your operating system runs programs will explain why this decision was made to separate the runner. First, the OS must load the program into memory, and then it begins executing the `main` method. Anytime you run a script for disc, python, or any cli the first thing the operating system must do, is load the program itself into memory. The larger this binary is, the longer it takes before it can execute any actual code at all! 

Making `discr` as small as possible ensures the fastest execution possible.

The `discd` or "disc dev" cli is a full fledged development tool that can manage projects, minify scripts, compile projects, lint projects, provide language server options, and more. It is recommended to use this to run scripts while developing and testing your code.

### Working with Projects
Type in a shell.
```
discr -h
```
and you will see a list of help functions. There isn't much.
```
$ discr -h
version: 0.0.1-alpha
march: x86-64
os: linux
disc runner 
```
let's make our first script. You can use `discd` as a full fledged development tool.
```
$ discd project new my-first-project --cli
```

there is a lot to unpack with the tooling provided.
```
                    |-input expected by feature
$ discd project new first-project --cli
        ^-tool  ^                    ^- any flags passed to either the tool or feature.
                |-feature of the tool
                     
```
- `project` is the tool being used.
- `new` is the feature the tool supports. A tool can have many features or none.
- `first-project` is the input that the feature requires.
- `--cli` is the flag that is passed to the feature in this case, here is where flags passed to the tool would go as well.
- `discd` as well as the tool:`project` and feature: `new` could be given an `-h` argument to get hints about how to use that tool or feature.

let's change directories to our new project.
```
$ cd first-project
```

The output on the runner from the first example was not very helpful because it is meant to be lean. Run `discd runner -h`.

```
$ discd runner -h
version: 0.0.1-alpha
march: x86-64
os: linux
disc runner

usage:
discd runner [<scripts>] [<args>] [<script-args>]
  <scripts> 
    ./myscript.dc ./nextscript.dc
    space separated scripts. The first script will be treated as the first and main script.
	
  <args>
    -N, --no-initial		no initial size allocated of ram, this is the default. [--limits] arg ignored.	
    -g, --gb			initial size of allocated ram in gb.
    -m, --mb			initial size of allocated ram in mb.
    -k, --kb			initial size of allocated ram in kb.
    -l, --limit			the initial size of allocated ram is also the maximum. [--gb, --mb, --kb] must be included or ignored.
  <script-args>
    --args			the args that will be passed to your scripts entrypoint. Make this the last argument passed to runner
```

Now that we can see how to actually run our scripts, let's go ahead and run our new project.
```
$ discr ./src/first-project.di --no-initial
Hello from first-project!
```
Let's now take a look at the project structure that was created.
```
first-project
  |
  |--src
      |
      |--first-project.di
  |--project.yaml
```
One of the goals of this project is to make the tooling easier to use. We already know that we can pass a series of scripts to the `runner`. This would be annoying and frustrating to do everytime we want to execute a command like  `discr ./src/first-project.di ./utility-funcs.di ./html-parser.di -G 20 --args google.com`. 

But if we supported something in the project yaml that would include the command to run. There would be an additional step in order to execute the cli, which goes against the fastest interpreter goal. First, `discr` would need to be loaded into memory, then discr would need to have all the code to load and parse a yaml file, parse the yaml file, and then load the scripts into memory, then begin execution.

In comes the `minifier`, it is the next tool in your arsenal.

```
$ discd minifier -h

usage:
discd minifier [<scripts>] [<args>] [<script-args>]
  <scripts> 
    ./myscript.dc ./nextscript.dc
    space separated scripts. The first script will be treated as the first and main script.
	
  <args>
    -N, --no-initial		no initial size allocated of ram, this is the default. [--limits] arg ignored.	
    -g, --gb			initial size of allocated ram in gb.
    -m, --mb			initial size of allocated ram in mb.
    -k, --kb			initial size of allocated ram in kb.
    -l, --limit			the initial size of allocated ram is also the maximum. [--gb, --mb, --kb] must be included or ignored.
  <script-args>
    --args			the args that will be passed to your scripts entrypoint. Make this the last argument passed to runner
```
Notice how the usage on the `minifier` tool is exactly the same as the `runner`.
The minifier will take all the scripts and args that it was given, and bake everything into one script. The args are additive. So if you wanted to minify the previous command. You would do so with.
```
$ discd minifier ./src/first-project.di ./utility-funcs.di ./html-parser.di -G 20 --args google.com
```
This script which was first-project.di and the other two scripts will be minified into just one `./out/first-project.min.di`.

Now in order to run it, all we need is `discr ./out/first-project.min.di`. This project parses html files from websites that are passed in via `--args`. We already know from the `minifier` command ran previously, that `google.com` will be included. We can also pass in other websites at the runner, as `--args` is additive. 
```
$ discr ./out/first-project.min.di --args twitch.tv
...
...
Success! parsed html for { google.com, twitch.tv}
```

### Compilation

---
### Embedded Project

**Prerequisites**

You have at least read 
- [First Project](#first-project)
- [Language Specification (Weak and Dynamic)](#language-specification-weak-and-dynamic)
- [Language Specification (Strong and Static)](#language-specification-strong-and-dynamic)
- [Compilation](#compilation)

You have to first determine what are the goals of your project.
Depending on the resources provided by your Board/Microcontroller, you may need to make a choice about how your project is laid out.


- disc allows the developer to write assembly using builtins.
- disc allows freestanding binaries devoid of any OS/Kernel.
- disc can be interpreted or compiled.
- `discr` could be installed on the board if there is a linux kernel and run scripts.
- disc compiled binaries (different from freestanding) could be installed on the board if there is a linux kernel and ran without the interpreter.

The last bullet point is usually the easiest route, and works for most embedded projects. This option will be referred to as a `(linux based)` embedded project 

If there is no linux kernel, or you do not wish to use a kernel, you will probably choose the second bullet point. This will be referred to as a `(freestanding)` embedded project.  

If you are using a rasberry pi, or other small single board computers, there is probably enough resources to run `discr` on the device. This will be referred to as a `(script based)` embedded project.

Let's set up an embedded project first, and then the tutorials will guide you based on one of those three options from above.

#### First Embedded Project

::todo::

Here is the tutorial .

- [Script Based Embedded](#script-based-embedded)
- [Linux Based Embedded](#linux-based-embedded)
- [Freestanding Embedded](#freestanding)

#### Script Based Embedded

#### Linux Based Embedded

#### Freestanding Embedded

You are the cool kid on the block. You want to understand how computers work down to every last bit in the machine. First, you have to see if your arch is supported. If it's not supported, its quite easy to add. It only requires a PR into disc to support it.

Assembly files are designated with their arch preceding the file extension `file-name.{arch}.di`

First add the arch to your project, this will ensure that when you use the `project` tool to generate a file, it will automatically generate all file extensions
```
$ discd project add arch --riscv32
$ discd project gen interrupts --feature
```
You will now have a folder and file created for riscv32 

```
first-project
  |
  |--src
      |--interrupts
          |--interrupts.riscv32.di
      |--first-project.di
  |--project.yaml
```

#### Assembly Syntax
The assembly syntax follows the new `asm!` syntax for rust as closely as possible. Writing assembly like this is probably one of the most pleasant experiences in writing assembly.
Here is an example.
*** WIP ***
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

### Language Specification Weak and Dynamic

**NOTE**
This section will not cover the type safety possibilities.
This is the bear minimum to run a program somewhat like what you expect. The `discr` tool will run the code provided, so long as its syntax makes a valid program. `((()))` is a valid program. `(()` is not a valid program. 

In many other dynamically typed languages, type checking will happen at run time. This has a performance implication, and this can easily be avoided if the developer who prefers a Dynamic and Weak understands the way the implicit conversions work. All type conversions and type checking will be discussed in the sections as they come up. These sections will be titled **Runtime Behavior**. This language has a `default` way of executing programs with the runner, and this will not change. You will find that the runner makes decisions about your programs in real time. That is why it is important to discuss how the runner will behave with regards to `weak and dynamic` execution. 

---
#### Identifiers

In order for parsing of a script to execute faster, there are special characters that can't be used anywhere else. Many of them are reserved for future use, or are used internally to the compiler, parser, or minifier. They are prepended to text.
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

`_` - rest identifier. used in casematching to catch the remaining possibilities.

`& * ? / | .` - are reserved for now.

---
#### Functions
You define functions as follows:
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
You can use a function with the name of the function. `myadd` file contains the previous function definition.
```
$ discd repl ./myadd.di
> (add 33 9)
42
```
Note: You can read the [Builtins](#builtins) section to understand why `+` was prepended with `%`.

---
#### Builtins

Some functions are built into the language. Here is a list of all.
```
%ffi - used for calling functions local to the Operating System.
%thread - used for creating and working with threads.
%proc - reserved.
%do - used for serially executing functions.
%if - used for conditionallity executing one of two blocks of code.
%ifelif - used for conditionallity executing several blocks of code.
%while - loops while a condition is true.
%1while - executes the block of code at least once even if condition is false.
%loop - executes consuming an implicit iterator.
%lambda - allows for a function to be projected onto its parameters.
%box - special function which puts the type or function on the heap and returns the boxed object.
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
including builtins allows us to quickly parse these commonly used functions, knowing immediately that an enclosing scope is a builtin, allows us to parse and build into an AST quickly.

---
#### Variables
You define variables using as follows:
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
(@l mystring)
(@l mystring2 "")
(@l mystring3 "this is a string.")
(@l mystring4 'string "strongly typed string")
```

All of the above are valid ways to declare a variable. The type of the first variable `mystring` is not yet known, we can say it the type `'unknown`.

---
number examples:
```
(@l mynum1) 
(@l mynum2 0)
(@l mynum3 0.0)
(@l mynum4 'float -500)
```

All of the above are valid ways to declare a number. All numbers without a specific type will be 64 bit floating point. 
It is possible to use other types of numbers, see [Language Specification Strong and Static](#language-specification-strong-and-static)

**Runtime Behavior**

Now that you know strings and numbers, what would happen if you had a function like `add` that expected two numbers, and a string was passed in. like this.

```
(@f add (x y)
  (%+ x y))

(@l mystring "72")
(@l mynum 22)
```

`mynum` is a primative type. It is a 64 bit floating point type. `mystring` is a string where each character is 1 byte. Each character takes up more space than a digit would to account for all alphanumeric numbers.

 In javascript, `"72" + 22` evaluates to "7222". This may or may not be intended by the developer. Instead of choosing the default behavior that javascipt has for types, the default implementation will work more with the `c standard`.

`disc` treats all types as their binary representatives. It was decided to follow the `c` standard and every character directly maps to the ascii table, that is defined by c. The function `atof` `ascii to float` is used to implicitly cast a string to a float. This would ultimately make both types a float, there is an overloaded set of parameters for the builtin `%+` that takes in two floats, and returns a float, so this function would yield `94`. `%+` also has an overload that takes two strings. This actually does return a string with concatenation, `(%+ "hello " "strings")` returns `"hello strings"`.   

---
list examples:
```
(@l mylist1)	; <-- nil
(@l mylist2 ()) ; <-- nil
(@l mylist3 [1 2 3])
(@l mylist4 'list ["hello" "there"])
(@l mylist5 ["general" 1 2 3])
```

All of the above are valid ways to declare a list. Lists do not need to be of the same type. `()` and `(@l mynum)` evaluate to nil.

**Runtime Behavior**

Now that we know about declaring a variable `@l`. We can briefly discuss a different type of declaration `@c`. This will be covered more in detail in the [Language Specification Weak and Dynamic](#language-specification-weak-and-dynamic)
, but essentially it means `const` or constant in other languages. Once the value is set, it is impossible to change the value. But during runtime, the discrunner just treats `@c` as a normal defined variable. We allow some sort of static analysis if the developer does not want this behavior. You can learn more about each flag necessary to pass to the linter to disallow reassignment of const variables. Hopefully, this is now clear, that the goal of the runner, is to parse a valid program, then execute it as fast as possible. The behavior has a default pattern, if you want to enforce more type safety, this is the job for static analysis tools.

---
#### Enums

You may use either of these declarations
```
@e
@enum
```
Here is how to define an enum
```
(@e Directions 
  'NORTH
  'SOUTH
  'EAST
  'WEST)
```
Defining an enum is as easy as.
```
(@l current-direction 'NORTH)
```
You can use `%match` in order to match on enums.
```
(@f turn-clockwise 'Direction (myparam)
  (%match myparam
    'NORTH myparam 'EAST 
    'SOUTH myparam 'WEST
    'EAST myparam 'SOUTH
    'WEST myparam 'NORTH))
```
And its usage. Let's assume the enum definition, the variable current-direction is declared, and the function is declared in `enum-example.di`
```
$ discr ./enum-example.di ./print.di
>(print-out (turn-clockwise current-direction))
'EAST
>(print-out myparam)
'EAST
```
Notice how in this example `myparam` was assigned the value, and then `myparam` was an implicit return to the caller.
Similarly, you could have the function declared as so,
```
(@f look-right 'Direction (myparam)
  (%match myparam
    'NORTH 'EAST 
    'SOUTH 'WEST
    'EAST 'SOUTH
    'WEST 'NORTH))
```
This will not change the value of myparam.
Its usage.
```
$ discr ./enum-example.di ./print.di
>(print-out (look-right current-direction))
'EAST
>(print-out myparam)
'NORTH
```

**Runtime Behavior**

In the first example we declared a variable with the `'NORTH` value, If there were two different enums that had a `'NORTH` values in each, it would not be possible to know which one it was. The runner will evaluate to whichever definition the parser found first. It is best to be explicit `'Direction'NORTH`.

Not every path could be provided to the `%match` builtin. In the [Language Specification Strong and Static](#language-specification-strong-and-static) you will see how this can be enforced to ensure pattern matching must complete every outcome. The runtime behavior is quite clear. If a pattern is not matched on, the return on match, will be `nil`.

---
#### Types
You may use either of these declarations
```
@t
@type
```
Here is how to define a type
```
(@t computer 
  :mouse ""
  :monitors ()
  :speakers ())
```
Type definitions stray from normal list syntax a little bit. In the above example, `:mouse` is a string much like how we use the `@l mystring ""` syntax. `:monitors` and `:speakers` evaluate to `nil`
Declaring a variable with a type is as easy as.
```
(@l mycomputer1 'computer)
(@l mycomputer2 'computer (:mouse "Gaming Mouse"))
(@l mycomputer3 'computer ("Gaming Mouse" ["Default 1" "Default 2"]))
```
You can use a type with its name. Let's create a file called `computer.di`
```
(@t computer 
  :mouse ""
  :monitors ()
  :speakers ())

(@f add-monitor (comp monitor)
 (comp :monitors (add (comp :monitors) monitor)))
```

In a moment we will talk about the `print-out` functions.
```
$ discd repl ./computer.di ./print.di
> (@l mycomp 'computer)
> (print-out mycomp)
('computer 
  :mouse ""
  :monitors ()
  :speakers ())
> (print-out (add-monitor mycomp "Generic 720p Monitor"))
('computer
  :mouse ""
  :monitors ["Generic 720p Monitor"])
  :speakers "")
```
In this example the add-monitor function returns the same instance of the computer to the caller `print-out` which pretty prints the type to stdout.
And finally, it is possible to provide `constructor args`
```
(@t computer 'string 'list (mouse monitors)
  :mouse mouse
  :monitors monitors)
```
This says that any instance of computer could be provided initial values, and to where they should be assigned.

**Runtime Behavior**

What were to happen if we were to pass some new type `'vehicle` to `add-monitor`? The runtime behavior here, is much like javascript, except there is no prototypal inheritance, either the property exists on the object or it does not. so lets say we have a new `'vehicle` type, and is defined as so in `vehicle.di`.
```
(@t vehicle
  :wheels [])

(@l myvehicle 'vehicle)
```
```
$ discd repl ./vehicle.di ./computer.di ./print.di
> (print-out (add-monitor myvehicle "Dashboard LED Screen"))
('vehicle 
  :wheels []
  :monitors ["Dashboard LED Screen"])
```
Another behavior that needs to be considered is of unknown type or even a number, how would this work?
```
$ discd repl ./vehicle.di ./computer.di ./print.di
> (@l mything)
> (@l mynum 22)
> (print-out (add-monitor mything "what?"))
('unknown
  :monitors ["what?"])
> (print-out (add-monitor mynum "this is odd"))
('unknown
  :monitors [])
```

---
#### Interfaces and Generics
You can define an interface as follows:
```
(@i debug-it (param1)
  :debug param1)

(@i debug-it-strong 'string (param1)
  :debug param1)
```
The first interface can take any type in the `constructor args` and ensures that they get assigned to the `:debug` property.
The second interface declares that this interface is strictly for `strings` in `:debug` property.

`constructor args` is used loosely here, as there is never going to be an instance of the interface.
You may also use either of these declarations:
```
@i
@interface
```
In order to add an interface to a type, you must use the identifier `,` for `hooking` in the requirement
```
(@t computer ,debug-it
  :mouse ""
  :debug "It's a computer")
```
Notice how `,debug-it` is not in its `constructor args`. That is because, there is no variable being passed into `computer`. This is just a contract, that the `computer` type should implement the `:debug` property.

Now, you can make a generic function which will take the interface prefaced with the hook `,` identifier.
```
(@g print-out (,debug-it)
  (printf :debug))
```
This generic function states that the first variable passed to `print-out` must implement the `debug-it` interface.
You may use either of these declarations:
```
@generic
@g
```
And finally, its usage.
```
$ discd repl ./computer.di ./print-out.di
> (@l mycomp 'computer)
> (print-out mycomp)
It's a computer
```
These are a bit more complex, albeit powerful, so let's go over what is happening.

First,
we define an interface, you must declare any types it uses in its constructor.
```
(@i debug-it (o)
  :debug o)
```
The above example requires any type, and will be referenced as `o`, and that type `o` is on the `:debug` property.
```
@t computer ,debug-it
  :mouse ""
  :debug "It's a computer")
```
The above is saying that in order to implement the `debug-it` interface, you must have a `property` called `:debug`. This example correctly implements the `debug-it` interface.

```
(@g print-out (,debug-it)
  (printf :debug)) ; <-- :debug is just going to get replaced
			  with "It's a computer".  
```
Instead of print-out receiving a string, or a list, we are saying, there will be a `property` called `:debug` you can just use whatever is evaluated in that.

`printf` is one of the earliest functions in computing history. It is extremely complex internally, and lives on your system in some way or another. On linux or mac, you can enter this into your shell.
```
$ printf "hello\n"
hello
$
```
`printf` can also take in multiple arguments and format the input string with some evaluation.
```
$ printf "hello %s\n" "world"
hello world
$ 
```
So `printf` can take a list of strings. We can edit `debug-it` interface to represent this.
```
(@i debug-it 'string 'string (format input)  
  :debug 'list [format input])

(@g print-out (,debug-it)
  (printf :debug))

(@t computer ,debug-it
  :mouse ""
  :debug 'list ["mouse key is %s\n" :mouse])
```
types are special in that they have access to their `properties` anywhere within the declaration scope. This is why the list in `:debug` can access `:mouse`

`printf` is robust. How do we ensure that we use printf as it is intended? Since you are in the `dynamic` and `weak` tutorial, we are going to let `printf` do the heavy lifting. Sure, you might accidentally use printf incorrectly, pass it 20 arguments? 30 arguments? Will it break? See the `strong` and `static` tutorial to learn about how to do all this safely.

Here is the full look at generics and interfaces in action.
```
(@i debug-it (o)
  :debug o)

(@g print-out (,debug-it)
  (printf :debug))

(@t computer (,debug-it)
  :mouse ""
  :debug "It's a computer")

(@t car ,debug-it
  :make ""
  :model ""
  :debug ["car => :make %s and :model %s\n" :make :model])
```
printf is doing a bit of work here, as we are passing it a string in one instance, and a list in another.
 
Here is the usage. `full-example` contains all the code from the previous code block.
```
$ discd repl ./full-example.di
> (@l mycomp 'computer)
> (print-out mycomp
It's a computer
> (@l mycar ('car :make "Ford" :model "RS200"))
> (print-out mycar)
car => :make Ford and :model RS200
>
```

---
### Language Specification Strong and Static
#### Prerequisites
You have at least read [Language Specification Weak and Dynamic](#language-specification-weak-and-dynamic)

#### Variables Pt. 2
You have possibly scoffed at some of the unreliable non type safe way of doing things in that tutorial. Let's first revist the variables section, now with type safety.
You can still declare variables the same way.
```
(@l mystring1)
```
However, this prevents a problem, what if we assign `mystring1` to a number. what if later we reassign `mystring1` to a string?

With the `--static` flag passed to either the `linter` or `compiler` this is no longer possible. you will get an error. If you recall the default behavior of the runner is to just execute, so there is no way to pass any of these flags. The default behavior of the linter and compiler is the same. Passing `--static` and `--strong` ensure the absolute maximum static analysis possible. It can be confusing which one enables what feature, the language itself right now is unclear as well. It is usually best to assume those two are synonymous and should be grouped together.

Take for example this file `./mytest.di`
```
(@l mystring1)
```
```
$ discd linter ./mytest.di --static --strong
errors:
  ./mytest.di - (ln 0, p 13)		: type-not-deduced (d0)
```
You can enable undeduced types by passing the name of the error or the id of the error so either `--type-not-deduced` or `d0` will allow this behavior.
Be careful about which flags you choose to enable on your projects.

Variables can also be marked as constant, once their initial value is assigned, they can never be reassigned again.
```
(@c mystring "Hello")
(mystring "Constant")
```
```
$ discd compiler ./mytest.di --static --strong
errors:
  ./mytest.di - (ln 1, p 10)		: const-reassignment (d2)
```

---
#### Numbers pt 2.

Here is a quick table to describe the number types supported.

|type  |size(bits)|range							|explanation|
|-------|-----------|-------------------|-----------|
|'mask	|8					|na									|typically used for bit manipulations, range is irrelevant|
|'char	|8					|na									|typically used for characters, range is irrelevant|
|'sbyte	|8					|[-127, +127]				|different type than char, guaranteed to be signed|
|'ubyte	|8					|[0,255]						|different type than char, guaranteed to be unsigned|
|'sshort|16					|[-32,767, +32,767]	|short signed integer|
|'ushort|16					|[0,+65,535]				|short unsigned integer|
|'sint	|32					|[-2,147,483,647, +2,147,483,647]|medium signed integer|
|'uint	|32					|[0,+4,294,967,295]	|medium unsigned integer|
|'slong	|64					|[-9,223,372,036,854,775,807, +9,223,372,036,854,775,807]|long signed integer|
|'ulong	|64					|[0, +18,446,744,073,709,551,615]|long unsigned integer|
			
Rather than provide `C` compatability, each arch is guaranteed to be sized appropriately.
You can declare variables as follows.
```
(@l myint1 'sint 2147483647)
(@l myint2 'uint) ; <-- all bits set to 0 if never set.
```

---
#### Enums Pt. 2
In the first discussion about enums, we covered declaration, and made a function. Here is the example again,

```
(@e Directions 
  'NORTH
  'SOUTH
  'EAST
  'WEST)

(@f turn-clockwise 'Direction (myparam)
  (%match myparam
    'NORTH myparam 'EAST 
    'SOUTH myparam 'WEST
    'EAST myparam 'SOUTH
    'WEST myparam 'NORTH))
```
Enums with static typing, are required to solve for all possible inputs. You may provide an `_` to indicate the rest of all possible types.
```
(@f turn-east 'Direction (myparam)
  (%match myparam
    'EAST myparam
    _ myparam 'EAST))
```
This is a contrived example, at minimum it looks like it saves an unnecessary assignment if `myparam` happens to be facing east already, at the expense using a branch check.
The actual type of object that the enum `Direction` is, is a type that must evaluate to a different type. Meaning the longform value of `myparam` in the above example is.
```
'Direction'EAST
```
`'EAST` is also a type. We can use `'EAST` as a parameter to a function as well.
```
(@f rotate-solar-panel 'EAST 'AFTERNOON (solarPanel sunLocation) ; <--- ::todo:: should it look something like this?
    (angle solarPanel 0))
```
::todo:: maybe use `@d` for dependent types identifier. could this be worked into interfaces to set the required types up ie, sunLocation doesn't matter if its morning, and east.

so then the usage might look something like this.
```
(@f update-panels 'rotate-solar-panel 'Direction 'SunLocation (panel sunlocation)
  (rotate-solar-panel panel sunlocation))
```
and it's potential usage
```
(@l mypanel)
(@l sun)
(%while (%= 1 1) 
  (update-panels mypanel (get-sun-location))
```

#### Mutability and the Borrow Checker
::todo::

