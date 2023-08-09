#ifndef CLIENT_H
#define CLIENT_H

#include <boost/asio.hpp>
#include <boost/log/core.hpp>
#include <boost/log/sources/logger.hpp>
#include <boost/log/trivial.hpp>
#include <iostream>

using namespace boost::asio::ip;

tcp::socket create_socket(std::string address, port_type port, boost::asio::io_service &io_context);

#endif