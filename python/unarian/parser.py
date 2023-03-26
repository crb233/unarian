import enum

from unarian.base import UnarianError





#=====================#
# Enums and Constants #
#=====================#

class TokenType(enum.Enum):
    OpenGroup  = enum.auto()
    CloseGroup = enum.auto()
    Branch     = enum.auto()
    Name       = enum.auto()

class BuiltinType(enum.Enum):
    Increment = enum.auto()
    Decrement = enum.auto()
    Print     = enum.auto()
    Trace     = enum.auto()

comment_start = '#'

special_tokens = {
    '{' : TokenType.OpenGroup,
    '}' : TokenType.CloseGroup,
    '|' : TokenType.Branch
}

builtin_functions = {
    '+' : BuiltinType.Increment,
    '-' : BuiltinType.Decrement,
    '!' : BuiltinType.Print,
    '@' : BuiltinType.Trace,
}





#===============#
# Error Classes #
#===============#

class ParserInternalError(UnarianError):
    pass

class ParserError(UnarianError):
    def __init__(self, line, err=None):
        line_str = '' if line is None else f' in line {line}'
        err_str = '.' if err is None else f': {err}'
        msg = f'Parser error{line_str}{err_str}'
        super().__init__(msg)
        self.line = line



#=====================#
# Syntax Tree Classes #
#=====================#

class Token:
    def __init__(self, string, type, line):
        self.string = string
        self.type = type
        self.line = line
    
    def __str__(self):
        return self.string
    
    def __repr__(self):
        return f'Token({self.string!r}, {self.type!r}, {self.line!r})'

class Expression:
    pass

class Builtin(Expression):
    def __init__(self, name):
        self.name = name
        self.type = builtin_functions[name]
    
    def __str__(self):
        return self.name
    
    def __repr__(self):
        return f'Builtin({self.name!r}, {self.type!r})'

class Function(Expression):
    def __init__(self, name):
        self.name = name
    
    def __str__(self):
        return self.name
    
    def __repr__(self):
        return f'Function({self.name!r})'

class Group(Expression):
    def __init__(self, branches, *, name=None):
        self.branches = branches
        self.name = name
    
    def __str__(self):
        outer = []
        for chain in self.branches:
            outer.append(' '.join(str(obj) for obj in chain))
        return '{ ' + ' | '.join(outer) + ' }'
    
    def __repr__(self):
        if self.name is None:
            return f'Group({str(self)!r})'
        else:
            return f'Group({str(self)!r}, name={self.name!r})'
    
    def get_subexpr_spacing(self, r, c):
        n = 2
        for i in range(r):
            for j in range(len(self.branches[i])):
                n += len(str(self.branches[i][j])) + 1
            n += 2
        for j in range(c):
            n += len(str(self.branches[r][j])) + 1
        return n
    
    def get_subexpr_str(self, r, c, *, spaced=False):
        if spaced:
            n = self.get_subexpr_spacing(r, c)
            return ' ' * n + str(self.branches[r][c])
        return str(self.branches[r][c])





#=================#
# Parsing Methods #
#=================#

def tokenize(text):
    """
    Converts string input into a list of tokens.
    """
    tokens = []
    for i, line in enumerate(text.splitlines()):
        # Remove comments
        line = line.split(comment_start, maxsplit=1)[0]
        
        # Split on whitespace
        line = line.split()
        
        # Construct tokens
        for word in line:
            if word in special_tokens:
                tokens.append(Token(word, special_tokens[word], i + 1))
            else:
                tokens.append(Token(word, TokenType.Name, i + 1))
    return tokens

def read_expr(tokens, i):
    """
    Reads an expression from a tokens list starting at index i.
    Returns the expression and the index of the first unused token.
    """
    branches = []
    chain = []
    while i < len(tokens):
        tok, i = tokens[i], i + 1
        if tok.type is TokenType.OpenGroup:
            subgroup, i = read_group(tokens, i - 1)
            chain.append(subgroup)
        elif tok.type is TokenType.CloseGroup:
            i -= 1
            break
        elif tok.type is TokenType.Branch:
            branches.append(chain)
            chain = []
        elif tok.type is TokenType.Name:
            chain.append(tok)
        else:
            raise ParserInternalError(f'Unexpected token type {tok.type!r}: {tok!r}.')
    
    branches.append(chain)
    group = Group(branches)
    return group, i

def read_group(tokens, i):
    """
    Reads a group from a tokens list starting at index i.
    Returns the group and the index of the first unused token.
    """
    if i >= len(tokens):
        if len(tokens) > 0:
            tok = tokens[-1]
            raise ParserError(tok.line, f'Unexpected end of file after {tok.string!r}. Expected a {"{"!r}.')
        else:
            raise ParserError(None, f'Unexpected end of file. Expected a {"{"!r}.')
    tok, i = tokens[i], i + 1
    if tok.type is not TokenType.OpenGroup:
        raise ParserError(tok.line, f'Unexpected token {tok.string!r}. Expected a {"{"!r}.')
    
    expr, i = read_expr(tokens, i)
    
    if i >= len(tokens):
        raise ParserError(tok.line, f'Unexpected end of file after {tok.string!r}. Expected a {"}"!r}.')
    tok, i = tokens[i], i + 1
    if tok.type is not TokenType.CloseGroup:
        raise ParserError(tok.line, f'Unexpected token {tok.string!r}. Expected a {"}"!r}.')
    
    return expr, i

