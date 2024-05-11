# How to write your own Solidity linter using Slang
### ... in 25 lines of code!

[Slang](https://blog.nomic.foundation/slang-alpha-release-6d322cf986a3) is our new API-first Solidity compiler designed to power your developer experience, whether that's by providing semantic information in your editor or enabling you to write custom linters, static analyzers, and other tools that operate on Solidity code.

In this guide we will show how you might use Slang to write a simple linter for Solidity in just 25 lines of code. To pick a simple, yet real-life example, we will attempt to write our own version of the [solhint](https://protofire.github.io/solhint/) [`avoid-tx-origin`](https://solhint-community.github.io/solhint-community/docs/rules/security/avoid-tx-origin.html) rule, which warns whenever `tx.origin` is used in the code.

## Installation
Slang is written in Rust and distributed both as a Rust package and an NPM package with TypeScript definitions.

We will use the Node.js package in this guide, but you can use the Rust package if you prefer.

Let's open a terminal and create a new project:

```bash
mkdir my-awesome-linter/
cd my-awesome-linter

npm init
npm install @nomicfoundation/slang
```

### Setting up TypeScript
Because there are many types and interfaces in Slang, we will use TypeScript to help us write our linter.

Let's install TypeScript and create a `tsconfig.json` file:

```bash
npm install --save-dev typescript
npx tsc --init
```

#### Running TypeScript code

There are [many ways to run TypeScript code in Node.js](https://nodejs.org/en/learn/getting-started/nodejs-with-typescript#running-typescript-code-in-nodejs), but we will use `tsx` for simplicity in this guide:

```bash
echo "console.log('Hello, world!');" > index.ts
npx tsx index.ts
# Should print "Hello, world!"
```

## Parsing the Solidity code
Before we can analyze the code, we need to parse the source code into a concrete syntax tree (CST). It can represent incomplete or invalid code, and is a good starting point for writing a linter.

Let's start by writing a simple `index.ts` that reads file contents specified as the first command line argument:

```ts
// index.ts
import fs from 'node:fs';

const filePath = process.argv[2];
const contents = fs.readFileSync(filePath, 'utf8');
```

#### Supporting multiple versions of Solidity
The Solidity language has changed quite a lot over the time, however Slang is designed to be able to parse all versions of Solidity that are in use today, which we consider to be 0.4.11 and later.

Because of this and because pragma directives are not required, we need to be explicit about the version of Solidity we aim to parse.

Let's say that we want to be source-compatible with code that's expected to work with Solidity 0.8.22. Let's contruct an instance of the `Language` class, which is the main entry point for parsing Solidity code:

```ts
import { Language } from "@nomicfoundation/slang/language";
const language = new Language("0.8.22");
```

#### Parsing different language constructs
Sometimes, we might be interested in parsing only a specific language construct, like a function declaration or an expression. To control this, we need to supply a `RuleKind` to the `parse` function, which represents a given language construct.

We're interested in parsing the entire file, so we will use `RuleKind.SourceUnit`.

```ts
import { RuleKind } from "@nomicfoundation/slang/kinds";
const output = language.parse(RuleKind.SourceUnit, contents);
```

#### Inspecting the parse output

The `parse` function returns a `ParseOutput` object, which contains the root of the CST (`tree()`) and a list of parse errors (`errors()`), if there are any.

For now, let's just print the CST to the console to see what it looks like:

```ts
const tree = results.tree();
console.log(tree.toJSON());

// Should print something like:
// {"kind":"SourceUnit","text_len":{...},"children":[...]}
```

## Matching specific patterns of code

We have just parsed the Solidity code into a structural representation that we can now analyze.

There are many ways to analyze the CST, but we will use our tree query language for brevity and its declarative nature.

The queries are instances of the `Query` class, which are created by parsing our query string, that match specific CST patterns and optionally
bind variables to them. The syntax is described in the [Tree Query Language](https://nomicfoundation.github.io/slang/user-guide/tree-query-language/) reference.

Without going too much into details, we want to match the `tx.origin` expression, which is a `MemberAccessExpression` with `tx` identifier as the left-hand side and `origin` identifier as the right-hand side:

```ts
import { Query } from "@nomicfoundation/slang/query";

let query = Query.parse(
  `@txorigin [MemberAccessExpression
        ...
        [Expression  ... @start ["tx"] ...]
        ...
        [MemberAccess ... ["origin"] ...]
    ]`,
);
```

That's a lot to unpack here! Let's break it down:
- tree nodes are enclosed in square brackets `[]`.
- the first name in the square brackets match the given node's `RuleKind`.
- after it, there is a list of children nodes we expect to match.
- `...` is a wildcard that matches any number of children.
- `@`-prefixed names before nodes are _bindings_, which are used to refer to specific nodes of the matched pattern.

#### Running the queries
The queries are executed using `Cursor` class, which is another way to traverse the syntax tree, so we need to instantiate one that starts at its root:

```ts
const cursor = results.createTreeCursor();
// This is a shorthand for:
// results.tree().createCursor({ utf8: 0, utf16: 0, char: 0 })
```

While it's possible to run multiple different queries concurrently using the same cursor, we will only run one in our case:

```ts
const matches = cursor.query([query]);
```

To access the matched `QueryResult`s, we need to call `next()` repeatedly until it returns `null`:

```ts
let match = null;
while (match = matches.next()) {
    // ... do something with the matched tree fragment
}
```

Now, for each query result, we can access the bindings we defined in the query to access the nodes we were interested in.

Each binding can point via `Cursor` to a single node or multiple nodes, depending on the query. In our case, we expect `@txorigin` to point to a single `MemberAccessExpression` node.

Let's print out the JSON representation of the matched node pointed to by a `Cursor`:

```ts
const txorigin = match.bindings.txorigin[0];
console.log(txorigin.node().toJSON());
// Should print our matched node:
// {"kind":"MemberAccessExpression","text_len":{...},"children":[...]}
```

## Reporting the findings
The only thing left to do is to report our findings to the user.

Because we get back a `Cursor` that point to the offending nodes from our queries, we can use its `.textOffset` property to map back its position in the source code.

At the time of writing, we support character offsets only, but we plan to provide cursors that keep track of the line/columns as well.

For completeness' sake, let's write a naive function that calculates the line and column number for a given character offset:

```ts
function getLineAndColumn(contents: string, offset: number): [number, number] {
  const NEWLINE = /(\r)?\n/g;

  contents = contents.slice(0, offset);
  const line = contents.match(NEWLINE)?.length ?? 1;
  const col = contents.split(NEWLINE).pop()?.length ?? 1;
  // Use 1-based indexing as commonly used in editors
  return [line + 1, col + 1];
}
```

With it, we can conveniently print out the warning message to the console in a format understood by most editors using the `<filename>:<line>:<column>` notation:

```ts
const byteOffset = txorigin.textOffset.utf8;
const [line, col] = getLineAndColumn(contents, byteOffset);
console.warn(`${filePath}:${line}:${col}: warning: avoid using \`tx.origin\``);
```

To access the full span of the node, we could use the `textRange` property on the cursor, which returns the start and the end offsets of the node in the source code.

We could get even more creative and plug this information into a custom formatter of our choice, but for now this will suffice.

## Putting it all together
Here's the complete code for our linter:

```ts
// file: index.ts
import fs from "node:fs";
import { Language } from "@nomicfoundation/slang/language";
import { RuleKind } from "@nomicfoundation/slang/kinds";
import { Query } from "@nomicfoundation/slang/query";

const filePath = process.argv[2];
const contents = fs.readFileSync(filePath, "utf8");

const language = new Language("0.8.22");
const results = language.parse(RuleKind.SourceUnit, contents);

const query = Query.parse(
  `@txorigin [MemberAccessExpression
        ...
        [Expression  ... @start ["tx"] ...]
        ...
        [MemberAccess ... ["origin"] ...]
    ]`,
);

const cursor = results.createTreeCursor();
const matches = cursor.query([query]);

let match = null;
while ((match = matches.next())) {
  const txorigin = match.bindings.txorigin[0];

  const byteOffset = txorigin.textOffset.utf8;
  const [line, col] = getLineAndColumn(contents, byteOffset);
  console.warn(`${filePath}:${line}:${col}: warning: avoid using \`tx.origin\``);
}

// Utility function to get line and column number from byte offset
function getLineAndColumn(contents: string, offset: number): [number, number] {
  const NEWLINE = /(\r)?\n/g;

  contents = contents.slice(0, offset);
  const col = contents.match(NEWLINE)?.length ?? 1;
  const line = contents.split(NEWLINE).pop()?.length ?? 1;
  // Use 1-based indexing as commonly used in editors
  return [col + 1, line + 1];
}
```

If we not account for the utility function, comments and empty lines, the code is indeed 25 lines long!

Let's run our linter against [the official motivating example](https://docs.soliditylang.org/en/latest/security-considerations.html#tx-origin) from the Solidity documentation for the `tx.origin` rule:

```solidity
// file: example.sol
// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;
// THIS CONTRACT CONTAINS A BUG - DO NOT USE
contract TxUserWallet {
    address owner;

    constructor() {
        owner = msg.sender;
    }

    function transferTo(address payable dest, uint amount) public {
        // THE BUG IS RIGHT HERE, you must use msg.sender instead of tx.origin
        require(tx.origin == owner);
        dest.transfer(amount);
    }
}
```

Running the linter:

```bash
$ npx tsx index.ts example.sol
example.sol:13:17: warning: avoid using `tx.origin`
# ...which points here:
#         require(tx.origin == owner);
#                 ^
```

We can see that it works as expected!

## Conclusion
In this guide, we have shown how to write a simple linter for Solidity using Slang in just 25 lines of code by implementing a simple version of the `avoid-tx-origin` rule from `solhint`.

We have covered the basics of parsing Solidity code, matching specific code patterns, and reporting the findings to the user in a simple and straightforward way.

We hope that this guide has inspired you to write your own linters or any other tools that operate on Solidity code using Slang!

If you have any questions or feedback, feel free to reach out to us on [GitHub](https://github.com/NomicFoundation/slang) and/or check out our [documentation](https://nomicfoundation.github.io/slang/).
