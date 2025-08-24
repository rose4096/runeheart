package rose.runeheart

import org.apache.logging.log4j.core.Logger
import rose.runeheart.Runeheart.LOGGER
import rose.runeheart.blockentity.ExampleBlockEntity.RelativeBlockEntity
import java.nio.ByteBuffer
import java.nio.file.Files

typealias NativeContextHandle = Long;
typealias NativeRenderContextHandle = Long;

object Native {
    var renderContext: NativeRenderContextHandle = 0

    init {
        val resPath = "/natives/runelib.dll"
        val inStream = Native::class.java.getResourceAsStream(resPath)
            ?: throw UnsatisfiedLinkError("Native not found in resources: $resPath")

        val file = Files.createTempFile("runeheart-", "runelib.dll").toFile()

        file.deleteOnExit()
        inStream.use { input -> file.outputStream().use { input.copyTo(it) } }

        System.load(file.absolutePath)
    }

    @JvmStatic
    external fun createContext(script: String): NativeContextHandle

    @JvmStatic
    external fun deleteContext(context: NativeContextHandle)

    @JvmStatic
    external fun tick(context: NativeContextHandle)


    @JvmStatic
    external fun createRenderContext(width: Long, height: Long): NativeRenderContextHandle
    @JvmStatic
    external fun deleteRenderContext(context: NativeRenderContextHandle)

    @JvmStatic
    external fun getPixelBuffer(context: NativeRenderContextHandle): ByteBuffer

    @JvmStatic
    external fun resizePixelBuffer(context: NativeRenderContextHandle, width: Long, height: Long): ByteBuffer

    @JvmStatic
    external fun render(context: NativeRenderContextHandle)
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