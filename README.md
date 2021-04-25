# About
The DISC language is a lisp dialect based language. The language itself has many features that most developers are accustomed to in the modern era, as well as several new ones.
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

You read that correctly. The interpreter and compiler for disc can give you options to control the four pillars of static, dynamic, weak, and strong typing.

Binaries are not cross-platform, Scripts are cross-platform. Developers with disc will be able to choose to create minified scripts that can be ran on any machine, compiled binaries, or both. Many developers work on different systems, and the tools output a binary that is specific to x86-64, riscv32, arm32, the list goes on.
In the business world for CICD, many developers would work on windows, but then pipelines or servers could be linux based, or if the company is fancy, they might even have their developers be on mac. We need to make the code that is developed, portable. 

There are two factions in the software building community. 

One side cares strictly about the correctness of the code, and has no issues enforcing strict type checking. Sometimes losing a faster feedback loop on how your code actually behaves, as more time is spent fighting the compiler. 

While the other side, has no problems with implicit casts, doesn't type check and allows you to run code. When working with javascript, I find its lack of typing nice at times when I just want to see the code run, inspect breakboints, variables in memory, etc. 

But, we also can't forgo the safety our programs receive in strongly typed systems. Rust has certainly taken the community by storm. This project will be no exception, there are plans to make the compiler type checking as strong as rusts type system.
#

**Disclaimer**

The rest of these sections are just to give the reader an understanding on what the expected language and tools should look and interact.

# Installation 
::todo::

