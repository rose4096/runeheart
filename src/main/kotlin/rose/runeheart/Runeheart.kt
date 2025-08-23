package rose.runeheart

import rose.runeheart.block.ModBlocks
import rose.runeheart.item.ModItems
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.CreativeModeTabs
import net.neoforged.bus.api.SubscribeEvent
import net.neoforged.fml.common.EventBusSubscriber
import net.neoforged.fml.common.Mod
import net.neoforged.fml.event.lifecycle.FMLClientSetupEvent
import net.neoforged.fml.event.lifecycle.FMLCommonSetupEvent
import net.neoforged.fml.event.lifecycle.FMLDedicatedServerSetupEvent
import net.neoforged.neoforge.registries.DeferredRegister
import org.apache.logging.log4j.Level
import org.apache.logging.log4j.LogManager
import org.apache.logging.log4j.Logger
import rose.runeheart.blockentity.ModBlockEntity
import rose.runeheart.menu.ModMenu
import thedarkcolour.kotlinforforge.neoforge.forge.MOD_BUS
import thedarkcolour.kotlinforforge.neoforge.forge.runForDist

@Mod(Runeheart.ID)
@EventBusSubscriber(bus = EventBusSubscriber.Bus.MOD)
object Runeheart {
    const val ID = "runeheart"

    val LOGGER: Logger = LogManager.getLogger(ID)

    val CREATIVE_MODE_TABS = DeferredRegister.create(Registries.CREATIVE_MODE_TAB, ID)
    val RUNEHEART_TAB = CREATIVE_MODE_TABS.register("runeheart_tab") { ->
        CreativeModeTab.builder().title(Component.translatable("itemGroup.runeheart"))
            .withTabsBefore(CreativeModeTabs.COMBAT).icon {
                ModItems.EXAMPLE_BLOCK_ITEM.get().defaultInstance
            }.displayItems { _, out ->
                out.accept(ModItems.EXAMPLE_BLOCK_ITEM)
            }.build()
    }

    init {
        LOGGER.log(Level.INFO, "Hello world!")

        ModBlocks.REGISTRY.register(MOD_BUS)
        ModItems.REGISTRY.register(MOD_BUS)
        ModBlockEntity.REGISTRY.register(MOD_BUS)
        ModMenu.REGISTRY.register(MOD_BUS)
        CREATIVE_MODE_TABS.register(MOD_BUS)

        runForDist(clientTarget = {
            MOD_BUS.addListener(::onClientSetup)

        }, serverTarget = {
            MOD_BUS.addListener(::onServerSetup)
        })
    }

    private fun onClientSetup(event: FMLClientSetupEvent) {
        LOGGER.log(Level.INFO, "Initializing client...")

//        ScriptContext(
//            """
//        pub fn tick() {
//            println!("hello from tick!");
//        }
//        """
//        ).use {
//            Native.tick(it.handle);
//        }
    }

    private fun onServerSetup(event: FMLDedicatedServerSetupEvent) {
        LOGGER.log(Level.INFO, "Server starting...")
    }

    @SubscribeEvent
    fun onCommonSetup(event: FMLCommonSetupEvent) {
        LOGGER.log(Level.INFO, "Hello! This is working!")
    }
}