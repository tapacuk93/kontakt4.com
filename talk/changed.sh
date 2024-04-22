set -e

protoc --js_out=library=lang,binary:../web lang.proto
cp lang.proto ../cloud/