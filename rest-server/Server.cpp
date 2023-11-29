#include "Server.hpp"
#include "Controller.hpp"
#include "oatpp/core/base/Environment.hpp"
#include "oatpp/network/Server.hpp"
#include "oatpp/network/tcp/server/ConnectionProvider.hpp"
#include "oatpp/web/mime/multipart/PartList.hpp"
#include "oatpp/parser/json/mapping/ObjectMapper.hpp"
#include "oatpp/web/server/HttpConnectionHandler.hpp"

Server::Server() {
    oatpp::base::Environment::init();
}

Server::~Server() {
    oatpp::base::Environment::destroy();
}

void Server::run() {
    auto objectMapper = oatpp::parser::json::mapping::ObjectMapper::createShared();

    // Create and configure ConnectionProvider
    auto connectionProvider = oatpp::network::tcp::server::ConnectionProvider::createShared({ "localhost", 8000, oatpp::network::Address::IP_4 });

    // Create HttpRouter and add your Controller
    auto router = oatpp::web::server::HttpRouter::createShared();
    auto controller = std::make_shared<Controller>(objectMapper);
    router->addController(controller);

    // Create HttpConnectionHandler with the configured router
    auto connectionHandler = oatpp::web::server::HttpConnectionHandler::createShared(router);

    // Create and configure the server
    oatpp::network::Server server(connectionProvider, connectionHandler);

    // Print information that the server is running
    OATPP_LOGI("Server", "Running on port %s...", connectionProvider->getProperty("port").toString()->c_str());

    // Run the server
    server.run();
}
