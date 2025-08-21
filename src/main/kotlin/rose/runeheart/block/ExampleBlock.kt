package rose.runeheart.block

import net.minecraft.core.BlockPos
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.entity.BlockEntityTicker
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockBehaviour
import net.minecraft.world.level.block.state.BlockState
import rose.runeheart.blockentity.ExampleBlockEntity
import rose.runeheart.blockentity.ModBlockEntity

class ExampleBlock(props: BlockBehaviour.Properties) : Block(props), EntityBlock {
    override fun newBlockEntity(pos: BlockPos, state: BlockState): BlockEntity = ExampleBlockEntity(pos, state)

    override fun <T : BlockEntity> getTicker(
        level: Level, state: BlockState, type: BlockEntityType<T>
    ): BlockEntityTicker<T>? {
        return if (type == ModBlockEntity.EXAMPLE_BLOCK.get()) {
            BlockEntityTicker { lvl, pos, st, be ->
                ExampleBlockEntity.tick(lvl, pos, st, be as ExampleBlockEntity)
            }
        } else null
    }
}