package rose.runeheart

import rose.runeheart.Runeheart.LOGGER
import java.io.File
import java.nio.ByteBuffer
import kotlin.io.path.createTempFile

typealias NativeContextHandle = Long;
typealias NativeRenderContextHandle = Long;

object Native {
    init {
        val (path, stream) = listOf(
            "/natives/runelib.dll", "/natives/librunelib.dylib"
        ).asSequence().mapNotNull { p -> Native::class.java.getResourceAsStream(p)?.let { p to it } }.first()

        val suffix = path.substringAfterLast('.', "")
        val file = File(System.getProperty("java.io.tmpdir"), "runeheart-runelib.$suffix")

        stream.use { input -> file.outputStream().use { input.copyTo(it) } }
        file.deleteOnExit()

        System.load(file.absolutePath)
    }

    @JvmStatic
    external fun createContext(script: String): NativeContextHandle

    @JvmStatic
    external fun deleteContext(context: NativeContextHandle)

    @JvmStatic
    external fun tick(context: NativeContextHandle)


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
    external fun renderExampleBlock(context: NativeRenderContextHandle, mouseX: Int, mouseY: Int, guiScale: Float)
}


// TODO: use this instead of a global render context handle
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
    fun render(mouseX: Int, mouseY: Int, guiScale: Float) {
        return Native.renderExampleBlock(handle, mouseX, mouseY, guiScale);
    }

    override fun close() {
        if (handle != 0L) {
            Native.deleteRenderContext(handle)
        }
    }
}

class ScriptContext(val script: String) : AutoCloseable {
    var handle: NativeContextHandle = 0;

    init {
        handle = try {
            Native.createContext(script)
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