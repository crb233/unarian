from unarian.base import UnarianError

from unarian.parser import (
    BuiltinType,
    Expression,
    Builtin,
    Function,
    Group,
    
    parse_expr,
)





#===========#
# Constants #
#===========#

main_function = 'main'





#===============#
# Error Classes #
#===============#

class InterpreterInternalError(UnarianError):
    pass

class InterpreterError(UnarianError):
    def __init__(self, stack=None, x=None, err=None):
        if stack is None: stack = []
        err_str = '.' if err is None else f': {err}'
        trace_str = '' if stack is None else '\n'.join(gen_stack_trace(stack, x))
        msg = f'Interpreter error{err_str}\n{trace_str}'
        super().__init__(msg)
        self.stack = stack
        self.x = x





#====================#
# Evaluation Methods #
#====================#

def gen_stack_trace(stack, x, *, maxlen=None):
    if maxlen is None: maxlen = 100
    
    if len(stack) <= maxlen:
        skip_from = len(stack)
        skip_to = len(stack)
    else:
        skip_from = maxlen // 2
        skip_to = len(stack) - (maxlen + 1) // 2
    
    yield ''
    yield 'Stack trace:'
    yield ''
    
    i = 0
    while i < len(stack):
        if i >= skip_from and i < skip_to - 1:
            yield ''
            yield f'{i} - {skip_to - 1}. Skipping frames...'
            yield ''
            i = skip_to
        
        y, group, r, c = stack[i]
        if group.name is not None:
            yield f'{i}. Evaluating {y} as input to function: {group.name}'
        else:
            yield f'{i}. Evaluating {y} as input to:'
        ind = ' ' * len(str(i))
        yield f'{ind}    {group}'
        yield f'{ind}    {group.get_subexpr_str(r, c - 1, spaced=True)}'
        
        i += 1
    
    yield f'{i}. Evaluated to {x}.'
    yield ''

def evaluate_builtin(builtin, x, stack, *, debug=None):
    if debug is None: debug = True
    
    if builtin.type == BuiltinType.Increment:
        return x + 1
    elif builtin.type == BuiltinType.Decrement:
        return x - 1 if x > 0 else None
    elif builtin.type == BuiltinType.Print:
        if debug:
            print(x)
        return x
    elif builtin.type == BuiltinType.Trace:
        if debug:
            for line in gen_stack_trace(stack, x):
                print(line)
        return x
    else:
        raise InterpreterInternalError(f'Unexpected builtin type {builtin.type!r}.')

def evaluate(lib, obj=None, x=None, *, debug=None, max_depth=None):
    if x is None: x = 0
    if debug is None: debug = True
    if max_depth is None: max_depth = 20_000
    
    if isinstance(obj, str):
        expr = parse_expr(obj, lib)
    elif isinstance(obj, Expression):
        expr = obj
    else:
        raise TypeError(f'Argument expr must be of type Expression, not {type(expr)!r}.')
    
    if isinstance(expr, Builtin):
        return evaluate_builtin(expr, x, [], debug=debug)
    elif isinstance(expr, Function):
        stack = [(x, lib[expr.name], 0, 0)]
    elif isinstance(expr, Group):
        stack = [(x, expr, 0, 0)]
    else:
        raise InterpreterInternalError(f'Unexpected object of type {type(expr)!r}.')
    
    while len(stack) > 0:
        if len(stack) > max_depth:
            raise InterpreterError(stack, x, f'Exceeded maximum stack depth: {max_depth}.')
        
        y, group, r, c = stack.pop(-1)
        
        if y is None:
            # Input is None. Return None
            x = None
            continue
        
        if r >= len(group.branches):
            # All branches returned None. Return None
            x = None
            continue
        
        if x is None:
            # Current branch returned None. Try next one with input y
            stack.append((y, group, r + 1, 0))
            x = y
            continue
        
        if c >= len(group.branches[r]):
            # Current branch returned value x. Return x
            continue
        
        # Evaluate the next expression in the current branch.
        expr = group.branches[r][c]
        stack.append((y, group, r, c + 1))
        
        if isinstance(expr, Builtin):
            # Evaluate a builtin.
            x = evaluate_builtin(expr, x, stack, debug=debug)
            
        elif isinstance(expr, Function):
            # Evaluate a function reference.
            name = expr.name
            if name not in lib:
                raise InterpreterError(stack, x, f'Reference to undefined function: {name!r}.')
            expr = lib[name]
            stack.append((x, expr, 0, 0))
            
        elif isinstance(expr, Group):
            # Evaluate a subgroup.
            stack.append((x, expr, 0, 0))
            
        else:
            raise InterpreterInternalError(f'Unexpected object of type {type(expr)!r}.')
    
    return x

def run(lib, x=None, *, debug=None, max_depth=None):
    expr = main_function
    if expr not in lib:
        raise InterpreterError(None, None, f'Cannot find main function {expr!r}.')
    return evaluate(lib, expr, x, debug=debug, max_depth=max_depth)
