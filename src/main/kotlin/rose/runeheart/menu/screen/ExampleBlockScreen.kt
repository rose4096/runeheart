package rose.runeheart.menu.screen

import com.mojang.blaze3d.systems.RenderSystem
import net.minecraft.client.Minecraft
import net.minecraft.client.gui.GuiGraphics
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
import net.minecraft.client.renderer.texture.DynamicTexture
import net.minecraft.network.chat.Component
import net.minecraft.resources.ResourceLocation
import net.minecraft.world.entity.player.Inventory
import org.lwjgl.opengl.GL11
import rose.runeheart.Native
import rose.runeheart.menu.ExampleBlockMenu
import java.nio.ByteBuffer

class ExampleBlockScreen(menu: ExampleBlockMenu, inv: Inventory, title: Component) :
    AbstractContainerScreen<ExampleBlockMenu>(menu, inv, title) {

    private var pixelBuffer: ByteBuffer? = null;
    private var texture: DynamicTexture? = null;
    private var resource: ResourceLocation? = null;

    override fun init() {
        super.init()

        if (Native.renderContext == 0L) {
            Native.renderContext = Native.createRenderContext(width, height);
        }

        this.resizeTexture();
    }

    fun resizeTexture() {
        resource?.let { minecraft?.textureManager?.release(it) }
        texture?.close()

        Native.resizePixelBuffer(Native.renderContext, width, height);
        texture = DynamicTexture(width, height, false)
        resource = minecraft?.textureManager?.register("runeheart_gui_tex", texture!!)
        pixelBuffer = Native.getPixelBuffer(Native.renderContext);
    }

    override fun resize(minecraft: Minecraft, width: Int, height: Int) {
        super.resize(minecraft, width, height)

        this.resizeTexture();
    }

    override fun renderBg(gui: GuiGraphics, tick: Float, mouseX: Int, mouseY: Int) {
        this.renderBlurredBackground(tick)
    }

    override fun render(gui: GuiGraphics, mouseX: Int, mouseY: Int, tick: Float) {
        // not calling super so we can just render stuff;;;
        renderBg(gui, tick, mouseX, mouseY)

        for (renderable in renderables) {
            renderable.render(gui, mouseX, mouseY, tick)
        }

        if (Native.renderContext == 0L) return;

        Native.render(Native.renderContext, mouseX, mouseY);

        if (pixelBuffer == null || texture == null) return;

        if (pixelBuffer!!.capacity() != (texture!!.pixels!!.width * texture!!.pixels!!.height * 4)) {
            this.resizeTexture();
            return;
        }

        RenderSystem.enableBlend();
        RenderSystem.bindTexture(texture!!.id)

        GL11.glBlendFunc(GL11.GL_SRC_ALPHA, GL11.GL_ONE_MINUS_SRC_ALPHA);
        GL11.glPixelStorei(GL11.GL_UNPACK_ALIGNMENT, 1)
        GL11.glTexSubImage2D(
            GL11.GL_TEXTURE_2D, 0, 0, 0,
            texture!!.pixels!!.width, texture!!.pixels!!.height,
            GL11.GL_RGBA, GL11.GL_UNSIGNED_BYTE,
            pixelBuffer!!
        )

        gui.blit(resource!!, 0, 0, 0f, 0f, width, height, width, height)
    }
}