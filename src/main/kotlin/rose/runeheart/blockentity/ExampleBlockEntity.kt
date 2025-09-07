package rose.runeheart.blockentity

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.network.chat.Component
import net.minecraft.world.MenuProvider
import net.minecraft.world.entity.player.Inventory
import net.minecraft.world.entity.player.Player
import net.minecraft.world.inventory.AbstractContainerMenu
import net.minecraft.world.inventory.ContainerLevelAccess
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.neoforged.neoforge.capabilities.Capabilities
import net.neoforged.neoforge.client.extensions.IMenuProviderExtension
import rose.runeheart.Native
import rose.runeheart.ScriptContext
import rose.runeheart.menu.ExampleBlockMenu
import rose.runeheart.menu.RuneScriptUI

class ExampleBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(ModBlockEntity.EXAMPLE_BLOCK.get(), pos, state), MenuProvider, IMenuProviderExtension {

    var scripts: MutableMap<String, ScriptContext> = hashMapOf()

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

//            blockEntity.scripts["rune"]?.let {
//                Native.tick(it.handle)
//            }
        }
    }

    override fun createMenu(
        id: Int,
        inv: Inventory,
        player: Player
    ): ExampleBlockMenu {
        return ExampleBlockMenu(
            id,
            inv,
            ContainerLevelAccess.create(level!!, blockPos),
            blockPos,
            this.scripts.map { (name, context) -> RuneScriptUI(name, context.script) })
    }

    override fun getDisplayName(): Component {
        return Component.literal("asdf")
    }
}