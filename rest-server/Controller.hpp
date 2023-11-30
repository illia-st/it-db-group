#ifndef MYCONTROLLER
#define MYCONTROLLER

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/macro/component.hpp"
#include "oatpp/web/protocol/http/Http.hpp"  

#include <iostream>
// Include ZeroMQ headers
#include <zmq.hpp>

void replacePercent2C(std::string &str) {
    size_t pos = 0;
    const std::string target = "%2C";
    const std::string replacement = ",";

    while ((pos = str.find(target, pos)) != std::string::npos) {
        str.replace(pos, target.length(), replacement);
        pos += replacement.length();
    }
}

#include OATPP_CODEGEN_BEGIN(ApiController)

class Controller : public oatpp::web::server::api::ApiController {
private:
  zmq::context_t m_zmqContext;
  zmq::socket_t m_zmqSocket;

public:
  Controller(const std::shared_ptr<ObjectMapper>& objectMapper)
    : oatpp::web::server::api::ApiController(objectMapper),
      m_zmqContext(1),
      m_zmqSocket(m_zmqContext, ZMQ_REQ)
  {
    m_zmqSocket.connect("tcp://127.0.0.1:4044");
    std::cout << "Zmq client connected to the server\n";
  }

public:

  ENDPOINT("GET", "/get", get, QUERY(oatpp::String, data)) {
    std::string requestData = data->c_str();
    replacePercent2C(requestData);
    std::cout << "Run get endpoint with data " << requestData << std::endl;
    m_zmqSocket.send(requestData.data(), requestData.size(), 0);

    zmq::message_t message;
    const auto ret = m_zmqSocket.recv(message);

    if (!ret.has_value()) {
        return createResponse(Status::CODE_500, "Error receiving message from server");
    }

    std::string received_message = std::string(static_cast<char*>(message.data()), message.size());

    return createResponse(Status::CODE_200, received_message.c_str());
  }

  ENDPOINT("POST", "/post", post, BODY_STRING(oatpp::String, data)) {
    std::string requestData = data->c_str();
    std::cout << "Run post endpoint with data " << requestData << std::endl;
    m_zmqSocket.send(requestData.data(), requestData.size(), 0);

    zmq::message_t message;
    const auto ret = m_zmqSocket.recv(message);

    if (!ret.has_value()) {
        return createResponse(Status::CODE_500, "Error receiving message from server");
    }

    std::string received_message = std::string(static_cast<char*>(message.data()), message.size());

    return createResponse(Status::CODE_200, received_message.c_str());
  }

  ENDPOINT("DELETE", "/delete", post, BODY_STRING(oatpp::String, data)) {
    std::string requestData = data->c_str();
    std::cout << "Run delete endpoint with data " << requestData << std::endl;
    m_zmqSocket.send(requestData.data(), requestData.size(), 0);

    zmq::message_t message;
    const auto ret = m_zmqSocket.recv(message);

    if (!ret.has_value()) {
        return createResponse(Status::CODE_500, "Error receiving message from server");
    }

    std::string received_message = std::string(static_cast<char*>(message.data()), message.size());

    return createResponse(Status::CODE_200, received_message.c_str());
  }

};

#include OATPP_CODEGEN_END(ApiController)

#endif
