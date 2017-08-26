curl -X GET \
  http://127.0.0.1:4000/api/v1/echo \
  -d '{"greeting":"ayayay"}'
#POST
curl -X GET \
  http://127.0.0.1:4000/api/v1/greet/gusdelact \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' 
#POST
curl -X POST \
  http://127.0.0.1:4000/api/v1/greet/gusdelact \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -d '{"greeting":"ayayay"}'
#PUT
curl -X PUT \
  http://127.0.0.1:4000/api/v1/greet/gusdelact \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -d '{"greeting":"gusgus"}'
#DELETE
curl -X DELETE \
  http://127.0.0.1:4000/api/v1/greet/gusdelact \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -H 'postman-token: bc41f11e-554c-d17a-deea-8a59ea5d0ba5' \
  -d '{
	"greeting":"gusgugus"
}'
