module;
#include <memory>
#include <cstdio>
export module application;

class ApplicationClass{
    public:
        ApplicationClass(){
            std::printf("App name: %s\n", this->NAME);
        }
        ~ApplicationClass(){
            std::printf("App Exit Successfully!!\n");
        }
    private:

        constexpr static auto NAME = "Luxarise";
        constexpr static int APP_WIDTH = 800;
        constexpr static int APP_HEIGHT = 600;
};

export std::unique_ptr<ApplicationClass> make_application()
{
    return std::make_unique<ApplicationClass>();
}

export using Application = std::unique_ptr<ApplicationClass>;
