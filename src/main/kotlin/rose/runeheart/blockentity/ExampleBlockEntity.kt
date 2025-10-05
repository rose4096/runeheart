package rose.runeheart.blockentity

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.cbor.Cbor
import kotlinx.serialization.encodeToByteArray
import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.core.registries.BuiltInRegistries
import net.minecraft.network.chat.Component
import net.minecraft.world.MenuProvider
import net.minecraft.world.entity.player.Inventory
import net.minecraft.world.entity.player.Player
import net.minecraft.world.inventory.ContainerLevelAccess
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.neoforged.neoforge.capabilities.Capabilities
import rose.runeheart.Native
import rose.runeheart.ScriptContext
import rose.runeheart.menu.ExampleBlockMenu


// current ideas:
// build jobject array, every script entity gets an index into its actual object
// we have a raw index and then important script data within the entity itself

@Serializable
data class ScriptableItem(
    @SerialName("slot_index") val slotIndex: Long,
    val name: String,
    val tags: List<String>,
    val count: Int,
)

@Serializable
data class ScriptableBlockEntity(
    @SerialName("raw_access_index") val rawAccessIndex: Long,
    @Serializable(with = BlockPosSerializer::class) @SerialName("block_pos") val blockPos: BlockPos,
    val dimension: String,
    val name: String,
    val items: List<ScriptableItem>
)

data class RawScriptableBlockEntity(
    val blockEntity: BlockEntity,
)

@OptIn(ExperimentalSerializationApi::class)
fun List<ScriptableBlockEntity>.toBytes(): ByteArray = Cbor.encodeToByteArray(this)

class ExampleBlockEntity(pos: BlockPos, state: BlockState) :
    BlockEntity(ModBlockEntity.EXAMPLE_BLOCK.get(), pos, state), MenuProvider {

    var scriptContext: ScriptContext? = null;
    var renderData: ByteArray? = null;
    var rawScriptableEntities: List<RawScriptableBlockEntity> = listOf()
    var scriptableEntities: List<ScriptableBlockEntity> = listOf()

    data class RelativeBlockEntity(val entity: BlockEntity?, val side: Direction)

    fun getSurroundingBlockEntities(level: Level, relative: BlockPos): List<RelativeBlockEntity> {
        return Direction.entries.map {
            RelativeBlockEntity(level.getBlockEntity(relative.relative(it)), it)
        }
    }

    // MOVE API./. NEEDS TO SUPPORT SIDES.
    // NEEDS TO SUPPORT FORGE TAGS TOO ON CHESTS MAYBE ??? omg

    // TODO: need to decode scriptable item to get raw acecss index and stfuf
    fun moveItem(srcRaw: RawScriptableBlockEntity, dstRaw: RawScriptableBlockEntity, itemSlotIndex: Int, face: Direction, amount: Int?) {
        val src = srcRaw.blockEntity;
        val dst = dstRaw.blockEntity;

        val srcLevel = src.level ?: return
        val dstLevel = dst.level ?: return
        val srcPos = src.blockPos
        val dstPos = dst.blockPos

        // ignore face on src we dont care
        val srcHandler = srcLevel.getCapability(Capabilities.ItemHandler.BLOCK, srcPos, null) ?: return;
        val dstHandler = dstLevel.getCapability(Capabilities.ItemHandler.BLOCK, dstPos, face) ?: return;

        val src_stack = srcHandler.getStackInSlot(itemSlotIndex)
        if (!src_stack.isEmpty) {
            val stack = srcHandler.extractItem(itemSlotIndex, amount ?: src_stack.count, false);
            run breaking@ {
                (0..<dstHandler.slots).forEach { if (dstHandler.insertItem(it, stack, false).isEmpty) return@breaking }
            }
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

            blockEntity.rawScriptableEntities = blockEntity.getSurroundingBlockEntities(level, pos)
                .mapNotNull { it.entity?.let { it1 -> RawScriptableBlockEntity(it1) } }
            blockEntity.scriptableEntities = blockEntity.rawScriptableEntities.mapIndexed { i, it ->
                ScriptableBlockEntity(
                    i.toLong(),
                    it.blockEntity.blockPos,
                    it.blockEntity.level!!.dimension().location().toString(),
                    BuiltInRegistries.BLOCK_ENTITY_TYPE.getKey(it.blockEntity.type).toString(),
                    level.getCapability(Capabilities.ItemHandler.BLOCK, it.blockEntity.blockPos, null)?.let {
                        (0..<it.slots).mapNotNull { i ->
                            val stack = it.getStackInSlot(i);
                            if (stack.isEmpty) {
                                return@mapNotNull null
                            }

                            ScriptableItem(
                                i.toLong(),
                                BuiltInRegistries.ITEM.getKey(stack.item).toString(),
                                BuiltInRegistries.ITEM.getHolder(BuiltInRegistries.ITEM.getKey(stack.item))
                                    .orElseThrow().tags().toList().map { item -> item.location().toString() },
                                stack.count
                            )
                        }
                    } ?: listOf())
            }

            blockEntity.scriptContext?.let {
                Native.tick(
                    it.handle,
                    blockEntity,
                    blockEntity.rawScriptableEntities.toTypedArray(),
                    blockEntity.scriptableEntities.toBytes()
                );
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
        id: Int, inv: Inventory, player: Player
    ): ExampleBlockMenu {
        // NOTE: maybe will remove toExampleBlockRenderData / cbor, but keeping it for now just in case its needed
        // but i think the kotlin side shouldnt need any render data. tldr; maybe switch to raw rust transmutations
        // instead of doing any encoding/decoding
        val data = getActiveRenderData()//?.toExampleBlockRenderData();
        return ExampleBlockMenu(
            id, inv, ContainerLevelAccess.create(level!!, blockPos), blockPos, data
        )
    }

    override fun getDisplayName(): Component {
        return Component.literal("asdf")
    }
}
