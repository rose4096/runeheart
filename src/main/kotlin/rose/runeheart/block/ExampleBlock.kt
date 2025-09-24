package rose.runeheart.block

import com.mojang.serialization.MapCodec
import net.minecraft.core.BlockPos
import net.minecraft.network.chat.Component
import net.minecraft.server.level.ServerPlayer
import net.minecraft.world.InteractionResult
import net.minecraft.world.MenuProvider
import net.minecraft.world.SimpleMenuProvider
import net.minecraft.world.entity.player.Player
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.BaseEntityBlock
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.RenderShape
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.entity.BlockEntityTicker
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockBehaviour
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.phys.BlockHitResult
import rose.runeheart.blockentity.ExampleBlockEntity
import rose.runeheart.blockentity.ModBlockEntity
import rose.runeheart.menu.ExampleBlockMenu


class ExampleBlock(props: Properties) : BaseEntityBlock(props) {
    companion object {
        val EXAMPLE_BLOCK_CODEC: MapCodec<ExampleBlock> = simpleCodec(::ExampleBlock)
    }

    override fun codec(): MapCodec<out BaseEntityBlock> = EXAMPLE_BLOCK_CODEC

    override fun getRenderShape(state: BlockState): RenderShape {
        return RenderShape.MODEL
    }

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

    override fun useWithoutItem(
        state: BlockState,
        level: Level,
        pos: BlockPos,
        player: Player,
        hitResult: BlockHitResult
    ): InteractionResult {
        if (!level.isClientSide && player is ServerPlayer) {
            val entity = level.getBlockEntity(pos)
            if (entity is ExampleBlockEntity) {
                player.openMenu(state.getMenuProvider(level, pos)) { buf ->
                    buf.writeBlockPos(pos);
                    buf.writeNullable(entity.getRenderData()) { buffer, value ->
                        buffer.writeByteArray(value)
                    }
                }
            }
        }

        return InteractionResult.SUCCESS
    }
}