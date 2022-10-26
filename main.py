import base64
import zlib

import Dependency
from create_blueprint import BlueprintCreator
from placer2 import Placer

example_string = "0eNqVkdsKwjAMht8l1524g27rq4jIVsMMrOloO3GMvbvdFFE8gHdJaL7/Ix2hbnvsLLEHOQIpww7kbgRHDVftPPNDhyCBPGoQwJWeu8o51HVL3ES6UidijGKYBBAf8QIynvYCkD15whtvaYYD97pGGx78JgnojAvLhmeDAIyybLURMIQqLVabkGRR0SKGLSpvSUXaeGODxFtY8ggjdmh9mH0ISF4CPmDSv53X35zvquFIy1nl0y8IOKN1CyUp4iwvk3wbl2WZFtN0BaWoi9U="
example_decoded = base64.b64decode(example_string.encode('ascii')[1:])
example_decompressed = zlib.decompress(example_decoded).decode('ascii')
print(example_decompressed)
from visualise import visualise

print("Test", Dependency.BurnerInserter() == Dependency.BurnerInserter())

grid = Placer().place(Dependency.FastInserter())
BlueprintCreator().create_blueprint_string(grid)

# sorted_tiles = sorted(grid.tiles.values(), key=lambda x: x.pos)
# print(len(sorted_tiles))
#
#
#
#
#
#

visualise(grid)