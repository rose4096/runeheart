package rose.runeheart.blockentity

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.neoforged.neoforge.capabilities.Capabilities
import rose.runeheart.Native
import rose.runeheart.Runeheart

class ExampleBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(ModBlockEntity.EXAMPLE_BLOCK.get(), pos, state) {

    data class RelativeBlockEntity(val entity: BlockEntity?, val side: Direction)

    fun getSurroundingBlockEntities(level: Level, relative: BlockPos): List<RelativeBlockEntity> {
        return Direction.entries.map {
            RelativeBlockEntity(level.getBlockEntity(relative.relative(it)), it)
        }
    }

    companion object {
        fun tick(level: Level, pos: BlockPos, state: BlockState, blockEntity: ExampleBlockEntity) {
            val itemHandlers = blockEntity.getSurroundingBlockEntities(level, pos).filter {
                it.entity !== null && level.getCapability(
                    Capabilities.ItemHandler.BLOCK,
                    it.entity.blockPos,
                    it.side.opposite
                ) !== null
            }

        }
    }
}