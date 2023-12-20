cargo run > modules.dot
tred modules.dot | dot -Tpdf > modules.pdf
tred modules.dot | dot -Tpdf > modules.svg
