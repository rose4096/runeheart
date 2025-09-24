package rose.runeheart.blockentity

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.Serializable
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
import kotlinx.serialization.cbor.Cbor
import kotlinx.serialization.decodeFromByteArray
import kotlinx.serialization.encodeToByteArray
import rose.runeheart.Runeheart

@Serializable
data class ExampleBlockRenderData(
    val scripts: List<List<String>>
)

@OptIn(ExperimentalSerializationApi::class)
fun ExampleBlockRenderData.toBytes(): ByteArray = Cbor.encodeToByteArray(this)

fun ByteArray.toExampleBlockRenderData(): ExampleBlockRenderData? =
    runCatching { Cbor.decodeFromByteArray<ExampleBlockRenderData>(this) }
        .onFailure { e -> Runeheart.LOGGER.error("CBOR decode failed", e) }
        .getOrNull()

class ExampleBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(ModBlockEntity.EXAMPLE_BLOCK.get(), pos, state), MenuProvider {

    var scriptContext: ScriptContext? = null;
    var renderData: ByteArray? = null;

    data class RelativeBlockEntity(val entity: BlockEntity?, val side: Direction)

    fun getSurroundingBlockEntities(level: Level, relative: BlockPos): List<RelativeBlockEntity> {
        return Direction.entries.map {
            RelativeBlockEntity(level.getBlockEntity(relative.relative(it)), it)
        }
    }

    fun test_get_data(): Int = 420

    companion object {
        fun tick(level: Level, pos: BlockPos, state: BlockState, blockEntity: ExampleBlockEntity) {
            if (level.isClientSide) return;

            if (blockEntity.scriptContext == null) {
                blockEntity.scriptContext = ScriptContext()
            }

            if (blockEntity.renderData != null) {
                Native.updateScriptContextFromRenderData(blockEntity.scriptContext!!.handle, blockEntity.renderData!!)
            }

            val itemHandlers = blockEntity.getSurroundingBlockEntities(level, pos).mapNotNull {
                if (it.entity == null) return@mapNotNull null

                level.getCapability(
                    Capabilities.ItemHandler.BLOCK,
                    it.entity.blockPos,
                    it.side.opposite
                )
            }

//            itemHandlers.forEach {
//                it.insertItem()
//            }

            blockEntity.scriptContext?.let {
                Native.tick(it.handle, blockEntity);
            }
        }
    }

    fun updateRenderData(data: ByteArray) {
        renderData = data
    }

    fun getActiveRenderData(): ByteArray? {
        if (renderData == null) {
            renderData = scriptContext?.handle?.let { Native.constructExampleBlockRenderData(it) }
        }

        return renderData
    }

    override fun createMenu(
        id: Int,
        inv: Inventory,
        player: Player
    ): ExampleBlockMenu {
        // NOTE: maybe will remove toExampleBlockRenderData / cbor, but keeping it for now just in case its needed
        // but i think the kotlin side shouldnt need any render data. tldr; maybe switch to raw rust transmutations
        // instead of doing any encoding/decoding
        val data = getActiveRenderData()//?.toExampleBlockRenderData();
        return ExampleBlockMenu(
            id,
            inv,
            ContainerLevelAccess.create(level!!, blockPos),
            blockPos,
            data
        )
    }

    override fun getDisplayName(): Component {
        return Component.literal("asdf")
    }
}
