from unarian import parser
from unarian import interpreter

#=========================#
# User-Friendly Interface #
#=========================#
 
class Unarian(dict):
    @classmethod
    def load(cls, text, **opts):
        lib = Unarian()
        return parser.parse_lib(text, lib, **opts)
    
    @classmethod
    def load_file(cls, filename, **opts):
        with open(filename, 'r', encoding='utf-8') as file:
            text = file.read()
        lib = Unarian(name=filename)
        return parser.parse_lib(text, lib, **opts)
    
    def __init__(self, *args, name=None, **kwargs):
        super().__init__(*args, **kwargs)
        self.name = name
    
    def parse(self, text, **opts):
        return parser.parse_expr(text, self, **opts)
    
    def evaluate(self, obj, x=None, **opts):
        return interpreter.evaluate(self, obj, x, **opts)
    
    def run(self, x=None, **opts):
        return interpreter.run(self, x, **opts)
