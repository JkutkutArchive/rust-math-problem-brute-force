## Exercise:
Use the numbers 5,6,7,8,9 in order, from smallest to largest,
together with one of the symbols +,-,* and / and two pairs of brackets
to make a calculation with an answer of 25/8

For example:
```
43/9
(5+6)-(7*8)/9 = 43/9
```

## Solution:
The simplest solution is `5+6-7/8*9`. As the problem enforces the use of parentheses,
the solution set can be found [here](./solution.txt)

## Running the program (docker):

```bash
make run
```

If you do not have make installed, you can run the following command:

```bash
docker run --rm -t -v $(pwd):/math-problem -v cargo_registy:/root/.cargo/registry -w /math-problem --entrypoint cargo jkutkut/docker4rust run
```

Keep in mind that `$(pwd)` is the current directory.
