#include <cstdio>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <array>

std::string exec(const char* cmd) {
    std::array<char, 128> buffer;
    std::string result;
    std::shared_ptr<FILE> pipe(popen(cmd, "r"), pclose);
    if (!pipe) throw std::runtime_error("popen() failed!");
    while (!feof(pipe.get())) {
        if (fgets(buffer.data(), 128, pipe.get()) != NULL)
            result += buffer.data();
    }
    return result;
}
int main() {
    std::cout << "\n-----------------------------------------------------\n";
    std::cout << "This is an example app to exercise the update client!\n";
    std::cout << "Checking for updates...\n";

    std::cout << exec("../../target/release/update_client");

    std::cout << "Update is ready to apply\n";
    std::cout << "Applying update...\n";
    std::cout << "Update has been applied in ./updates/\n";

    std::cout << "All done!\n";
    std::cout << "\n-----------------------------------------------------\n";
}