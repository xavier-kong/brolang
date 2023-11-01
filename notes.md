How to Build a New Programming Language
https://pgrandinetti.github.io/compilers/page/how-to-build-a-new-programming-language/

- How to build a new programming language
    - high level 3 main steps:
        1. define the grammar
        2. build the front-end compiler for the source code
        3. build the back-end code generator
    - grammar: the syntactical rules that programmers will have to respect
    - front-end compiler: take source code and produces a data structure
    - back-end code generator: takes product of front end and creates code that machine can run

- steps to create a compiler for a programming language
    - 1. lexical analysis:  Recognize language keywords, operators, constant and every token that the grammar defines.
    - 2. parsing: tokens are "understood" such that relationship between them can be encoded in a tree-like data structure that describes the meaning of operations
    - 3. semantic analysis: understanding types and checking for inconsistency in the meaning of source code
    - 4. optimization
    - 5. code generation: optimized code translated into executable code


Writing Your Own Programming Language
https://scorpiosoftware.net/2023/08/18/writing-your-own-programming-language/

- tokenizer
- parser calls on tokenizer and builds AST
- interpreter provides runtime behaviour

How would I go about creating a programming language?
https://tomassetti.me/how-to-create-programming-language/

- more than just a compiler
    - language has to be designed
        - 2 phases: big picture, refinement
    - compiler created
        - parser
        - translate to ast
        - resolve symbols
        - validate tree
        - generate machine code
        - linking of static libraries
    - standard library implemented
    - supporting tools like editor and build systems


