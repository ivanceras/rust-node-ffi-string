var ffi = require('ffi');

var library_name = './target/release/libstringtools.so';
var  api= ffi.Library(library_name, {
      'get_data': ['string', ['string', 'string']]
});

var call1 = api.get_data("/list", "products");

console.log("call1: ", call1);
console.log("call2: ", api.get_data("/get", "all_bananas"));
