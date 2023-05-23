# bop

bop is new language that is focused on simplifying and optimizing the transformation of information from a source structure into a target structure.

## working spec

a `.bop` file will contain mapping definitions like so

```
bankA.transfer.account_number   = bankB.transfer.accountNo
bankA.transfer.amount           = bankB.transfer.value
bankA.transfer.currency         = bankB.transfer.currency_symbol
```

From this we will then use the `lexer` which will remove empty space and generate a flattened vector like so:

- bankA
- .
- transfer
- .
- account_number
- =
- bankB
- .
- transfer
- .
- accountNo
- EOL

The parse then picks this up and returns a vector of small syntax tress

> These all have a root node of assign

From here will still need to work out some details ::laugh:: of the compiler backend!

## planning

- Generation of types to be done dynamically or from a defined schema (this would be good for VSCode extension)
- optimize null checking on source object
- optimize initialization of target object i.e. don't conditional check the root every assignment
- consuming bop via different services
    - json
    - xml
    - grpc
    - all above via batch file?