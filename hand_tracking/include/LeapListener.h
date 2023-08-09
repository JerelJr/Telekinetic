#ifndef LEAPLISTENER_H
#define LEAPLISTENER_H

#include <boost/array.hpp>
#include <boost/asio.hpp>
#include <boost/integer.hpp>
#include <boost/log/core.hpp>
#include <boost/log/sinks.hpp>
#include <boost/log/sources/logger.hpp>
#include <boost/log/trivial.hpp>
#include <boost/log/utility/setup/file.hpp>
#include <boost/smart_ptr/make_shared.hpp>
#include <client.hpp>
#include <signal.h>
#include <string.h>

#include <Leap.h>

#define POS_PORT 1277
#define GEST_PORT 1278
#define LOOPBACK "127.0.0.1"

#define RETRACTED 0
#define EXTENDED 1
#define LEFT_PRESS 2
#define LEFT_RELEASE 3
#define RIGHT_PRESS 4
#define RIGHT_RELEASE 5

using namespace boost::asio::ip;

namespace TK
{
    void log_init();
}

class CustomListener : public Leap::Listener
{
public:
    CustomListener();
    ~CustomListener();
    virtual void onInit(const Leap::Controller &);
    virtual void onConnect(const Leap::Controller &);
    virtual void onDisconnect(const Leap::Controller &);
    virtual void onExit(const Leap::Controller &);
    virtual void onFrame(const Leap::Controller &);

private:
};

#endif
