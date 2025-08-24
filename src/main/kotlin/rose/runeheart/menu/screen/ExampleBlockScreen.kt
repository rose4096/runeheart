package rose.runeheart.menu.screen

import com.mojang.blaze3d.systems.RenderSystem
import com.mojang.blaze3d.vertex.BufferUploader
import com.mojang.blaze3d.vertex.DefaultVertexFormat
import com.mojang.blaze3d.vertex.Tesselator
import com.mojang.blaze3d.vertex.VertexFormat
import net.minecraft.client.Minecraft
import net.minecraft.client.gui.GuiGraphics
import net.minecraft.client.gui.components.AbstractWidget
import net.minecraft.client.gui.narration.NarratedElementType
import net.minecraft.client.gui.narration.NarrationElementOutput
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen
import net.minecraft.client.renderer.texture.DynamicTexture
import net.minecraft.network.chat.Component
import net.minecraft.resources.ResourceLocation
import net.minecraft.util.FastColor.ARGB32
import net.minecraft.world.entity.player.Inventory
import org.joml.Matrix4f
import rose.runeheart.Native
import rose.runeheart.Runeheart
import rose.runeheart.menu.ExampleBlockMenu
import java.nio.ByteBuffer

// no translations for now just literal titles
class CodeEditBox(x: Int, y: Int, val padding: Int, title: String) :
    AbstractWidget(x, y, 0, 0, Component.literal(title)) {

    private var initialized = false

    override fun renderWidget(
        gui: GuiGraphics,
        mouseX: Int,
        mouseY: Int,
        tick: Float
    ) {
        if (!initialized) {
            this.height = gui.guiHeight() - padding
            this.width = gui.guiWidth() - padding
            this.x = x
            this.y += padding

            initialized = true;
        }

        this.isHovered = gui.containsPointInScissor(
            mouseX,
            mouseY
        ) && mouseX >= x && mouseY >= y && mouseX < this.width && mouseY < this.height;


        gui.fill(x, y, this.width, this.height, 0, ARGB32.color(75, 0, 0, 0))

        if (this.isHovered) {
            gui.renderOutline(
                x,
                y,
                this.width - x,
                this.height - y,
                ARGB32.color(75, 255, 255, 255)
            )
        }
    }

    override fun updateWidgetNarration(out: NarrationElementOutput) {
        out.add(NarratedElementType.TITLE, this.createNarrationMessage())
    }
}

class ExampleBlockScreen(menu: ExampleBlockMenu, inv: Inventory, title: Component) :
    AbstractContainerScreen<ExampleBlockMenu>(menu, inv, title) {

    private var pixelBuffer: ByteBuffer? = null;
    private var texture: DynamicTexture? = null;
    private var resource: ResourceLocation? = null;
    private var windowWidth: Int = 0
    private var windowHeight: Int = 0

    override fun init() {
        super.init()

//        windowWidth = minecraft?.window?.width ?: 0;
//        windowHeight = minecraft?.window?.height ?: 0;

        windowWidth = width;
        windowHeight = height;

        if (Native.renderContext == 0L) {
            Native.renderContext = Native.createRenderContext(windowWidth.toLong(), windowHeight.toLong());
        }

        texture = DynamicTexture(windowWidth, windowHeight, false)
        resource = minecraft?.textureManager?.register("runeheart_gui_tex", texture!!)

        pixelBuffer = Native.getPixelBuffer(Native.renderContext);

        //        this.addRenderableWidget(CodeEditBox(width / 2, 0, 10, "edit script"))
    }

    override fun resize(minecraft: Minecraft, width: Int, height: Int) {
        super.resize(minecraft, width, height)

//        windowWidth = minecraft.window.width
//        windowHeight = minecraft.window.height

        if (width <= 1 || height <= 1) return

        windowWidth = width;
        windowHeight = height;

        pixelBuffer = Native.resizePixelBuffer(Native.renderContext, windowWidth.toLong(), windowHeight.toLong())

        texture?.close()
        texture = DynamicTexture(windowWidth, windowHeight, false)
        resource = minecraft.textureManager.register("runeheart_gui_tex", texture!!)
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

        if (this.width != windowWidth || this.height != windowHeight) {
            windowWidth = this.width
            windowHeight = this.height

            pixelBuffer = Native.resizePixelBuffer(Native.renderContext, this.width.toLong(), this.height.toLong())
            texture?.close()
            texture = DynamicTexture(this.width, this.height, false)
            resource = minecraft!!.textureManager.register("runeheart_gui_tex", texture!!)
        }

        if (Native.renderContext != 0L) {
            Native.render(Native.renderContext);

            pixelBuffer = Native.getPixelBuffer(Native.renderContext);
            if (pixelBuffer != null) {
                pixelBuffer!!.rewind();
                for (yy in 0 until windowHeight) {
                    for (xx in 0 until windowWidth) {
                        val r = pixelBuffer!!.get().toInt() and 0xFF
                        val g = pixelBuffer!!.get().toInt() and 0xFF
                        val b = pixelBuffer!!.get().toInt() and 0xFF
                        val a = pixelBuffer!!.get().toInt() and 0xFF
                        val abgr = (a shl 24) or (b shl 16) or (g shl 8) or r
                        texture?.pixels?.setPixelRGBA(xx, yy, abgr)
                    }
                }
                texture?.upload();

                RenderSystem.disableScissor()
                gui.blit(
                    resource!!,
                    0, 0,
                    0f, 0f,
                    windowWidth, windowHeight,
                    windowWidth, windowHeight,
                )
            }
        }
    }
}