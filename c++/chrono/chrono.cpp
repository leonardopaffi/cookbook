#include <iostream>
#include <chrono>

int main()
{
    // Start Timer
    std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();

    // Random stuff
    for(int i = 0; i < 10000; i++)
        std::cout << "n" << i << std::endl;

    // Stop Timer
    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();

    std::cout << "Time difference = " << std::chrono::duration_cast<std::chrono::seconds>(end - begin).count() << "[s]" << std::endl;
    std::cout << "Time difference = " << std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count() << "[ms]" << std::endl;
    std::cout << "Time difference = " << std::chrono::duration_cast<std::chrono::microseconds>(end - begin).count() << "[us]" << std::endl;
    std::cout << "Time difference = " << std::chrono::duration_cast<std::chrono::nanoseconds> (end - begin).count() << "[ns]" << std::endl;
}