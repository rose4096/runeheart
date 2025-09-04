package rose.runeheart

import com.sun.jna.NativeLibrary
import rose.runeheart.Runeheart.LOGGER
import java.io.File
import java.nio.ByteBuffer
import java.nio.file.Files

typealias NativeContextHandle = Long;
typealias NativeRenderContextHandle = Long;

object Native {
    var loadedFile: File? = null

    init {
        val resPath = "/natives/runelib.dll"
        val inStream = Native::class.java.getResourceAsStream(resPath)
            ?: throw UnsatisfiedLinkError("Native not found in resources: $resPath")

        val file = Files.createTempFile("runeheart-", "runelib.dll").toFile()

        file.deleteOnExit()
        inStream.use { input -> file.outputStream().use { input.copyTo(it) } }


        loadedFile = file
        System.load(loadedFile!!.absolutePath)
    }

    fun reload() {
        NativeLibrary.getInstance(loadedFile!!.name).close();
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
    external fun onKeyReleased(context: NativeRenderContextHandle)

    @JvmStatic
    external fun onMousePressed(context: NativeRenderContextHandle, button: Int)

    @JvmStatic
    external fun onMouseReleased(context: NativeRenderContextHandle)

    @JvmStatic
    external fun onMouseScrolled(context: NativeRenderContextHandle, scrollX: Double, scrollY: Double)

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

    fun onKeyReleased() {
        Native.onKeyReleased(handle);
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

    fun onCharacterTyped(character: Char) {}

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