import Dependency
from Tile import Tile


def pos_add(x, y):
    return x[0] + y[0], x[1] + y[1]

def pos_sub(x, y):
    return x[0] - y[0], x[1] - y[1]


available_directions = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (0, 1)]


class Grid:

    def __init__(self):
        self.tiles = dict()
        self.tiles_min = 0

        self.tiles_y_min = 0
        self.tiles_y_max = 0
        self.non_resource_min = 0
        self.base = 0
        # self.resource_max = (9999, 9999)

    def place(self, tile: Tile, pos: (int, int)):
        tile.pos = pos
        self.tiles[tile.pos] = tile
        self.update_resource_minmax()

    def free_neighbours(self, tile: Tile):
        return [p for p in [pos_add(x, tile.pos) for x in available_directions] if p not in self.tiles]

    def unplace(self, tile: Tile):
        # self.unplace_children(tile)
        # tile.parent.children.remove(tile)
        del self.tiles[tile.pos]
        self.update_resource_minmax()
        tile.pos = None

    def surrounding_dependencies(self, tile: Tile):
        surrounding = [p for p in [pos_add(x, tile.pos) for x in available_directions] if p in self.tiles]
        return [(self.tiles[x].elem, x) for x in surrounding]

    def update_resource_minmax(self):
        self.tiles_min = min([t.pos[0] for t in self.tiles.values()])
        self.tiles_y_min = min([t.pos[1] for t in self.tiles.values()])
        self.tiles_y_max = max([t.pos[0] for t in self.tiles.values()])
        self.non_resource_min = min([t.pos[0] for t in self.tiles.values() if t.elem.kind != Dependency.BuildingType.Resource])

    def unplace_children(self, tile: Tile):
        print(f"removing {tile.children} from {self.tiles.values()} parent: {tile}")
        for t in tile.children:
            print(f"Removing {t}")
            self.unplace(t)
        tile.children.clear()

    def expansion_factor(self, pos):
        if pos[1] > self.tiles_y_max:
            return pos[1] - self.tiles_y_max
        if pos[1] < self.tiles_y_min:
            return self.tiles_y_min - pos[1]
        return 0

    def candidate_order(self, candidate):
        free_places = self.free_neighbours(candidate.parent)
        if len(free_places) == 0:
            return 10000
        return min([self.expansion_factor(x) for x in free_places])

