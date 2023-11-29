#ifndef MYCONTROLLER
#define MYCONTROLLER

#include "oatpp/web/server/api/ApiController.hpp"
#include "oatpp/core/macro/codegen.hpp"
#include "oatpp/core/macro/component.hpp"
#include "oatpp/web/protocol/http/Http.hpp"  


#include OATPP_CODEGEN_BEGIN(ApiController)

class Controller : public oatpp::web::server::api::ApiController {
public:
  Controller(const std::shared_ptr<ObjectMapper>& objectMapper)
    : oatpp::web::server::api::ApiController(objectMapper)
  {}

public:

  ENDPOINT("GET", "/get", get, QUERY(String, data)) {
    return createResponse(Status::CODE_200, "Hello from GET!");
  }

  ENDPOINT("POST", "/post", post, BODY_STRING(String, data)) {
    
    return createResponse(Status::CODE_200, "Hello form POST");
  }

};

#include OATPP_CODEGEN_END(ApiController)

#endif
