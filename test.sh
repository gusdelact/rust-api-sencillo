curl -X GET \
  http://127.0.0.1:4000/api/v1/frikis 
#POST
curl -X POST \
  http://127.0.0.1:4000/api/v1/friki/curlUser \
  -H 'cache-control: no-cache' \
  -H 'content-type: application/json' \
  -d '{"saludo":"esto es desde un prueba unitaria"}'
