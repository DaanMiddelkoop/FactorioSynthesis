import Dependency


class Tile:
    def __init__(self, elem: Dependency, parent):
        self.elem = elem
        self.pos = None
        self.parent = parent
        self.children = []

    def __repr__(self):
        return f"T({self.elem.__class__.__name__}, {self.pos})"