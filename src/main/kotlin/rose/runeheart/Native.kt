package rose.runeheart

import org.apache.logging.log4j.core.Logger
import rose.runeheart.Runeheart.LOGGER
import rose.runeheart.blockentity.ExampleBlockEntity.RelativeBlockEntity
import java.nio.file.Files

typealias NativeContextHandle = Long;

object Native {
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
    external fun createContext(name: String, script: String): NativeContextHandle

    @JvmStatic
    external fun deleteContext(context: NativeContextHandle)

    @JvmStatic
    external fun tick(context: NativeContextHandle)
}

class ScriptContext(name: String, script: String) : AutoCloseable {
    var handle: NativeContextHandle = 0;

    init {
        handle = try {
            Native.createContext(name, script)
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