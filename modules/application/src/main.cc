
import luxarise;

int main(){
    auto event_loop = luxarise::window::createNewEventLoop();

    auto window = event_loop->createWindow("My Window", 800, 600);
    auto instance = luxarise::rhi::createInstance(luxarise::rhi::InstanceCreateInfo{});
    while (event_loop->running()) {
        
    }
    return 0;
}