import itertools
import random

import Dependency
from Grid import Grid
from Tile import Tile


def generate_child_tiles(tile):
    tile.children = [Tile(t, tile) for t in tile.elem.dependencies]
    [generate_child_tiles(x) for x in tile.children]


def get_child_tiles(tile):
    result = []
    result.extend(tile.children)
    [result.extend(get_child_tiles(x)) for x in tile.children]
    return result


class Placer:

    def place(self, dependency: Dependency):
        grid = Grid()
        tile = Tile(dependency, None)
        generate_child_tiles(tile)
        grid.place(tile, (0, 0))
        self.place_candidates_perms(grid, tile.children)
        return grid

    def place_tile(self, grid, tile, rest):
        # Check if tile kind is already located in the vicinity:
        for (e, p) in grid.surrounding_dependencies(tile.parent):
            print(e, p, tile.elem, e == tile.elem, id(e), id(tile.elem))
            if e == tile.elem:
                print("Checking for path through exis")
                tile.pos = p
                if self.place_candidates_perms(grid, rest + tile.children):
                    return True
                tile.pos = None

        free_places = grid.free_neighbours(tile.parent)
        # random.shuffle(free_places)
        for p in sorted(free_places, key=grid.expansion_factor):
            # Restrict the possible placements
            if p[0] > grid.base:
                continue
            # if tile.elem.kind == Dependency.BuildingType.Resource and p[0] >= grid.non_resource_min:
            #     continue
            # if tile.elem.kind == Dependency.BuildingType.Assembler and p[0] < grid.tiles_min:
            #     continue

            grid.place(tile, p)
            if self.place_candidates_perms(grid, rest + tile.children):
                return True
            grid.unplace(tile)
        return False

    def place_candidates_perms(self, grid, candidates):
        if len(candidates) == 0:
            print("WE DID IT BOYSSSSS_-----------------------------------------------")
            return True

        # print("Layed down", grid.tiles.values())
        # print("Candidates", candidates)
        # print(grid.free_neighbours(candidates[0].parent))
        # random.shuffle(candidates)
        for candidate in sorted(candidates, key=grid.candidate_order):
            if self.place_tile(grid, candidate, [x for x in candidates if x != candidate]):
                return True
        return False


