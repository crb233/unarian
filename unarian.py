#!/usr/bin/env python3

import sys
import argparse
import pathlib

from unarian import Unarian

#========================#
# Command-Line Interface #
#========================#

def get_argparser():
    ap = argparse.ArgumentParser(description='Unarian language interpreter and compiler.')
    ap.add_argument('file',
        nargs='?', default=None, type=pathlib.Path,
        help='If included, load the specified Unarian source code.')
    ap.add_argument('-e', '--expr', dest='expr',
        default='main',
        help='Evaluates or compiles the specified function / expression. Defaults to \'main\'.')
    ap.add_argument('-g', '--debug', dest='debug',
        action='store_true',
        help='Evaluates or compiles with debugging. Defaults to false.')
    ap.add_argument('-d', '--depth', dest='depth',
        default=10000,
        help='Evaluates or compiles with the specified maximum depth. Defaults to 10000.')
    ap.add_argument('-c', '--compile', dest='compile',
        nargs='?', default=False, const=True, type=pathlib.Path,
        help='If included, compile to the specified output file. If no file is given, compile to an auto-generated output file. Otherwise, don\'t compile. Incompatible with \'--input\'.')
    ap.add_argument('-i', '--input', dest='input',
        action='store_true',
        help='Evaluates the input from stdin. Incompatible with \'--compile\'.')
    return ap

def main(argv):
    ap = get_argparser()
    args = ap.parse_args(argv)
    
    try:
        # Load source code
        if args.file is None:
            prog = Unarian()
        elif not args.file.exists():
            sys.exit(f'Source code path {args.file} doesn\'t exist.')
        elif not args.file.is_file():
            sys.exit(f'Source code path {args.file} isn\'t a file.')
        else:
            prog = Unarian.load_file(args.file)
        
        # Get expression
        expr = prog.parse(args.expr)
        
        # Get debug option
        debug = args.debug
        
        # Get depth option
        depth = args.depth
        
        # Get compilation path
        if args.compile is False:
            compile = None
        elif args.compile is True:
            # Default compilation paths
            if args.file is not None:
                compile = args.file.with_suffix('.c')
            else:
                compile = pathlib.Path.cwd() / 'out.c'
            
            # Add counter to name if name already exists
            i = 0
            new_compile = compile
            while new_compile.exists():
                i += 1
                new_compile = compile.with_stem(f'{compile.stem}_{i}')
            compile = new_compile
        else:
            compile = args.compile
        
        # Get input
        if compile is not None:
            if args.input is not None:
                sys.exit(f'Cannot run with options \'--compile\' and \'--input\' at the same time.')
            input = None
        elif args.input is None:
            input = [0]
        else:
            input = args.input
        
        opts = {
            'debug': debug,
            'max_depth': depth,
        }
        
        # Run
        if compile is not None:
            sys.exit(f'Compilation not yet implemented.')
        elif input:
            for line in sys.stdin:
                for x in line.split():
                    try:
                        x = int(x)
                    except ValueError as e:
                        print('-', end=' ', flush=True)
                    else:
                        y = prog.evaluate(expr, x, **opts)
                        print(y if y is not None else '-', end=' ', flush=True)
                print()
        else:
            y = prog.evaluate(expr, **opts)
            print(y if y is not None else '-')
        
    except ParserError as err:
        pass
        
    except InterpreterError as err:
        pass

if __name__ == '__main__':
    main(sys.argv[1:])