def read_lib(tokens, i, lib=None):
    """
    Reads a library from a tokens list starting at index i.
    Returns the library and the index of the first unused token.
    """
    if lib is None: lib = dict()
    
    while i < len(tokens):
        tok, i = tokens[i], i + 1
        
        if tok.type is not TokenType.Name:
            raise ParserError(tok.line, f'Unexpected token {tok.string!r}. Expected function declaration.')
        elif tok.string in builtin_functions:
            raise ParserError(tok.line, f'Function {tok.string!r} is built-in and cannot be redefined.')
        elif tok.string in lib:
            raise ParserError(tok.line, f'Function {tok.string!r} already defined')
        
        group, i = read_group(tokens, i)
        group.name = tok.string
        lib[group.name] = group
    
    return lib, i

def resolve_references(obj, lib=None):
    """
    Recursively replaces tokens with the functions they reference.
    """
    if isinstance(obj, Token):
        if obj.string in builtin_functions:
            return Builtin(obj.string)
        elif lib is None or obj.string in lib:
            return Function(obj.string)
        else:
            raise ParserError(obj.line, f'Reference to undefined function: {obj.string!r}.')
        
    elif isinstance(obj, Builtin) or isinstance(obj, Function):
        return obj
        
    elif isinstance(obj, Group):
        for chain in obj.branches:
            for i in range(len(chain)):
                chain[i] = resolve_references(chain[i], lib)
        return obj
        
    else:
        raise ParserInternalError(f'Unexpected object of type {type(obj)!r}: {obj!r}.')

def simplify_expr(expr, *, asgroups=None):
    """
    Returns a simpler expression with equivalent semantics.
    """
    if asgroups is None: asgroups = False
    
    if isinstance(expr, Builtin) or isinstance(expr, Function):
        if asgroups:
            return Group([[expr]])
        else:
            return expr
        
    elif isinstance(expr, Group):
        # Simplified list of branches
        new_branches = []
        for chain in expr.branches:
            
            # Simplified chain of expressions
            new_chain = []
            for subexpr in chain:
                subexpr = simplify_expr(subexpr)
                i = len(new_chain)
                
                # Simplify subgroups with only one branch
                if isinstance(subexpr, Group) and len(subexpr.branches) == 1:
                    new_chain.extend(subexpr.branches[0])
                else:
                    new_chain.append(subexpr)
                
                # Cancel out adjacent + - builtins
                while 0 < i < len(new_chain):
                    a = new_chain[i - 1]
                    if not isinstance(a, Builtin) or a.type != BuiltinType.Increment:
                        break
                    b = new_chain[i]
                    if not isinstance(b, Builtin) or b.type != BuiltinType.Decrement:
                        break
                    # Delete a and b from new_chain
                    del new_chain[i - 1 : i + 1]
                    i -= 1
            
            # Simplify branches containing only a single group
            if len(new_chain) == 1 and isinstance(new_chain[0], Group):
                new_branches.extend(new_chain[0].branches)
            else:
                new_branches.append(new_chain)
        
        # Remove obviously unreachable code
        for i in range(len(new_branches) - 1):
            if len(new_branches[i]) == 0:
                new_branches = new_branches[: i + 1]
                break
        
        # If the group contains only one subexpression and we don't have to
        # return a group, return that subexpression instead.
        if not asgroups and len(new_branches) == 1 and len(new_branches[0]) == 1:
            return new_branches[0][0]
        
        return Group(new_branches, name=expr.name)
        
    else:
        raise ParserInternalError(f'Unexpected object of type {type(expr)!r}: {expr!r}.')

def parse_expr(text, lib=None, *, simplify=None):
    """
    Parses and returns an expression from string input.
    """
    if simplify is None: simplify = True
    
    # Tokenize and parse
    tokens = tokenize(text)
    expr, i = read_expr(tokens, 0)
    
    if i < len(tokens):
        tok = tokens[i]
        raise ParserError(tok.line, f'Unexpected token {tok.string!r}.')
    
    # Clean up expression
    expr = resolve_references(expr, lib)
    if simplify:
        expr = simplify_expr(expr)
    
    return expr

def parse_lib(text, lib=None, *, simplify=True):
    """
    Parses and returns a library from string input.
    """
    if simplify is None: simplify = True
    
    # Tokenize and parse
    tokens = tokenize(text)
    lib, i = read_lib(tokens, 0, lib)
    
    # Clean up function definitions
    for name in lib:
        lib[name] = resolve_references(lib[name], lib)
        if simplify:
            lib[name] = simplify_expr(lib[name], asgroups=True)
    
    return lib
