import itertools
import random

import Dependency
from Dependency import BuildingType
from Grid import Grid
from Tile import Tile
import copy

from visualise import visualise


class Placer:
    best = None
    best_size = (999, 999)

    def place(self, dependency: Dependency):
        grid = Grid()
        tile = Tile(dependency, (0, 0), None)
        grid.place(tile)
        assert self.place_dependencies(grid, dependency.dependencies, tile)

        return grid

    def add_elem(self, grid, dependency: Dependency, parent: Tile):
        # This parent already has such a unit around it.
        # if dependency in grid.surrounding_dependencies(parent):
        #     return True

        free_spots = grid.free_neighbours(parent)
        random.shuffle(free_spots)
        if len(free_spots) == 0:
            return False

        # Place this tile in one of the free spots:
        for spot in free_spots:
            # We want our end product to be on the right
            if spot[0] > grid.base:
                continue
            # if dependency.kind == BuildingType.Assembler and spot[0] < grid.resource_min - 1:
            #     continue
            # if dependency.kind == BuildingType.Resource and spot[0] > grid.resource_min + 1:
            #     continue

            tile = Tile(dependency, spot, parent)
            print(f"Placing {tile} in {grid.tiles.values()}")
            grid.place(tile)
            if self.place_dependencies(grid, dependency.dependencies, tile):
                return True
            grid.unplace(tile)

        # None of the spots worked...
        return False

    def place_dependencies(self, grid, dependencies, parent: Tile):
        # Place all dependencies in all orders
        permutations = itertools.permutations(dependencies)
        for p in permutations:
            if self.place_dependencies2(grid, p, parent):
                return True
            grid.unplace_children(parent)
        return False

    def place_dependencies2(self, grid, dependencies, parent: Tile):
        for dependency in dependencies:
            if not self.add_elem(grid, dependency, parent):
                return False
        return True





