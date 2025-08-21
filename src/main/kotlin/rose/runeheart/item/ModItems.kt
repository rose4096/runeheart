package rose.runeheart.item

import rose.runeheart.Runeheart
import rose.runeheart.block.ModBlocks
import net.neoforged.neoforge.registries.DeferredRegister

object ModItems {
    val REGISTRY = DeferredRegister.createItems(Runeheart.ID)

    val EXAMPLE_BLOCK_ITEM = REGISTRY.registerSimpleBlockItem("example_block") { ->
        ModBlocks.EXAMPLE_BLOCK
    }
}