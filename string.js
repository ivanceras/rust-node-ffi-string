var ffi = require('ffi');

var library_name = './target/release/libstringtools.so';
var stringtools = ffi.Library(library_name, {
      'count_substrings': ['string', ['string', 'string']]
});

console.log(stringtools.count_substrings("bąnana", "na"));
