# About
The DISC language is a lisp dialect based language. The language itself has many features that most developers are acustomed to in the modern era, as well as several new ones.
This project has goals that will ensure success and adoption of the language. The goals are...

- **Tooling is first class**. The language server, the cli, the parser, everything for a developer to be successful will all be included in decisions and apart of the main project.
- **Make the fastest interpreted language available**. Tools will be able to output optimal performance suggestions for interpreted scripts. The language is designed around executing scripts as fast as possible.
- **Provide the best embedded support**. Embedded systems require demanding performance use of their hardware. Supporting embedded systems will force the tooling we make, to be as efficient as possible, thus further helping the second goal.
- **Documentation is key**. Making documentation a number another high priority, will ensure that any newcomers to the language get the most up to date features,knowledge, and tutorials for easy use.

What does disc stand for?

- **D**ynamically (typed)
- **I**nterpreted
- **S**trongly (typed) 
- **C**ompiled language.

You read that correctly. The interpreter and compiler for disc can give you options to control the four pillars of static, dynamic, weak, and strong typing.

Binaries are not cross-platform, Scripts are cross-platform. Many developers work on different systems, and the tools output a binary that is specific to x86-64, riscv32, arm32, the list goes on.
In the business world for CICD, many developers would work on windows, but then pipelines or servers could be linux based, or if the company is fancy, they might even have their developers be on mac. We need to make the code that is developed, portable. 

There are two factions in the software building community. 

One side cares strictly about the correctness of the code, and has no issues enforcing strict type checking. Sometimes losing a faster feedback loop on how your code actually behaves, as more time is spent fighting the compiler. 

While the other side, has no problems with implicit casts, doesn't type check and allows you to run code. When working with javascript, I find its lack of typing nice at times when I just want to see the code run, inspect breakboints, variables in memory, etc. 

But, we also can't forgo the safety our programs receive in strongly typed systems. Rust has certainly taken the community by storm. This project will be no acception, there are plans to make the compiler type checking as strong as rusts type system.
#

**Disclaimer**

The rest of these sections are just to give the reader an understanding on what the expected language and tools should look and interact.

# Installation 
::todo::

# Tutorial

### Prerequisites
- You have downloaded and installed `discr`.
- You have downloaded and installed `discd`.

### Tooling Mindset
The `discr` or "disc runner" is the absolute minimum required cli necessary in order to run scripts. No repl, no validators, nothing. The way your operating system runs programs will explain why this decision was made to separate the runner. First, the OS must load the program into memory, and then it begins executing the `main` method. Anytime you run a script for disc, python, or any cli the first thing the operating system must do, is load the program itself into memory. The larger this binary is, the longer it takes before it can execute any actual code at all! 

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
- `new` is the feature the tool supports.
- `first-project` is the input that the feature requires.
- `--cli` is the flag that is passed to the feature in this case, here is where flags passed to the tool would go as well.
- `discd` as well as the tool:`project` and feature: `new` could be given an `-h` argument to get hints about how to use that tool or feature.

The output on the runner from the first example was not very helpful because it is meant to be lean. Run `discd runner -h`.

```
$ discd runner -h
version: 0.0.1-alpha
march: x86-64
os: linux
disc runner

usage:
discd runner [{array of scripts}] 
