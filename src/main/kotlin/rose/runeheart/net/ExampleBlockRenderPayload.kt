package rose.runeheart.net

import net.minecraft.core.BlockPos
import net.minecraft.network.codec.ByteBufCodecs
import net.minecraft.network.codec.StreamCodec
import net.minecraft.network.protocol.common.custom.CustomPacketPayload
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.level.ServerLevel
import net.neoforged.neoforge.network.handling.IPayloadContext
import rose.runeheart.blockentity.ExampleBlockEntity
import rose.runeheart.menu.ExampleBlockMenu

class ExampleBlockRenderDataPayload(
    val pos: BlockPos,
    val data: ByteArray,
) : CustomPacketPayload {
    override fun type() = TYPE

    companion object {
        val TYPE = CustomPacketPayload.Type<ExampleBlockRenderDataPayload>(
            ResourceLocation.fromNamespaceAndPath("runeheart", "example_block_render_data")
        )

        val STREAM_CODEC: StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, ExampleBlockRenderDataPayload> =
            StreamCodec.composite(
                BlockPos.STREAM_CODEC, ExampleBlockRenderDataPayload::pos,
                ByteBufCodecs.BYTE_ARRAY, ExampleBlockRenderDataPayload::data,
                ::ExampleBlockRenderDataPayload
            )

        fun handle(payload: ExampleBlockRenderDataPayload, context: IPayloadContext) {
            context.enqueueWork {
                val menu = context.player().containerMenu as? ExampleBlockMenu ?: return@enqueueWork
                if (menu.pos != payload.pos) return@enqueueWork
                menu.renderData = payload.data
            }
        }
    }
}

class ExampleBlockRenderPayload(
    val pos: BlockPos,
    val data: ByteArray,
) : CustomPacketPayload {
    override fun type() = TYPE

    companion object {
        val TYPE = CustomPacketPayload.Type<ExampleBlockRenderPayload>(
            ResourceLocation.fromNamespaceAndPath(
                "runeheart",
                "example_block_render_payload"
            )
        )

        val STREAM_CODEC = StreamCodec.composite(
            BlockPos.STREAM_CODEC, ExampleBlockRenderPayload::pos,
            ByteBufCodecs.BYTE_ARRAY, ExampleBlockRenderPayload::data,
            ::ExampleBlockRenderPayload
        )

        fun handle(payload: ExampleBlockRenderPayload, context: IPayloadContext) {
            context.enqueueWork {
                val player = context.player() ?: return@enqueueWork
                if (player.isSpectator) return@enqueueWork

                val level = (player.level()) as? ServerLevel ?: return@enqueueWork
                if (!level.isLoaded(payload.pos)) return@enqueueWork

                val menu = player.containerMenu as? ExampleBlockMenu ?: return@enqueueWork
                if (menu.pos != payload.pos) return@enqueueWork

                val be = level.getBlockEntity(payload.pos) as? ExampleBlockEntity
                    ?: return@enqueueWork

                be.updateRenderData(payload.data)

                val data = be.getActiveRenderData() ?: return@enqueueWork
                context.reply(
                    ExampleBlockRenderDataPayload(
                        payload.pos,
                        data
                    )
                )
            }
        }
    }
}