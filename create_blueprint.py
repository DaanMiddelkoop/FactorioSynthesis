import Dependency
from Grid import Grid, pos_sub
from Tile import Tile
import zlib
import json
import base64


class BlueprintCreator:
    def __init__(self):
        self.id = 1

    def create_blueprint_string(self, grid: Grid):
        entities = []
        [entities.extend(self.create_entities(t)) for t in grid.tiles.values()]
        blueprint = {
            "blueprint": {
                "entities": entities,
                "item": "blueprint",
                "version": 281479276199938
            }
        }
        json_data = json.dumps(blueprint)
        print(blueprint)
        compressed = zlib.compress(json_data.encode('ascii'))
        b64_encoded = base64.b64encode(compressed)
        result = "0" + b64_encoded.decode('ascii')
        print(result)


    def create_entities(self, tile: Tile):
        if tile.elem.kind == Dependency.BuildingType.Resource:
            return []

        y_shift = -tile.pos[0] * 2

        inserter_pos, inserter_direction = self.get_inserter_pos(tile)

        self.id += 2
        return [
            # Assembly machine
            {
                "entity_number": self.id - 2,
                "name": "assembling-machine-1",
                "position": {
                    "x": tile.pos[0] * 4 + 0.5,
                    "y": tile.pos[1] * 4 + 0.5 + y_shift
                },
                "recipe": tile.elem.recipe
            },
            # Connecting carrier
            {
                "entity_number": self.id - 1,
                "name": "inserter",
                "position": {
                    "x": inserter_pos[0] + 0.5,
                    "y": inserter_pos[1] + 0.5 + y_shift,
                },
                "direction": inserter_direction
            }
        ]

    def get_inserter_pos(self, tile: Tile):
        if tile.parent is None:
            return (tile.pos[0] * 4 + 2, tile.pos[1] * 4), 6

        relative_pos = pos_sub(tile.pos, tile.parent.pos)
        if relative_pos == (1, 0):
            pos = tile.pos[0] * 4 - 2, tile.pos[1] * 4 + 1
            return pos, 2

        if relative_pos == (0, 1):
            pos = tile.pos[0] * 4, tile.pos[1] * 4 - 2
            return pos, 4

        if relative_pos == (-1, 0):
            pos = tile.pos[0] * 4 + 2, tile.pos[1] * 4 - 1
            return pos, 6

        if relative_pos == (-1, -1):
            pos = tile.pos[0] * 4 + 2, tile.pos[1] * 4 + 1
            return pos, 6

        if relative_pos == (0, -1):
            pos = tile.pos[0] * 4, tile.pos[1] * 4 + 2
            return pos, 0

        if relative_pos == (1, -1):
            pos = tile.pos[0] * 4 - 2, tile.pos[1] * 4 - 1
            return pos, 2


        raise Exception(f"OOOOOEEEWIEEEEE {relative_pos}")





