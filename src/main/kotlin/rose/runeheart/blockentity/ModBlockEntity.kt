package rose.runeheart.blockentity

import net.minecraft.core.registries.Registries
import net.minecraft.world.level.block.entity.BlockEntityType
import net.neoforged.neoforge.registries.DeferredRegister
import rose.runeheart.Runeheart
import rose.runeheart.block.ModBlocks

object ModBlockEntity {
    val REGISTRY = DeferredRegister.create(Registries.BLOCK_ENTITY_TYPE, Runeheart.ID)

    val EXAMPLE_BLOCK = REGISTRY.register("example_block") { ->
        BlockEntityType.Builder.of(::ExampleBlockEntity, ModBlocks.EXAMPLE_BLOCK).build(null)
    }
}