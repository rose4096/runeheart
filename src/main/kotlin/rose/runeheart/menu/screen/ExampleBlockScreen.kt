package rose.runeheart.menu.screen

import com.mojang.blaze3d.platform.InputConstants
import com.mojang.blaze3d.systems.RenderSystem
import net.minecraft.client.Minecraft
import net.minecraft.client.gui.GuiGraphics
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
import net.minecraft.client.renderer.texture.DynamicTexture
import net.minecraft.network.chat.Component
import net.minecraft.resources.ResourceLocation
import net.minecraft.world.entity.player.Inventory
import org.lwjgl.opengl.GL11
import rose.runeheart.RenderContext
import rose.runeheart.menu.ExampleBlockMenu
import java.nio.ByteBuffer

class ExampleBlockScreen(menu: ExampleBlockMenu, inv: Inventory, title: Component) :
    AbstractContainerScreen<ExampleBlockMenu>(menu, inv, title) {

    private var renderContext: RenderContext? = null
    private var pixelBuffer: ByteBuffer? = null;
    private var texture: DynamicTexture? = null;
    private var resource: ResourceLocation? = null;

    override fun init() {
        super.init()

        if (this.renderContext == null) {
            this.renderContext = RenderContext(minecraft!!.window.width, minecraft!!.window.height);
        }

        this.resizeTexture();
    }

    fun resizeTexture() {
        resource?.let { minecraft?.textureManager?.release(it) }
        texture?.close()

        renderContext?.resizePixelBuffer(minecraft!!.window.width, minecraft!!.window.height);
        texture = DynamicTexture(minecraft!!.window.width, minecraft!!.window.height, false)
        resource = minecraft?.textureManager?.register("runeheart_gui_tex", texture!!)
        pixelBuffer = renderContext?.getPixelBuffer();
    }

    override fun resize(minecraft: Minecraft, width: Int, height: Int) {
        super.resize(minecraft, width, height)

        this.resizeTexture();
    }

    override fun renderBg(gui: GuiGraphics, tick: Float, mouseX: Int, mouseY: Int) {
        this.renderBlurredBackground(tick)
    }

    override fun keyPressed(keyCode: Int, scanCode: Int, modifiers: Int): Boolean {
        val key = InputConstants.getKey(keyCode, scanCode)
        if (minecraft!!.options.keyInventory.isActiveAndMatches(key)) {
            return false;
        }

        renderContext?.onKeyPressed(keyCode, scanCode, modifiers);
        return super.keyPressed(keyCode, scanCode, modifiers)
    }

    override fun keyReleased(keyCode: Int, scanCode: Int, modifiers: Int): Boolean {
        renderContext?.onKeyReleased(keyCode, scanCode, modifiers)
        return super.keyReleased(keyCode, scanCode, modifiers);
    }

    override fun mouseClicked(mouseX: Double, mouseY: Double, button: Int): Boolean {
        renderContext?.onMousePressed(button);
        return super.mouseClicked(mouseX, mouseY, button)
    }

    override fun mouseReleased(mouseX: Double, mouseY: Double, button: Int): Boolean {
        renderContext?.onMouseReleased();
        return super.mouseReleased(mouseX, mouseY, button);
    }

    override fun mouseScrolled(mouseX: Double, mouseY: Double, scrollX: Double, scrollY: Double): Boolean {
        renderContext?.onMouseScrolled(scrollX, scrollY);
        return super.mouseScrolled(mouseX, mouseY, scrollX, scrollY)
    }

    override fun charTyped(codePoint: Char, modifiers: Int): Boolean {
        renderContext?.onCharacterTyped(codePoint, modifiers);
        return super.charTyped(codePoint, modifiers)
    }

    override fun render(gui: GuiGraphics, mouseX: Int, mouseY: Int, tick: Float) {
        // not calling super so we can just render stuff;;;
        renderBg(gui, tick, mouseX, mouseY)

        for (renderable in renderables) {
            renderable.render(gui, mouseX, mouseY, tick)
        }

        if (renderContext == null || renderContext?.valid() == false) return;

        renderContext?.render(mouseX, mouseY, minecraft!!.window.guiScale.toFloat());

        if (pixelBuffer == null || texture == null) return;

        if (pixelBuffer!!.capacity() != (texture!!.pixels!!.width * texture!!.pixels!!.height * 4)) {
            this.resizeTexture();
            return;
        }

        RenderSystem.enableBlend();
        RenderSystem.bindTexture(texture!!.id)

        GL11.glBlendFunc(GL11.GL_ONE, GL11.GL_ONE_MINUS_SRC_ALPHA);
        GL11.glPixelStorei(GL11.GL_UNPACK_ALIGNMENT, 1)

        GL11.glTexSubImage2D(
            GL11.GL_TEXTURE_2D, 0, 0, 0,
            texture!!.pixels!!.width, texture!!.pixels!!.height,
            GL11.GL_RGBA, GL11.GL_UNSIGNED_BYTE,
            pixelBuffer!!
        )

        // we are removing minecrafts gui scaling so we can render our texture at full resolution
        // upon the entire window size. i dont fully like this approach, but its the best option
        // for now i think.
        val scale = minecraft!!.window.guiScale.toFloat();
        val pose = gui.pose();
        pose.pushPose();
        pose.scale(1f / scale, 1f / scale, 1f);

        gui.blit(
            resource!!,
            0,
            0,
            0f,
            0f,
            minecraft!!.window.width,
            minecraft!!.window.height,
            minecraft!!.window.width,
            minecraft!!.window.height
        )

        pose.popPose();
    }

    override fun onClose() {
        this.renderContext?.close();

        super.onClose()
    }
}