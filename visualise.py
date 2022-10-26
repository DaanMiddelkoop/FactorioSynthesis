from PIL import Image, ImageDraw, ImageFont

import Dependency
from Grid import Grid, pos_add, pos_sub

scale = 100

def visualise(grid: Grid):
    print("VISUALISING")
    tiles = grid.tiles.values()
    locs = [x.pos for x in tiles]
    min_loc = (min([x[0] for x in locs]), min([x[1] for x in locs]))
    for t in tiles:
        t.pos = pos_sub(t.pos, min_loc)
    locs = [x.pos for x in tiles]
    max_loc = (max([x[0] for x in locs]) + 3, max([x[1] for x in locs]) + 3)
    print(min_loc)
    print(grid.tiles.values())
    print(max_loc)
    print(len(grid.tiles.values()))

    image = Image.new("RGB", (max_loc[0] * scale, max_loc[1] * scale))
    d = ImageDraw.Draw(image)

    for tile in tiles:
        extra_units = -tile.pos[0] * 50

        start_x = tile.pos[0] * scale + 100
        start_y = tile.pos[1] * scale + extra_units + 100
        if tile.elem.kind == Dependency.BuildingType.Resource:
            d.rectangle((start_x, start_y, start_x + scale - 1, start_y + scale - 1), fill=(125, 125, 125, 125))
        else:
            d.rectangle((start_x, start_y, start_x + scale - 1, start_y + scale - 1))
        d.text((start_x, start_y), tile.elem.__class__.__name__, fill=(255, 255, 255, 255))
    image.show()