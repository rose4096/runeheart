package rose.runeheart.block

import rose.runeheart.Runeheart
import net.minecraft.world.level.block.state.BlockBehaviour
import net.neoforged.neoforge.registries.DeferredRegister

import thedarkcolour.kotlinforforge.neoforge.forge.getValue

object ModBlocks {
    val REGISTRY = DeferredRegister.createBlocks(Runeheart.ID)

    val EXAMPLE_BLOCK by REGISTRY.registerSimpleBlock(
        "example_block",
        BlockBehaviour.Properties.of().lightLevel { 15 }
    )

}
