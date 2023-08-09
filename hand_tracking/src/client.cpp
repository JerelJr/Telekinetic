#include "client.hpp"

using namespace boost::asio::ip;

tcp::socket create_socket(std::string address, port_type port, boost::asio::io_service &io_context)
{
    try
    {
        tcp::endpoint endpoint(boost::asio::ip::address::from_string(address), port);
        tcp::socket socket(io_context);
        socket.connect(endpoint);

        return socket;
    }
    catch (const std::exception &e)
    {
        boost::log::sources::logger client_log;
        BOOST_LOG(client_log) << e.what() << '\n';
        exit(-1);
    }
}