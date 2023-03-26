#!/usr/bin/env python3

def tokenize(file):
    for line in file:
        i = line.find('#')
        if i >= 0:
            line = line[:i]
        for tok in line.split():
            yield tok

def parse_group(toks, i):
    i += 1
    alternation = []
    composition = []
    while i < len(toks):
        if toks[i] == '}':
            alternation.append(composition)
            return alternation, i
        elif toks[i] == '|':
            alternation.append(composition)
            composition = []
        elif toks[i] == '{':
            group, i = parse_group(toks, i)
            composition.append(group)
        else:
            composition.append(toks[i])
        i += 1

def parse(file):
    toks = list(tokenize(file))
    lib = {}
    i = 0
    while i < len(toks):
        name = toks[i]
        i += 1
        grp, i = parse_group(toks, i)
        lib[name] = grp
        i += 1
    return lib

def evaluate(lib, expr, x):
    stack = [(x, expr, 0, 0)]
    while len(stack) > 0:
        y, expr, i, j = stack.pop(-1)
        if i >= len(expr):
            x = None
        elif x is None:
            x = y
            stack.append((y, expr, i + 1, 0))
        elif j < len(expr[i]):
            stack.append((y, expr, i, j + 1))
            subexpr = expr[i][j]
            if subexpr == '+':
                x += 1
            elif subexpr == '-':
                x = x - 1 if x > 0 else None
            elif isinstance(subexpr, list):
                stack.append((x, subexpr, 0, 0))
            else:
                stack.append((x, lib[subexpr], 0, 0))
    return x

def main():
    import sys
    with open(sys.argv[1]) as file:
        lib = parse(file)
    expr = [['main']]
    for x in map(int, sys.argv[2:]):
        print(x, '->', evaluate(lib, expr, x), end='   ')
    print()

if __name__ == '__main__':
    main()