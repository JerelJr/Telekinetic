#include <LeapListener.h>

using namespace Leap;

uint8_t prevThumbState = RETRACTED, prevMiddleState = RETRACTED;
boost::asio::io_service io_context;
boost::shared_ptr<tcp::socket> posSock, gestSock;
boost::log::sources::logger event_log;

void TK::log_init()
{
	boost::log::add_file_log(
		boost::log::keywords::file_name = "%Ntracker.log",
		boost::log::keywords::rotation_size = 10 * 1024 * 1024,
		boost::log::keywords::max_files = 1,
		boost::log::keywords::format = "[%TimeStamp%]: %Message%");
}

CustomListener::CustomListener()
{
}

CustomListener::~CustomListener() {}

void CustomListener::onInit(const Controller &controller)
{
	TK::log_init();

	BOOST_LOG(event_log) << "Leap device init" << std::endl;

	/**
	 * @brief Open and connect to socket to transmit hand data
	 *
	 */

	// create_socket
	posSock = boost::make_shared<tcp::socket>(create_socket(LOOPBACK, POS_PORT, io_context));
	BOOST_LOG(event_log) << "Position socket connected" << std::endl;

	gestSock = boost::make_shared<tcp::socket>(create_socket(LOOPBACK, GEST_PORT, io_context));
	BOOST_LOG(event_log) << "Gesture socket connected" << std::endl;
}

void CustomListener::onConnect(const Controller &controller)
{
	/*Configurations*/
	// controller.config().save();
}

void CustomListener::onDisconnect(const Controller &controller)
{
	/**
	 * @brief Close opened sockets
	 *
	 */
	BOOST_LOG(event_log) << "Leap device disconnected" << std::endl;

	posSock->close();
	gestSock->close();
}

void CustomListener::onExit(const Controller &controller) {}

void CustomListener::onFrame(const Controller &controller)
{
	boost::array<float, 2> coords;
	boost::uint8_t gesture = RETRACTED;

	/*Leap Declarations*/
	const Frame frame = controller.frame();
	InteractionBox box = frame.interactionBox();
	Leap::FingerList allFingers = frame.fingers();
	Leap::GestureList gestures = frame.gestures();

	/*Loop through fingers in frame*/
	for (FingerList::const_iterator fl = allFingers.begin();
		 fl != allFingers.end(); fl++)
	{
		switch ((*fl).type())
		{
		case Finger::TYPE_INDEX:
		{
			/**
			 * @brief Send index finger coordinates
			 *
			 */
			Vector fingerPos = (*fl).tipPosition();
			Vector boxFingerPos = box.normalizePoint(fingerPos);

			coords[0] = boxFingerPos.x;
			coords[1] = (1 - boxFingerPos.y);

			posSock->send(boost::asio::buffer(coords));

			// std::cout << "X:" << coords[0] << std::endl;
			// std::cout << "Y:" << coords[1] << std::endl;

			break;
		}
		case Finger::TYPE_MIDDLE:
		{
			/**
			 * @brief Detect change in middle finger gesture
			 *
			 */
			if ((*fl).isExtended() && prevMiddleState == RETRACTED)
			{
				gesture = RIGHT_RELEASE;
				gestSock->send(boost::asio::buffer(&gesture, sizeof(gesture)));
				prevMiddleState = EXTENDED;
			}
			if (!(*fl).isExtended() && prevMiddleState == EXTENDED)
			{
				gesture = RIGHT_PRESS;
				gestSock->send(boost::asio::buffer(&gesture, sizeof(gesture)));
				prevMiddleState = RETRACTED;
			}
			break;
		}
		case Finger::TYPE_THUMB:
		{
			/**
			 * @brief Detect change in thumb gesture
			 *
			 */
			if ((*fl).isExtended() && prevThumbState == RETRACTED)
			{
				gesture = LEFT_RELEASE;
				gestSock->send(boost::asio::buffer(&gesture, sizeof(gesture)));
				prevThumbState = EXTENDED;
			}
			if (!(*fl).isExtended() && prevThumbState == EXTENDED)
			{
				gesture = LEFT_PRESS;
				gestSock->send(boost::asio::buffer(&gesture, sizeof(gesture)));
				prevThumbState = RETRACTED;
			}
			break;
		}
		default:
			break;
		}
	}
}
