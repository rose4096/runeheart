package rose.runeheart

import java.nio.file.Files

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

    @JvmStatic external fun hello(input: String): String
}