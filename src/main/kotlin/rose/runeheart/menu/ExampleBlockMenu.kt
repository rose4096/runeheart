package rose.runeheart.menu

import net.minecraft.core.BlockPos
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.network.RegistryFriendlyByteBuf
import net.minecraft.world.SimpleContainer
import net.minecraft.world.entity.player.Inventory
import net.minecraft.world.entity.player.Player
import net.minecraft.world.inventory.AbstractContainerMenu
import net.minecraft.world.inventory.ContainerData
import net.minecraft.world.inventory.ContainerLevelAccess
import net.minecraft.world.inventory.SimpleContainerData
import net.minecraft.world.inventory.Slot
import net.minecraft.world.item.ItemStack
import rose.runeheart.ScriptContext
import rose.runeheart.block.ModBlocks

// server constructor called by client after reading server data
class ExampleBlockMenu(
    id: Int,
    val inv: Inventory,
    val access: ContainerLevelAccess,
    val pos: BlockPos,
    var renderData: ByteArray?
) :
    AbstractContainerMenu(ModMenu.EXAMPLE_BLOCK.get(), id) {

    // converged initialization from server/client
    init {
        // NOTE: if we want to render EMI/JEI we need at least one slot.
    }

    // client
    constructor(
        id: Int,
        inv: Inventory,
        buf: RegistryFriendlyByteBuf,
    ) : this(
        id,
        inv,
        ContainerLevelAccess.NULL,
        buf.readBlockPos(),
        buf.readNullable(FriendlyByteBuf::readByteArray)
    )

    override fun stillValid(player: Player) = stillValid(access, player, ModBlocks.EXAMPLE_BLOCK)
    override fun quickMoveStack(player: Player, index: Int): ItemStack = ItemStack.EMPTY
}