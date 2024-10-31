# Ven: A Modern, Lightweight IR Language

**Ven** is an advanced, ultra-efficient Intermediate Representation (IR) language engineered to deliver exceptional performance and minimal runtime footprint. By directly transpiling to assembly code, Ven avoids unnecessary C/stdlib dependencies, producing extremely compact and lightning-fast executables without compromise.

> #### This project uses NASM as the assembler, and its license can be found here: **[License-NASM](tools/LICENSE-NASM)**

> #### Ven (this project) is under a custom license called ***[Ven License](LICENSE)***

### Key Features

* **Direct Assembly Transpilation:** Ven bypasses traditional compilation stages, generating highly optimized assembly code that runs close to the hardware, maximizing speed and efficiency.
* **Independence from C/stdlib:** Ven is designed to be entirely self-contained, eliminating the typical dependencies that can slow down runtime performance, resulting in remarkably small binaries.
* **Concise and Clear Semantics:** Ven prioritizes simplicity and clarity, providing a streamlined syntax that enables developers to write high-performance code with ease.
* **Extensible, Modular Architecture:** Ven’s adaptable design allows for the development of custom architectures, making it possible to target specific hardware platforms for optimized results.



## Ven Syntax:
- #### Variables
    - use ``@`` to define **static variables**
    - use ``!@`` to define variables that can change , aka **mutable variables**
    - there are 3 types of variables for now and they are ``i`` for integer , ``f`` for float and ``str`` for string 
    - variables can be declared in the following way : ``@ name str jay`` **notice how you dont provide ``=`` or `'` as they have special meanings in ven and ven auto adds ``'`` by checking the type!**
- #### Printing
    - you can print to stdout using `>` followed by the text , here **you need to add `'` around text and separate variables and text using , but make sure to close the text , new line character is the **Assembly** language way so ``0x0A``
    - example - ``> 'hello world',0X0A``


---

### ***How do you use ven in your own language?***
> #### To use ven for your own language you need to have a token system or some system through which you can generate a single .ven file and pass it to the ven compiler (have the compiler ship with your program , its portable tho needs nasm, installed on linux and mac , ven ships with the nasm assembler for windows so make sure to ship that or have a custom nasm version in the root folder called ``nasm.exe``) when you have made a .ven fine that has valid ven code simply call the ven compiler and pass in the file and the target OS+Architecture like this - ``ven file.ven l64`` this builds for linux 64 bit , you can also have the ***assembly file + object file*** by passing in the ``--retain-asm`` at the end.
> **This will produce ``a.asm`` , ``a.o`` and (``a.exe`` for windows and ``a.out`` for linux/mac) which you can rename move , keep in mind if you dont pass ``--retain-asm`` , you will not have ``a.asm`` and ``a.o``**

> ### !Note - If you are going to use ven in your language please read the Ven License file , its small and compact and has only 2 reqs

---
### Contributing to Ven

Ven is an open-source project, open to contributions from the community. Here are some ways to get involved:

* **Architectural Customization:** Design new architectures tailored for specialized hardware or unique software needs.
* **Optimizing Code Generation:** Enhance the performance of Ven’s code generation to push existing architectures to their limits.
* **Expanding Language Features:** Collaborate on adding new language features and syntax to broaden Ven’s functionality.
* **Bug Reporting and Issue Resolution:** Identify and resolve issues within the Ven compiler and runtime, contributing to its ongoing improvement.

### Future Directions

Ven is the foundation for the **Neit** programming language—a high-level language built on Ven's IR to provide exceptional speed, simplicity, and developer productivity. Neit’s integration with Ven’s IR aims to redefine what’s possible with high-performance languages.

**Development Started:** October 30, 2024
