doc-directory = "doc"

default:

build-doc:
    cd {{doc-directory}} && asciidoctor protocol.adoc

upload-doc: build-doc
    cd {{doc-directory}} && scp protocol.html nestedworld_dev@realtime-dev.nestedworld.com:www/protocol.html
