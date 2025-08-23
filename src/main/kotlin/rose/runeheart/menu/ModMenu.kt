package rose.runeheart.menu

import net.minecraft.core.registries.Registries
import net.neoforged.neoforge.common.extensions.IMenuTypeExtension
import net.neoforged.neoforge.registries.DeferredRegister
import rose.runeheart.Runeheart

object ModMenu {
    val REGISTRY = DeferredRegister.create(Registries.MENU, Runeheart.ID)

    val EXAMPLE_BLOCK = REGISTRY.register("example_menu") { ->
        IMenuTypeExtension.create(::ExampleBlockMenu)
    }
}