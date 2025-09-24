package rose.runeheart

import net.minecraft.world.level.block.entity.BlockEntity
import rose.runeheart.Runeheart.LOGGER
import rose.runeheart.blockentity.ExampleBlockEntity
import java.io.File
import java.nio.ByteBuffer

typealias NativeContextHandle = Long;
typealias NativeRenderContextHandle = Long;

object Native {
    // TODO: clean this up
    init {
        val mapped = System.mapLibraryName("runelib")
        val res = "/natives/$mapped"
        val bytes = Native::class.java.getResourceAsStream(res)
            ?.use { it.readAllBytes() }
            ?: error("runelib not found: $res")

        val sha16 = java.security.MessageDigest.getInstance("SHA-256")
            .digest(bytes).joinToString("") { "%02x".format(it) }.take(16)

        val dir = java.nio.file.Path.of(System.getProperty("java.io.tmpdir"), "runeheart", "cache")
        java.nio.file.Files.createDirectories(dir)
        val target = dir.resolve("$mapped-$sha16")

        if (!java.nio.file.Files.exists(target)) {
            java.nio.file.Files.write(target, bytes)
            try { target.toFile().deleteOnExit() } catch (_: Throwable) {}
        }

        System.load(target.toAbsolutePath().toString())
    }


    @JvmStatic
    external fun createContext(): NativeContextHandle

    @JvmStatic
    external fun deleteContext(context: NativeContextHandle)

    @JvmStatic
    external fun tick(context: NativeContextHandle, obj: BlockEntity)

    @JvmStatic
    external fun constructExampleBlockRenderData(context: NativeContextHandle): ByteArray

    @JvmStatic
    external fun updateScriptContextFromRenderData(context: NativeContextHandle, renderData: ByteArray)

    @JvmStatic
    external fun createRenderContext(width: Int, height: Int): NativeRenderContextHandle

    @JvmStatic
    external fun deleteRenderContext(context: NativeRenderContextHandle)

    @JvmStatic
    external fun getPixelBuffer(context: NativeRenderContextHandle): ByteBuffer

    @JvmStatic
    external fun resizePixelBuffer(context: NativeRenderContextHandle, width: Int, height: Int): ByteBuffer

    @JvmStatic
    external fun onKeyPressed(context: NativeRenderContextHandle, keyCode: Int, scanCode: Int, modifiers: Int)

    @JvmStatic
    external fun onKeyReleased(context: NativeRenderContextHandle, keyCode: Int, scanCode: Int, modifiers: Int)

    @JvmStatic
    external fun onMousePressed(context: NativeRenderContextHandle, button: Int)

    @JvmStatic
    external fun onMouseReleased(context: NativeRenderContextHandle)

    @JvmStatic
    external fun onMouseScrolled(context: NativeRenderContextHandle, scrollX: Double, scrollY: Double)

    @JvmStatic
    external fun onCharacterTyped(context: NativeRenderContextHandle, codePoint: Char, modifiers: Int)

    @JvmStatic
    external fun renderExampleBlock(
        renderContext: NativeRenderContextHandle,
        mouseX: Int,
        mouseY: Int,
        guiScale: Float,
        renderData: ByteArray
    ): ByteArray?
}


class RenderContext(val width: Int, val height: Int) : AutoCloseable {
    var handle: NativeRenderContextHandle = 0;

    init {
        handle = try {
            Native.createRenderContext(width, height)
        } catch (e: RuntimeException) {
            LOGGER.error(e.message)
            0L
        }
    }

    fun valid(): Boolean {
        return handle != 0L;
    }

    fun getPixelBuffer(): ByteBuffer {
        return Native.getPixelBuffer(handle);
    }

    fun resizePixelBuffer(width: Int, height: Int): ByteBuffer {
        return Native.resizePixelBuffer(handle, width, height);
    }

    fun onKeyPressed(keyCode: Int, scanCode: Int, modifiers: Int) {
        Native.onKeyPressed(handle, keyCode, scanCode, modifiers);
    }

    fun onKeyReleased(keyCode: Int, scanCode: Int, modifiers: Int) {
        Native.onKeyReleased(handle, keyCode, scanCode, modifiers);
    }

    fun onMousePressed(button: Int) {
        Native.onMousePressed(handle, button);
    }

    fun onMouseReleased() {
        Native.onMouseReleased(handle);
    }

    fun onMouseScrolled(scrollX: Double, scrollY: Double) {
        Native.onMouseScrolled(handle, scrollX, scrollY);
    }

    fun onCharacterTyped(codePoint: Char, modifiers: Int) {
        Native.onCharacterTyped(handle, codePoint, modifiers);
    }

    // TODO: maybe override this and then have the native funciton be provided so like ScreenRenderContext and
    //       make this funciton overridable .
    fun render(mouseX: Int, mouseY: Int, guiScale: Float, renderData: ByteArray): ByteArray? {
        return Native.renderExampleBlock(handle, mouseX, mouseY, guiScale, renderData);
    }

    override fun close() {
        if (handle != 0L) {
            Native.deleteRenderContext(handle)
        }
    }
}

class ScriptContext() : AutoCloseable {
    var handle: NativeContextHandle = 0;

    init {
        handle = try {
            Native.createContext()
        } catch (e: RuntimeException) {
            LOGGER.error(e.message)
            0L
        }
    }

    override fun close() {
        if (handle != 0L) {
            Native.deleteContext(handle);
        }
    }
}