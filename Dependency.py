from enum import Enum


class BuildingType(Enum):
    Assembler = 1
    Resource = 2


class BaseDependency:
    kind = BuildingType.Assembler
    dependencies = []
    pass

    def __repr__(self):
        return self.__class__.__name__

    def __eq__(self, other):
        return self.__class__.__name__ == other.__class__.__name__


class IronPlate(BaseDependency):
    kind = BuildingType.Resource
    dependencies = []


class CopperPlate(BaseDependency):
    kind = BuildingType.Resource
    dependencies = []


class StoneTablet(BaseDependency):
    kind = BuildingType.Resource
    dependencies = []


class CopperCable(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [CopperPlate()]
    recipe = "copper-cable"


class IronGearWheel(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [IronPlate()]
    recipe = "iron-gear-wheel"


class IronStick(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [IronPlate()]
    recipe = "iron-stick"


class SingleCylinderEngine(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [IronGearWheel(), IronPlate()]
    recipe = "motor"


class BurnerInserter(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [IronStick(), SingleCylinderEngine()]
    recipe = "burner-inserter"


class SmallElectricMotor(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [IronGearWheel(), CopperCable(), IronPlate()]
    recipe = "electric-motor"


class Inserter(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [BurnerInserter(), SmallElectricMotor()]
    recipe = "inserter"


class LongHandedInserter(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [Inserter(), IronStick(), IronPlate()]
    recipe = "long-handed-inserter"


class ElectronicCircuit(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [CopperCable(), StoneTablet()]
    recipe = "electronic-circuit"


class FastInserter(BaseDependency):
    kind = BuildingType.Assembler
    dependencies = [ElectronicCircuit(), Inserter(), IronPlate()]
    recipe = "fast-inserter"