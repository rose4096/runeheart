package rose.runeheart.blockentity

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.neoforged.neoforge.capabilities.Capabilities
import rose.runeheart.Native
import rose.runeheart.ScriptContext

//        ScriptContext(
//            """
//        pub fn tick() {
//            println!("hello from tick!");
//        }
//        """
//        ).use {
//            Native.tick(it.handle);
//        }

class ExampleBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(ModBlockEntity.EXAMPLE_BLOCK.get(), pos, state) {

    private var scripts: MutableMap<String, ScriptContext> = hashMapOf()

    data class RelativeBlockEntity(val entity: BlockEntity?, val side: Direction)

    fun getSurroundingBlockEntities(level: Level, relative: BlockPos): List<RelativeBlockEntity> {
        return Direction.entries.map {
            RelativeBlockEntity(level.getBlockEntity(relative.relative(it)), it)
        }
    }

    companion object {
        fun tick(level: Level, pos: BlockPos, state: BlockState, blockEntity: ExampleBlockEntity) {
            if (level.isClientSide) return;

            if (blockEntity.scripts.isEmpty()) {
                blockEntity.scripts["rune"] = ScriptContext(
                    """
                    pub fn tick() {
                        println!("hello from tick!");
                    }
                    """
                )
            }

            val itemHandlers = blockEntity.getSurroundingBlockEntities(level, pos).filter {
                it.entity !== null && level.getCapability(
                    Capabilities.ItemHandler.BLOCK,
                    it.entity.blockPos,
                    it.side.opposite
                ) !== null
            }

            blockEntity.scripts["rune"]?.let {
                Native.tick(it.handle)
            }
        }
    }
}