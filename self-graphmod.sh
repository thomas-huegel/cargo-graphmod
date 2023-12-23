cargo run > modules.dot
tred modules.dot | dot -Tpdf > modules.pdf
tred modules.dot | dot -Tsvg > modules.svg

cargo run -- tests/web_app > tests/web_app/modules.dot
pushd tests/web_app
tred modules.dot | dot -Tpdf > modules.pdf
tred modules.dot | dot -Tsvg > modules.svg
popd