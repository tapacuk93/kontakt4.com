set -e

# protoc --js_out=library=talk,binary:../web lang.proto



protoc -I=./ lang.proto \
  --js_out=import_style=commonjs:../web \
  --grpc-web_out=import_style=commonjs,mode=grpcwebtext:../web