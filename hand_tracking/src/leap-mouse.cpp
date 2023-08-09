#include <iostream>
#include <LeapListener.h>

/**
 * @brief Create an event listener for the controller
 * to implement the CustomListener class
 */

static CustomListener listener;
static Leap::Controller controller;

int main(int argc, char const *argv[])
{
    boost::log::sources::logger main_log;
    TK::log_init();

    BOOST_LOG(main_log) << "Hand tracker started." << std::endl;

    controller.setPolicy(Leap::Controller::POLICY_BACKGROUND_FRAMES);

    controller.addListener(listener);

    BOOST_LOG(main_log) << "Controller listener added." << std::endl;

    std::cin.get();

    controller.removeListener(listener);

    return 0;
}