# Tutorials
- [Prerequisites](#prerequisites)
- [Tooling Mindset](#tooling-mindset)
- [First Project](#first_project)
- [Compilation](#compilation)
- Embedded Project
- [Language Specification (Weak + Dynamic)](#language-specification-weak-%2B-dynamic)

# Prerequisites
- You have downloaded and installed `discr`.
- You have downloaded and installed `discd`.

### Tooling Mindset
The `discr` or "disc runner" is the absolute minimum required cli necessary in order to run scripts. No repl, no validators, nothing. The way your operating system runs programs will explain why this decision was made to separate the runner. First, the OS must load the program into memory, and then it begins executing the `main` method. Anytime you run a script for disc, python, or any cli the first thing the operating system must do, is load the program itself into memory. The larger this binary is, the longer it takes before it can execute any actual code at all! You will find that all decisions were carefully made, and it is recommended to use the tooling provided to its full advantage.

The `discd` or "disc dev" cli is a full fledged development tool that can manage projects, minify scripts, compile projects, lint projects, provide language server options, and more.

### Working with Projects
Type in a shell.
```
discr -h
```
and you will see a list of help functions. there isn't much.
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
discd runner [<scripts>] [<args>]
  <scripts> 
    ./myscript.dc ./nextscript.dc
    space separated scripts. The first script will be treated as the first and main script.
	
  <args>
    -N, --no-initial		no initial size allocated of ram, this is the default. [--limits] arg ignored.	
    -G, --gb			initial size of allocated ram in gb.
    -M, --mb			initial size of allocated ram in mb.
    -K, --kb			initial size of allocated ram in kb.
    -L, --limit			the initial size of allocated ram is also the maximum. [--gb, --mb, --kb] must be included or ignored.
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

But if we supported something in the project yaml that would include the command to run. We know that we have an additional step in order to execute the cli, which goes against the fastest interpreter goal. First, `discr` would need to be loaded into memory, then discr would need to have all the code to load and parse a yaml file, parse the yaml file, and then load the scripts into memory, then begin execution.

In comes the `minifier`, it is the next tool in your arsenal.

```
$ discd minifier -h

usage:
discd minifier [<scripts>] [<args>]
  <scripts> 
    ./myscript.dc ./nextscript.dc
    space separated scripts. The first script will be treated as the first and main script.
	
  <args>
    -N, --no-initial		no initial size allocated of ram, this is the default. [--limits] arg ignored.	
    -G, --gb			initial size of allocated ram in gb.
    -M, --mb			initial size of allocated ram in mb.
    -K, --kb			initial size of allocated ram in kb.
    -L, --limit			the initial size of allocated ram is also the maximum. [--gb, --mb, --kb] must be included or ignored.
    --args			the args that will be passed to your scripts entrypoint. Make this the last argument passed to runner
```
Notice how the usage on the `minifier` tool is exactly the same as the `runner`.
The minifier will take all the scripts and args that it was given, and bake everything into one script. The args are additive. So if you wanted to minify the previous command. You would do so with.
```
$ discd minifier ./src/first-project.di ./utility-funcs.di .html-parser.di -G 20 --args google.com
```
This script which was first-project.di and the other two scripts will be minified into just one `out/first-project.min.di`

Now in order to run it, all we need is `discr ./out/first-project.min.di`. Let's pretend this project parses html files from websites that are passed in via `--args`. We already know from the `minifier` command ran previously, that `google.com` will be included, we can also pass in others at the runner, as `--args` is additive. 
```
$ discr ./out/first-project.min.di --args twitch.tv
...
...
Success! parsed html for { google.com, twitch.tv}
```

### Compilation
::todo::

### Language Specification Weak + Dynamic

**NOTE**
This section will not cover the type safety possibilities.
This is the bear minimum to run a program somewhat like what you expect.

---
#### Identifiers

In order for parsing of a script to execute faster, there are special characters that can't be used anywhere else. Many of them are reserved for future use, or are used internally to the compiler, parser, or minifier. They are prepended to text.
```
@ # $ % & * ; :  , ... . () ' "
```
`@` - used for defining/declaring.

`;` - used for comments, everything up until the next line is immediately ignored. multi-line comments are up to your ide tooling to insert ; on every line to be commented

`#` - used for preprocessor commands.

`()` - scoping block. A file is an implicit scope block.

`:` - used for keys, may also be referred to as properties.

`,` - hook identifier. Used for implementing interfaces.

`%` - builtin identifier. These are functions that are embedded in the language.

`'` - type indication identifier.

`$ & * ... .` - are reserved for now.

---
#### Functions
You define functions using `f` in conjunction with the `@` identifier:
```
@f is short for `function`
```
```
(@f add (x y)
  (%+ x y)) 
```
You can use a function with the name of the function:
```
$ discd repl ./myadd.di
> (add 33 9)
42
```
Note: You can read the [Builtin](#builtin) section to understand why `+` was prepended with `%`.

---
#### Builtin

Some functions are built into the language. Here is a list of all.
```
%ffi - used for calling functions local to the Operating System.
%thread - used for creating and working with threads.
%proc - reserved.
%do - used for serially executing functions.
%if - used for conditionallity executing one of two blocks of code.
%elif - used for conditionallity executing several blocks of code.
%while - loops while a condition is true.
%1while - executes the block of code at least once even if condition is false.
%loop - executes consuming an implicit iterator.
%lambda - allows for a function to be projected onto its parameters.  
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
%! - falsey. works the same as not on bits, but handles nil for types.
%= - equality. same
```
including builtins allows us to quickly parse these commonly used functions, and do optimizations at the compiler level.

---
#### Variables
You define variables using the identifier `@` along with `l`:
```
@l is short for `let`
```

---
string examples:
``` 
(@l mystring)
(@l mystring "")
(@l mystring "this is a string.")
```

All of the above are valid ways to declare a string. All these examples will live on the heap.

---
number examples:
```
(%l mynum)
(%l mynum 0)
(%l mynum 0.0)
```

All of the above are valid ways to declare a number. All numbers by default are 64 bit floating point.

---
list examples:
```
(%l mylist)
(%l mylist ())
(%l mylist (1 2 3))
(%l mylist ("hello" "there"))
(%l mylist ("general" 1 2 3))
```

All of the above are valid ways to declare a list. Lists do not need to be of the same type.

---
#### Types
You can define a type with the identifier for definitions `@` along with `t`, and give the type properties with the identifier  `:`
```
@t is short for type
```
```
(@t computer 
  :mouse ""
  :keyboard ""
  :cpu ""
  :monitors ()
  :speakers ())
```
You can use a type with its name. The type will live on the heap.
```
$ discd repl ./computer.di ./add-monitor.di
> (add-monitor (computer "Generic 720p Monitor"))
(@t computer
  :mouse ""
  :keyboard ""
  :cpu ""
  :monitors ("Generic 720p Monitor")
  :speakers ())
```

---
#### Interfaces and Generics
You can define an interface with the identifier for definitions `@` along with `i`:
```
@i is short for interface
```
```
(@i debug-it (o)
  :debug (o))
```
In order to add an interface to a type, you must use the identifier `,` for hooking in the requirement
```
(@t computer (,debug-it)
  :mouse ""
  :debug ("It's a computer"))
```
Then, you can make a generic which will take the interface prefaced with the hook `,` identifier.
```
@g is short for generic
```
```
(@g print-out (,debug-it)
  (printf :debug))
```
And finally, its usage.
```
$ discd repl ./computer.di ./print-out.di
> (print-out (computer))
It's a computer
```
These are a bit more complex, albeit powerful, so let's go over what is happening.

First,
we define an interface. When implementing it, you must consume 1 type, the previous example used `(o)`. This indicates it needs to be an expression of any type.
```
(@i debug-it o
  :debug o)

@t computer (,debug-it)
  :mouse ""
  :debug "It's a computer")
```
The above is saying that in order to implement the `debug-it` interface, you must have a `key` called `:debug`, and it must be a type. All expressions evaluate to a type. So these are both valid syntaxes. Visualizing the substitution I find is most helpful.

```
(@g print-out (,debug-it)
  (printf :debug))
```
Instead of print-out receiving a string, or a list, we are saying, there will be a key called `:debug` you can just use that. `:debug` will get expanded to "It's a computer".

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
So printf can take a list of strings. We can edit `debug-it` interface to represent this.
```
(@i debug-it (format input)
  :debug (format input))

(@g print-out (,debug-it)
  (printf :debug))

(@t computer (,debug-it)
  :mouse ""
  :debug ("mouse key is %s\n" :mouse))
```
types are special in that they have access to their `keys` anywhere within the declared `type` scope.
`printf` is seemingly very robust, and we want to be able to pass in any of the accepted arguments, but you are in the `dynamic` and `weak` tutorial. So we could, just force `printf` to figure out all this for us.

Here is the full look at generics and interfaces in action.
```
(@i debug-it o
  :debug o)

(@g print-out (,debug-it)
  (printf :debug))

(@t computer (,debug-it)
  :mouse ""
  :debug "It's a computer")

(@t car (,debug-it)
  :make ""
  :model ""
  :debug ("car => :make %s and :model %s\n" :make :model))
```
and the usage.
```
$ discd repl ./full-example.di
> (print-out computer)
It's a computer
> (@l mycar (:make "Ford" :model "RS200"))
> (print-out mycar)
car => :make Ford and :model RS200
>
```

